// Phase 17g — binary hardening flags for the `copythat-helper`
// elevated worker binary.
//
// Mirrors `crates/copythat-cli/build.rs` and
// `apps/copythat-ui/src-tauri/build.rs`. The helper runs with
// elevated privileges on Linux (via `sudo` / `polkit`); the same
// memory-corruption mitigations the main CLI gets are doubly
// important here, because a compromised helper holds root.
//
// Linux: full RELRO + BIND_NOW + noexecstack. Tells the dynamic
// linker to (1) resolve every PLT entry at load time (BIND_NOW) and
// (2) write-protect both the relocation table and the GOT after
// relocation (RELRO). The combination closes the GOT-overwrite class
// of memory-corruption exploit. NX is on by default since
// gcc 4.5 / binutils 2.20 but we declare it explicitly so a
// downgraded toolchain still produces a hardened binary.
//
// Windows: MSVC's default toolchain emits `/guard:cf` (Control Flow
// Guard) automatically; we rely on it.
//
// macOS arm64: PAC + BTI are enabled by default in the Apple linker
// on the current GitHub-hosted runners; no `cargo:rustc-link-arg`
// needed.
//
// All other targets fall through to the default linker flags.

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "linux" {
        // -Wl prefixes pass these straight through to the linker. The
        // ordering matters: relro before now, matching what e.g. the
        // Debian hardening defaults emit. The flags are no-ops on
        // statically-linked targets but harmless.
        println!("cargo:rustc-link-arg=-Wl,-z,relro");
        println!("cargo:rustc-link-arg=-Wl,-z,now");
        println!("cargo:rustc-link-arg=-Wl,-z,noexecstack");
    }
}
