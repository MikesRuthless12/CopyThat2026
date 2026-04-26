//! Windows Volume Shadow Copy Service backend.
//!
//! Two paths, picked at runtime:
//!
//! - **In-process.** When the parent is already elevated (Administrator
//!   token), we mint the shadow with the `IVssBackupComponents` COM
//!   port in [`super::vss_com`]. No child binary, no PowerShell
//!   shellout. With `--no-default-features` (i.e. the `vss-com`
//!   feature off) the in-process path falls back to PowerShell's
//!   `Win32_ShadowCopy::Create`.
//! - **Helper binary + UAC.** Otherwise we start the sibling
//!   `copythat-helper-vss.exe` via `Start-Process -Verb RunAs`, which
//!   triggers the UAC prompt. The helper connects to two named pipes
//!   we pre-created and speaks the JSON-RPC protocol in [`crate::rpc`].
//!
//! Both paths converge on the same two outputs: the shadow-copy ID
//! (a brace-wrapped GUID) and the device path
//! (`\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN`). Windows lets
//! you open paths of the form
//! `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN\path\to\file` for
//! read the same way `C:\path\to\file` works — no extra mount.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::windows::named_pipe::NamedPipeServer;
use tokio::process::Command;

use crate::SnapshotHandle;
use crate::error::SnapshotError;
use crate::kind::SnapshotKind;
use crate::rpc::{self, Request, Response};

use super::Cleanup as SuperCleanup;

#[derive(Debug)]
pub(crate) struct Cleanup {
    mode: Mode,
    shadow_id: String,
}

#[derive(Debug)]
enum Mode {
    InProcess,
    /// Keeps the helper pipes alive for as long as any snapshot it
    /// owns is live. On Drop / release, we send a `Release` then a
    /// `Shutdown` and close the writer — the helper EOFs and exits.
    Helper {
        conn: Box<HelperConn>,
    },
}

#[derive(Debug)]
struct HelperConn {
    writer: NamedPipeServer,
    reader: BufReader<NamedPipeServer>,
}

pub(crate) fn applies_to(path: &Path) -> bool {
    let s = path.to_string_lossy();
    if s.starts_with(r"\\") {
        return false;
    }
    volume_letter_for(path).is_some()
}

pub(crate) async fn create(src_path: &Path) -> Result<SnapshotHandle, SnapshotError> {
    let volume = volume_letter_for(src_path).ok_or_else(|| SnapshotError::BackendFailure {
        kind: SnapshotKind::Vss,
        message: format!("path has no drive letter: {}", src_path.display()),
    })?;

    if is_process_elevated() {
        let (shadow_id, device) = create_in_process(&volume).await?;
        return Ok(SnapshotHandle {
            kind: SnapshotKind::Vss,
            mount_path: PathBuf::from(&device),
            original_root: PathBuf::from(&volume),
            cleanup: Some(SuperCleanup::Vss(Cleanup {
                mode: Mode::InProcess,
                shadow_id,
            })),
        });
    }

    let mut conn = spawn_helper().await?;
    handshake(&mut conn).await?;
    let (shadow_id, device) = rpc_create(&mut conn, &volume).await?;
    Ok(SnapshotHandle {
        kind: SnapshotKind::Vss,
        mount_path: PathBuf::from(&device),
        original_root: PathBuf::from(&volume),
        cleanup: Some(SuperCleanup::Vss(Cleanup {
            mode: Mode::Helper {
                conn: Box::new(conn),
            },
            shadow_id,
        })),
    })
}

pub(crate) async fn release(c: Cleanup) -> Result<(), SnapshotError> {
    match c.mode {
        Mode::InProcess => release_in_process(&c.shadow_id).await,
        Mode::Helper { mut conn } => {
            let release_err = rpc_release(&mut conn, &c.shadow_id).await.err();
            let _ = rpc_shutdown(&mut conn).await;
            match release_err {
                Some(e) => Err(e),
                None => Ok(()),
            }
        }
    }
}

pub(crate) fn release_blocking(c: Cleanup) -> Result<(), SnapshotError> {
    match c.mode {
        Mode::InProcess => release_in_process_blocking(&c.shadow_id),
        Mode::Helper { conn } => {
            // Can't run async RPC from Drop. Best-effort: drop the
            // pipes to signal EOF → helper exits its loop. The
            // helper's own cleanup on exit releases any shadow we
            // didn't explicitly release.
            drop(conn);
            Ok(())
        }
    }
}

// --- In-process path ----------------------------------------------
//
// Two implementations, feature-gated on `vss-com`:
//
// * Default (`vss-com` on): direct `IVssBackupComponents` COM via
//   `super::vss_com`. No PowerShell, no WMI, no string interpolation.
// * Fallback (`--no-default-features`): the original PowerShell +
//   `Get-WmiObject Win32_ShadowCopy` shellout, kept for users who
//   need to drop the COM dep (e.g. ultra-minimal Windows builds).

#[cfg(feature = "vss-com")]
async fn create_in_process(volume: &str) -> Result<(String, String), SnapshotError> {
    // Defence-in-depth — `volume_letter_for` already restricts to the
    // `[A-Za-z]:\` shape upstream. `vss_com::create_shadow_via_com`
    // documents that it trusts the input, so we keep the same gate
    // active even on the COM path.
    if !is_valid_drive_root(volume) {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!(
                "refusing to shadow non-drive-root volume {volume:?}; expected `X:\\\\`"
            ),
        });
    }
    let volume = volume.to_string();
    tokio::task::spawn_blocking(move || {
        super::vss_com::create_shadow_via_com(&volume).map_err(|msg| {
            SnapshotError::BackendFailure {
                kind: SnapshotKind::Vss,
                message: msg,
            }
        })
    })
    .await
    .map_err(|e| SnapshotError::Protocol(format!("vss_com create spawn_blocking: {e}")))?
}

#[cfg(not(feature = "vss-com"))]
async fn create_in_process(volume: &str) -> Result<(String, String), SnapshotError> {
    if !is_valid_drive_root(volume) {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!(
                "refusing to shadow non-drive-root volume {volume:?}; expected `X:\\\\`"
            ),
        });
    }
    let script = format!(
        concat!(
            "$ErrorActionPreference='Stop';",
            "$v='{vol}';",
            "$r=(Get-WmiObject -List Win32_ShadowCopy).Create($v,'ClientAccessible');",
            "if($r.ReturnValue -ne 0){{throw \"Create returned $($r.ReturnValue)\"}};",
            "$s=Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq $r.ShadowID }};",
            "if(-not $s){{throw 'shadow not found after Create'}};",
            "Write-Output \"$($s.ID)`t$($s.DeviceObject)\""
        ),
        vol = powershell_escape(volume),
    );
    let out = Command::new(powershell_exe_path())
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err)?;
    if !out.status.success() {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!(
                "powershell VSS create failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        });
    }
    parse_id_device(&String::from_utf8_lossy(&out.stdout))
}

#[cfg(feature = "vss-com")]
async fn release_in_process(shadow_id: &str) -> Result<(), SnapshotError> {
    if !is_valid_guid_brace(shadow_id) {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!("refusing to release non-GUID shadow_id {shadow_id:?}"),
        });
    }
    let shadow_id = shadow_id.to_string();
    tokio::task::spawn_blocking(move || {
        super::vss_com::release_shadow_via_com(&shadow_id).map_err(|msg| {
            SnapshotError::BackendFailure {
                kind: SnapshotKind::Vss,
                message: msg,
            }
        })
    })
    .await
    .map_err(|e| SnapshotError::Protocol(format!("vss_com release spawn_blocking: {e}")))?
}

#[cfg(not(feature = "vss-com"))]
async fn release_in_process(shadow_id: &str) -> Result<(), SnapshotError> {
    // Mirror the helper-side validation: shadow_id must be the
    // braced-GUID shape WMI hands back from `Win32_ShadowCopy.ID`.
    // Refuses anything else so a bug in a future caller can't smuggle
    // a script-fragment-bearing string past `powershell_escape`
    // (which only handles `'`).
    if !is_valid_guid_brace(shadow_id) {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!("refusing to release non-GUID shadow_id {shadow_id:?}"),
        });
    }
    let script = format!(
        concat!(
            "$ErrorActionPreference='Stop';",
            "$id='{id}';",
            "$s=Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq $id }};",
            "if($s){{ $s.Delete() }}"
        ),
        id = powershell_escape(shadow_id),
    );
    let out = Command::new(powershell_exe_path())
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err)?;
    if out.status.success() {
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!(
                "powershell VSS delete failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

#[cfg(feature = "vss-com")]
fn release_in_process_blocking(shadow_id: &str) -> Result<(), SnapshotError> {
    if !is_valid_guid_brace(shadow_id) {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!("refusing to release non-GUID shadow_id {shadow_id:?}"),
        });
    }
    super::vss_com::release_shadow_via_com(shadow_id).map_err(|msg| SnapshotError::BackendFailure {
        kind: SnapshotKind::Vss,
        message: msg,
    })
}

#[cfg(not(feature = "vss-com"))]
fn release_in_process_blocking(shadow_id: &str) -> Result<(), SnapshotError> {
    // Same GUID-shape gate as the async `release_in_process`.
    if !is_valid_guid_brace(shadow_id) {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!("refusing to release non-GUID shadow_id {shadow_id:?}"),
        });
    }
    let script = format!(
        concat!(
            "$ErrorActionPreference='Stop';",
            "$id='{id}';",
            "$s=Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq $id }};",
            "if($s){{ $s.Delete() }}"
        ),
        id = powershell_escape(shadow_id),
    );
    let out = std::process::Command::new(powershell_exe_path())
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(map_spawn_err)?;
    if out.status.success() {
        Ok(())
    } else {
        Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: format!(
                "powershell VSS delete failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            ),
        })
    }
}

// --- Elevated helper path ------------------------------------------

async fn spawn_helper() -> Result<HelperConn, SnapshotError> {
    let helper_path = locate_helper_binary()?;
    let (in_pipe_name, out_pipe_name) = make_pipe_names();

    // Create both pipe server ends BEFORE launching the helper, so
    // its connect attempts never race ahead of us.
    //
    // Phase 17 follow-up — pipes use a custom DACL granting
    // pipe access only to the current user SID and
    // BUILTIN\Administrators (see `win_pipe_security`). The
    // default Win32 pipe DACL would otherwise grant
    // GENERIC_READ|GENERIC_WRITE to every process in the same
    // desktop session, letting a local non-admin attacker
    // enumerate `\\.\pipe\copythat-vss-*` and race the legitimate
    // helper to the connect.
    let reader_pipe = super::win_pipe_security::create_secure_named_pipe_server(&out_pipe_name)
        .map_err(|e| SnapshotError::Protocol(format!("create out-pipe: {e}")))?;
    let writer_pipe = super::win_pipe_security::create_secure_named_pipe_server(&in_pipe_name)
        .map_err(|e| SnapshotError::Protocol(format!("create in-pipe: {e}")))?;

    let helper_str = helper_path.to_string_lossy().to_string();
    let ps_cmd = format!(
        concat!(
            "try {{",
            "  Start-Process -Wait:$false -Verb RunAs ",
            "  -FilePath '{helper}' ",
            "  -ArgumentList @('--rpc-stdin={inp}','--rpc-stdout={outp}')",
            "}} catch {{ Write-Error $_.Exception.Message; exit 2 }}",
        ),
        helper = powershell_escape(&helper_str),
        inp = powershell_escape(&in_pipe_name),
        outp = powershell_escape(&out_pipe_name),
    );

    let launch = Command::new(powershell_exe_path())
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(map_spawn_err)?;
    if !launch.status.success() {
        let stderr = String::from_utf8_lossy(&launch.stderr).trim().to_string();
        let denied = stderr.to_ascii_lowercase().contains("canceled by the user")
            || stderr
                .to_ascii_lowercase()
                .contains("operation was canceled")
            || stderr
                .to_ascii_lowercase()
                .contains("operation was cancelled");
        return Err(if denied {
            SnapshotError::UacDenied
        } else {
            SnapshotError::BackendFailure {
                kind: SnapshotKind::Vss,
                message: format!(
                    "Start-Process RunAs failed (code {:?}): {}",
                    launch.status.code(),
                    stderr
                ),
            }
        });
    }

    // Wait for the helper to connect to both pipes. 15 s covers the
    // UAC consent dialog + the COM init the helper does on start.
    let accept_with_timeout = |pipe: NamedPipeServer, label: &'static str| async move {
        tokio::time::timeout(std::time::Duration::from_secs(15), pipe.connect())
            .await
            .map_err(|_| SnapshotError::Protocol(format!("{label} pipe connect timed out")))?
            .map_err(|e| SnapshotError::Protocol(format!("{label} pipe connect: {e}")))?;
        Ok::<_, SnapshotError>(pipe)
    };
    let reader_pipe = accept_with_timeout(reader_pipe, "stdout").await?;
    let writer_pipe = accept_with_timeout(writer_pipe, "stdin").await?;

    Ok(HelperConn {
        writer: writer_pipe,
        reader: BufReader::new(reader_pipe),
    })
}

async fn handshake(conn: &mut HelperConn) -> Result<(), SnapshotError> {
    let resp = rpc(
        conn,
        &Request::Hello {
            version: rpc::PROTOCOL_VERSION,
        },
    )
    .await?;
    if !resp.ok {
        return Err(SnapshotError::Protocol(
            resp.message.unwrap_or_else(|| "hello rejected".into()),
        ));
    }
    match resp.version {
        Some(v) if v == rpc::PROTOCOL_VERSION => Ok(()),
        Some(v) => Err(SnapshotError::Protocol(format!(
            "helper protocol version mismatch: got {v}, expected {}",
            rpc::PROTOCOL_VERSION
        ))),
        None => Err(SnapshotError::Protocol(
            "helper hello missing version".into(),
        )),
    }
}

async fn rpc_create(
    conn: &mut HelperConn,
    volume: &str,
) -> Result<(String, String), SnapshotError> {
    let resp = rpc(
        conn,
        &Request::Create {
            volume: volume.to_string(),
        },
    )
    .await?;
    if !resp.ok {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: resp
                .message
                .unwrap_or_else(|| "helper create returned ok=false".into()),
        });
    }
    let id = resp
        .shadow_id
        .ok_or_else(|| SnapshotError::Protocol("helper create missing shadow_id".into()))?;
    let dev = resp
        .device_path
        .ok_or_else(|| SnapshotError::Protocol("helper create missing device_path".into()))?;
    Ok((id, dev))
}

async fn rpc_release(conn: &mut HelperConn, shadow_id: &str) -> Result<(), SnapshotError> {
    let resp = rpc(
        conn,
        &Request::Release {
            shadow_id: shadow_id.to_string(),
        },
    )
    .await?;
    if !resp.ok {
        return Err(SnapshotError::BackendFailure {
            kind: SnapshotKind::Vss,
            message: resp
                .message
                .unwrap_or_else(|| "helper release returned ok=false".into()),
        });
    }
    Ok(())
}

async fn rpc_shutdown(conn: &mut HelperConn) -> Result<(), SnapshotError> {
    let _ = rpc(conn, &Request::Shutdown).await;
    Ok(())
}

async fn rpc(conn: &mut HelperConn, req: &Request) -> Result<Response, SnapshotError> {
    let line =
        serde_json::to_string(req).map_err(|e| SnapshotError::Protocol(format!("encode: {e}")))?;
    conn.writer
        .write_all(line.as_bytes())
        .await
        .map_err(SnapshotError::Io)?;
    conn.writer
        .write_all(b"\n")
        .await
        .map_err(SnapshotError::Io)?;
    conn.writer.flush().await.map_err(SnapshotError::Io)?;

    let mut reply = String::new();
    let n = conn
        .reader
        .read_line(&mut reply)
        .await
        .map_err(SnapshotError::Io)?;
    if n == 0 {
        return Err(SnapshotError::Protocol("helper stdout EOF".into()));
    }
    serde_json::from_str::<Response>(reply.trim())
        .map_err(|e| SnapshotError::Protocol(format!("decode: {e}: {reply:?}")))
}

// --- Helpers -------------------------------------------------------

/// Whether the current process token is an elevated Administrator.
pub(crate) fn is_process_elevated() -> bool {
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::Security::{
        GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false;
        }
        let mut elev: TOKEN_ELEVATION = std::mem::zeroed();
        let mut ret_len: u32 = 0;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            &mut elev as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut ret_len,
        );
        CloseHandle(token);
        ok != 0 && elev.TokenIsElevated != 0
    }
}

fn volume_letter_for(path: &Path) -> Option<String> {
    let s = path.to_string_lossy();
    let mut iter = s.chars();
    let letter = iter.next()?;
    let colon = iter.next()?;
    if !letter.is_ascii_alphabetic() || colon != ':' {
        return None;
    }
    Some(format!("{}:\\", letter.to_ascii_uppercase()))
}

fn locate_helper_binary() -> Result<PathBuf, SnapshotError> {
    let mut exe = std::env::current_exe().map_err(SnapshotError::Io)?;
    exe.pop();
    let helper = exe.join("copythat-helper-vss.exe");
    if helper.exists() {
        Ok(helper)
    } else {
        Err(SnapshotError::BackendMissing {
            tool: "copythat-helper-vss.exe",
        })
    }
}

fn make_pipe_names() -> (String, String) {
    // Use 256 bits of CSPRNG entropy (Uuid v4 carries ~122 bits of
    // randomness) so an attacker enumerating `\\.\pipe\copythat-vss-*`
    // can't predict the suffix. Falls back to a Uuid v4 if
    // getrandom fails; the helper's argv-time validation catches
    // shape drift either way.
    let mut bytes = [0u8; 32];
    let id = match getrandom::fill(&mut bytes) {
        Ok(()) => hex::encode(bytes),
        Err(_) => uuid::Uuid::new_v4().to_string().replace('-', ""),
    };
    (
        format!(r"\\.\pipe\copythat-vss-{id}-in"),
        format!(r"\\.\pipe\copythat-vss-{id}-out"),
    )
}

#[cfg(not(feature = "vss-com"))]
fn parse_id_device(stdout: &str) -> Result<(String, String), SnapshotError> {
    for line in stdout.lines() {
        let line = line.trim();
        if let Some((id, dev)) = line.split_once('\t') {
            return Ok((id.to_string(), dev.to_string()));
        }
    }
    Err(SnapshotError::BackendFailure {
        kind: SnapshotKind::Vss,
        message: format!("could not parse shadow ID + device from: {stdout:?}"),
    })
}

fn powershell_escape(s: &str) -> String {
    s.replace('\'', "''")
}

/// Reject anything not matching `[A-Za-z]:\` exactly. Matches
/// `helper_vss::is_valid_drive_root`; duplicated rather than
/// shared because the helper binary is intentionally
/// `unsafe_code = forbid` and copying the four-line check is
/// cheaper than threading a pub(crate) helper across the crate
/// boundary for the helper bin.
fn is_valid_drive_root(s: &str) -> bool {
    let bytes = s.as_bytes();
    bytes.len() == 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'\\'
}

/// Accept `{` + 32 hex digits with 4 hyphens + `}` (Win32
/// `Win32_ShadowCopy.ID` shape). Hyphen positions are 8/4/4/4/12.
/// Mirrors `helper_vss::is_valid_guid_brace`.
fn is_valid_guid_brace(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 38 {
        return false;
    }
    if bytes[0] != b'{' || bytes[37] != b'}' {
        return false;
    }
    for (i, &b) in bytes[1..37].iter().enumerate() {
        let want_hyphen = matches!(i, 8 | 13 | 18 | 23);
        if want_hyphen {
            if b != b'-' {
                return false;
            }
        } else if !b.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

/// Resolve the absolute path to PowerShell. Falls back to
/// `%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe`
/// when the env var is missing. Using the absolute path defends
/// against PATH-hijack: the snapshot calls run on the user's main
/// process before elevation, but the elevated `Start-Process -Verb
/// RunAs` ceremony inherits the unprivileged side's CWD, which on
/// the typical first-launch of a Tauri app is the user's Downloads
/// folder. Without an absolute path a `powershell.exe` planted in
/// CWD would be picked up first by the default lookup order.
fn powershell_exe_path() -> String {
    let sysroot = std::env::var("SystemRoot")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| r"C:\Windows".to_string());
    format!(r"{sysroot}\System32\WindowsPowerShell\v1.0\powershell.exe")
}

fn map_spawn_err(e: std::io::Error) -> SnapshotError {
    if e.kind() == std::io::ErrorKind::NotFound {
        SnapshotError::BackendMissing { tool: "powershell" }
    } else {
        SnapshotError::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_letter_for_extracts_uppercase_letter() {
        assert_eq!(
            volume_letter_for(Path::new(r"c:\users\alice\file.txt")).as_deref(),
            Some("C:\\")
        );
        assert_eq!(
            volume_letter_for(Path::new(r"D:\data")).as_deref(),
            Some("D:\\")
        );
        assert_eq!(volume_letter_for(Path::new(r"\\server\share\f")), None);
        assert_eq!(volume_letter_for(Path::new(r".\rel")), None);
    }

    #[cfg(not(feature = "vss-com"))]
    #[test]
    fn parse_id_device_splits_on_tab() {
        let (id, dev) =
            parse_id_device("{ABC-DEF}\t\\\\?\\GLOBALROOT\\Device\\HarddiskVolumeShadowCopy5\r\n")
                .unwrap();
        assert_eq!(id, "{ABC-DEF}");
        assert_eq!(dev, r"\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy5");
    }

    #[cfg(not(feature = "vss-com"))]
    #[test]
    fn parse_id_device_surfaces_empty_input_as_error() {
        assert!(parse_id_device("").is_err());
    }

    #[test]
    fn applies_to_rejects_unc_paths() {
        assert!(!applies_to(Path::new(r"\\server\share\f.txt")));
    }

    #[test]
    fn applies_to_accepts_drive_path() {
        assert!(applies_to(Path::new(r"C:\Users\alice\file.txt")));
    }
}
