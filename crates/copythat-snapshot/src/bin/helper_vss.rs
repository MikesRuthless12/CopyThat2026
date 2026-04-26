//! `copythat-helper-vss` — elevated Windows Volume Shadow Copy helper.
//!
//! The main `copythat-ui` process spawns this binary via
//! `Start-Process -Verb RunAs` when it needs a VSS snapshot but does
//! not itself hold an Administrator token. The helper runs at High
//! integrity level (via UAC consent), opens the two named pipes whose
//! names it received on the command line, and serves the JSON-RPC
//! protocol defined in `copythat_snapshot::rpc`.
//!
//! Wire:
//! - `--rpc-stdin=\\.\pipe\...-in` — the helper *reads* requests from
//!   this pipe. From the main process's point of view, it *writes* to
//!   this pipe (sends requests).
//! - `--rpc-stdout=\\.\pipe\...-out` — the helper *writes* responses
//!   here.
//!
//! On non-Windows platforms `main()` prints an explanatory error and
//! exits non-zero so nothing silently ships a no-op binary.

#[cfg(not(windows))]
fn main() {
    eprintln!("copythat-helper-vss is Windows-only");
    std::process::exit(2);
}

#[cfg(windows)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let rpc_stdin = pick_arg(&args, "--rpc-stdin=");
    let rpc_stdout = pick_arg(&args, "--rpc-stdout=");
    let (rpc_stdin, rpc_stdout) = match (rpc_stdin, rpc_stdout) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            eprintln!(
                "usage: copythat-helper-vss --rpc-stdin=<pipe> --rpc-stdout=<pipe>\n\
                 (invoked by the main copythat process; not a user-facing tool)"
            );
            std::process::exit(2);
        }
    };

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("copythat-helper-vss: tokio runtime init failed: {e}");
            std::process::exit(3);
        }
    };

    std::process::exit(rt.block_on(windows_main::run(rpc_stdin, rpc_stdout)));
}

#[cfg(windows)]
fn pick_arg(args: &[String], prefix: &str) -> Option<String> {
    args.iter()
        .find_map(|a| a.strip_prefix(prefix).map(|s| s.to_string()))
}

#[cfg(windows)]
mod windows_main {
    use copythat_snapshot::rpc::{self, Request, Response};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient};

    pub(super) async fn run(in_pipe: String, out_pipe: String) -> i32 {
        let (reader, writer) = match connect_pipes(&in_pipe, &out_pipe).await {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("copythat-helper-vss: pipe connect failed: {e}");
                return 4;
            }
        };
        let mut reader = BufReader::new(reader);
        let mut writer = writer;

        // Track shadow IDs we minted so we can best-effort-release on
        // EOF — the main process can die without sending `release` +
        // `shutdown` (e.g. `taskkill`), and leaking shadow copies is
        // ugly at scale.
        let mut owned: Vec<String> = Vec::new();

        // Bound consecutive malformed-JSON lines so a hijacked pipe
        // peer can't keep an elevated helper alive (and its owned
        // shadow copies mounted) indefinitely by streaming
        // garbage. Exit after MAX_BAD requests in a row; reset the
        // counter on every successful parse.
        const MAX_BAD: u32 = 5;
        let mut consecutive_bad: u32 = 0;
        loop {
            let mut line = String::new();
            let n = match reader.read_line(&mut line).await {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("copythat-helper-vss: read_line: {e}");
                    break;
                }
            };
            if n == 0 {
                // EOF. The main process closed the pipe without an
                // explicit shutdown — release everything we still own.
                break;
            }
            let req: Request = match serde_json::from_str(line.trim()) {
                Ok(r) => {
                    consecutive_bad = 0;
                    r
                }
                Err(e) => {
                    consecutive_bad = consecutive_bad.saturating_add(1);
                    let resp = Response::err(format!("bad request: {e}"));
                    let _ = write_response(&mut writer, &resp).await;
                    if consecutive_bad >= MAX_BAD {
                        eprintln!(
                            "copythat-helper-vss: {consecutive_bad} consecutive malformed requests, exiting"
                        );
                        break;
                    }
                    continue;
                }
            };
            let (resp, exit) = handle(req, &mut owned).await;
            if write_response(&mut writer, &resp).await.is_err() {
                break;
            }
            if exit {
                break;
            }
        }

        // Best-effort cleanup of anything still owned.
        for id in owned.drain(..) {
            let _ = release_shadow(&id);
        }
        0
    }

    async fn connect_pipes(
        in_pipe: &str,
        out_pipe: &str,
    ) -> std::io::Result<(NamedPipeClient, NamedPipeClient)> {
        let reader = connect_one(in_pipe).await?;
        let writer = connect_one(out_pipe).await?;
        Ok((reader, writer))
    }

    async fn connect_one(name: &str) -> std::io::Result<NamedPipeClient> {
        // A tiny retry loop handles the window between the server
        // calling `CreateNamedPipeW` and the helper connecting. In
        // practice the parent creates the server ends before
        // Start-Process, but a stray `ERROR_PIPE_BUSY` still happens.
        for _ in 0..50 {
            match ClientOptions::new().open(name) {
                Ok(c) => return Ok(c),
                Err(e) if e.raw_os_error() == Some(231) /* PIPE_BUSY */ => {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                Err(e) => return Err(e),
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "named pipe still busy after 5 s",
        ))
    }

    async fn write_response(writer: &mut NamedPipeClient, resp: &Response) -> std::io::Result<()> {
        let line = serde_json::to_string(resp)
            .map_err(|e| std::io::Error::other(format!("encode: {e}")))?;
        writer.write_all(line.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }

    async fn handle(req: Request, owned: &mut Vec<String>) -> (Response, bool) {
        match req {
            Request::Hello { version } => {
                if version == rpc::PROTOCOL_VERSION {
                    let mut r = Response::ok();
                    r.version = Some(rpc::PROTOCOL_VERSION);
                    (r, false)
                } else {
                    (
                        Response::err(format!(
                            "protocol mismatch: got {version}, want {}",
                            rpc::PROTOCOL_VERSION
                        )),
                        false,
                    )
                }
            }
            Request::Create { volume } => match create_shadow(&volume) {
                Ok((id, dev)) => {
                    owned.push(id.clone());
                    let mut r = Response::ok();
                    r.shadow_id = Some(id);
                    r.device_path = Some(dev);
                    r.volume = Some(volume);
                    (r, false)
                }
                Err(msg) => (Response::err(msg), false),
            },
            Request::Release { shadow_id } => match release_shadow(&shadow_id) {
                Ok(()) => {
                    owned.retain(|id| id != &shadow_id);
                    (Response::ok(), false)
                }
                Err(msg) => (Response::err(msg), false),
            },
            Request::Shutdown => (Response::ok(), true),
        }
    }

    // VSS create / release primitives. With `vss-com` (default), the
    // helper goes straight to `IVssBackupComponents` via the lib's
    // re-exported COM port — same primitive the in-process snapshot
    // path uses. With `--no-default-features` we fall back to the
    // PowerShell + `Get-WmiObject Win32_ShadowCopy` shellout. Either
    // way the validation gates are unconditional: no path through
    // here lets a crafted volume string or shadow_id reach the
    // privileged backend without the shape check fronting it.

    #[cfg(feature = "vss-com")]
    fn create_shadow(volume: &str) -> Result<(String, String), String> {
        if !is_valid_drive_root(volume) {
            return Err(format!(
                "refusing to shadow non-drive-root volume {volume:?}; expected `X:\\\\`"
            ));
        }
        copythat_snapshot::vss_com::create_shadow_via_com(volume)
    }

    #[cfg(not(feature = "vss-com"))]
    fn create_shadow(volume: &str) -> Result<(String, String), String> {
        // Validate the volume identifier before interpolation. The
        // pipe peer is the unprivileged caller (or, in a future
        // squat-attack scenario, an attacker — see the snapshot
        // helper review). Allow only the canonical Windows
        // drive-root shape `[A-Z]:\`; reject anything else with a
        // typed error so the elevated WMI call never sees a
        // crafted argument.
        if !is_valid_drive_root(volume) {
            return Err(format!(
                "refusing to shadow non-drive-root volume {volume:?}; expected `X:\\\\`"
            ));
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
            vol = ps_escape(volume),
        );
        // Absolute path defends against PATH hijack: the helper
        // inherits CWD from the elevation spawn (typically the
        // Tauri app's CWD, which on first launch can be the user's
        // Downloads folder). Windows' default lookup includes CWD
        // before %SystemRoot%\System32, so a `powershell.exe`
        // dropped into Downloads would otherwise execute at High
        // integrity.
        let out = std::process::Command::new(powershell_exe())
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .map_err(|e| format!("spawn powershell: {e}"))?;
        if !out.status.success() {
            return Err(format!(
                "powershell VSS create failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        }
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let line = line.trim();
            if let Some((id, dev)) = line.split_once('\t') {
                return Ok((id.to_string(), dev.to_string()));
            }
        }
        Err(format!("could not parse shadow ID + device from: {text:?}"))
    }

    #[cfg(feature = "vss-com")]
    fn release_shadow(shadow_id: &str) -> Result<(), String> {
        if !is_valid_guid_brace(shadow_id) {
            return Err(format!(
                "refusing to release non-GUID shadow_id {shadow_id:?}"
            ));
        }
        copythat_snapshot::vss_com::release_shadow_via_com(shadow_id)
    }

    #[cfg(not(feature = "vss-com"))]
    fn release_shadow(shadow_id: &str) -> Result<(), String> {
        // Reject anything that isn't the WMI GUID shape
        // `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` (case
        // insensitive). Defence-in-depth: even if a crafted pipe
        // peer slipped a script-fragment-bearing string past the
        // ps_escape doubling (which only handles `'`, not
        // newlines), the GUID shape leaves no room for one.
        if !is_valid_guid_brace(shadow_id) {
            return Err(format!(
                "refusing to release non-GUID shadow_id {shadow_id:?}"
            ));
        }
        let script = format!(
            concat!(
                "$ErrorActionPreference='Stop';",
                "$id='{id}';",
                "$s=Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq $id }};",
                "if($s){{ $s.Delete() }}"
            ),
            id = ps_escape(shadow_id),
        );
        let out = std::process::Command::new(powershell_exe())
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .map_err(|e| format!("spawn powershell: {e}"))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(format!(
                "powershell VSS delete failed (code {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            ))
        }
    }

    /// Resolve the absolute path to PowerShell. Falls back to
    /// `%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe`
    /// when the env var is missing.
    #[cfg(not(feature = "vss-com"))]
    fn powershell_exe() -> String {
        let sysroot = std::env::var("SystemRoot")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| r"C:\Windows".to_string());
        format!(r"{sysroot}\System32\WindowsPowerShell\v1.0\powershell.exe")
    }

    /// Reject anything not matching `[A-Za-z]:\` exactly.
    fn is_valid_drive_root(s: &str) -> bool {
        let bytes = s.as_bytes();
        bytes.len() == 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'\\'
    }

    /// Accept `{` + 32 hex digits with 4 hyphens + `}` (Win32
    /// `Win32_ShadowCopy.ID` shape). Hyphen positions are 8/4/4/4/12.
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

    #[cfg(not(feature = "vss-com"))]
    fn ps_escape(s: &str) -> String {
        s.replace('\'', "''")
    }
}
