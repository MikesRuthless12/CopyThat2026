fn main() {
    // Phase 17g — Linux RELRO + BIND_NOW + noexecstack for the
    // `copythat-ui` Tauri binary. Mirrors the build.rs in
    // `crates/copythat-cli/build.rs`. Other targets fall through
    // — Windows /guard:cf is automatic on MSVC, macOS arm64
    // PAC+BTI is automatic on the Apple linker.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        println!("cargo:rustc-link-arg=-Wl,-z,relro");
        println!("cargo:rustc-link-arg=-Wl,-z,now");
        println!("cargo:rustc-link-arg=-Wl,-z,noexecstack");
    }
    tauri_build::build()
}
