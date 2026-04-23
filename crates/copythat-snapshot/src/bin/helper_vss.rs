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
                Ok(r) => r,
                Err(e) => {
                    let resp = Response::err(format!("bad request: {e}"));
                    let _ = write_response(&mut writer, &resp).await;
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

    fn create_shadow(volume: &str) -> Result<(String, String), String> {
        // PowerShell's Win32_ShadowCopy::Create is the same path the
        // in-process elevated code uses. Having both paths share the
        // one syscall-shape keeps the "works on one platform, works
        // on both" invariant cheap.
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
        let out = std::process::Command::new("powershell")
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

    fn release_shadow(shadow_id: &str) -> Result<(), String> {
        let script = format!(
            concat!(
                "$ErrorActionPreference='Stop';",
                "$id='{id}';",
                "$s=Get-WmiObject Win32_ShadowCopy | Where-Object {{ $_.ID -eq $id }};",
                "if($s){{ $s.Delete() }}"
            ),
            id = ps_escape(shadow_id),
        );
        let out = std::process::Command::new("powershell")
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

    fn ps_escape(s: &str) -> String {
        s.replace('\'', "''")
    }
}
