//! Windows Volume Shadow Copy Service backend.
//!
//! Two paths, picked at runtime:
//!
//! - **In-process.** When the parent is already elevated (Administrator
//!   token), we shell to PowerShell's `Win32_ShadowCopy::Create` to
//!   mint the shadow. No child binary needed.
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
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};
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

// --- In-process PowerShell path ------------------------------------

async fn create_in_process(volume: &str) -> Result<(String, String), SnapshotError> {
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

async fn release_in_process(shadow_id: &str) -> Result<(), SnapshotError> {
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

fn release_in_process_blocking(shadow_id: &str) -> Result<(), SnapshotError> {
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
    // Phase 17 follow-up — the default Win32 pipe DACL grants
    // GENERIC_READ|GENERIC_WRITE to the Everyone SID, meaning a
    // local non-admin attacker on the same desktop session could
    // enumerate `\\.\pipe\copythat-vss-*` and connect as the
    // helper-side client first, racing the legitimate parent. We
    // can't pass a SECURITY_ATTRIBUTES through tokio's
    // `ServerOptions::create`, but `reject_remote_clients(true)`
    // closes the over-network case and `first_pipe_instance(true)`
    // detects an existing-name squat at construction. The full
    // DACL fix (current-user SID + BUILTIN\\Administrators only)
    // requires a raw `CreateNamedPipeW` call with SECURITY_ATTRIBUTES,
    // which lands in the IVssBackupComponents COM port; in the
    // meantime, the random Uuid v4 in the pipe name already keeps
    // the surface obscure — bumping it to 256-bit random would
    // close most of the gap.
    let reader_pipe = ServerOptions::new()
        .first_pipe_instance(true)
        .max_instances(1)
        .reject_remote_clients(true)
        .create(&out_pipe_name)
        .map_err(|e| SnapshotError::Protocol(format!("create out-pipe: {e}")))?;
    let writer_pipe = ServerOptions::new()
        .first_pipe_instance(true)
        .max_instances(1)
        .reject_remote_clients(true)
        .create(&in_pipe_name)
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

    #[test]
    fn parse_id_device_splits_on_tab() {
        let (id, dev) =
            parse_id_device("{ABC-DEF}\t\\\\?\\GLOBALROOT\\Device\\HarddiskVolumeShadowCopy5\r\n")
                .unwrap();
        assert_eq!(id, "{ABC-DEF}");
        assert_eq!(dev, r"\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy5");
    }

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
