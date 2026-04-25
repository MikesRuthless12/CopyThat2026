# Copy That Mobile — native Tauri target

Phase 37 follow-up #2. Wraps the Vite + Svelte 5 PWA at
`apps/copythat-mobile/` in a Tauri 2 native shell so users on
iOS / Android can install a binary instead of going through the
PWA's "Add to Home Screen" flow. The PeerJS pairing + control
protocol is identical — the native build just gives the user a
tappable home-screen icon backed by Tauri's webview.

## Why is this not a workspace member?

The crate is **deliberately excluded from the root `Cargo.toml`'s
`workspace.members` list** because building it requires a
matching mobile toolchain on the host (Xcode for iOS, Android
SDK + NDK for Android). Including it in the workspace would force
`cargo build` from a Windows-only desktop to fail at the
`tauri-build` step. It's standalone so the cargo build of the rest
of the workspace stays NASM-free + Xcode-free.

## Build instructions

### iOS (macOS host required)

```sh
# One-time setup
cargo install tauri-cli@^2
rustup target add aarch64-apple-ios x86_64-apple-ios

# From this directory
cd apps/copythat-mobile/src-tauri
cargo tauri ios init
cargo tauri ios build
```

Output: `apps/copythat-mobile/src-tauri/gen/apple/build/arm64/`
contains the `.app` bundle. Sign + upload via Xcode for App Store
distribution; or sideload via TestFlight.

### Android (any host with Android SDK + NDK + JDK 17)

```sh
# One-time setup
cargo install tauri-cli@^2
rustup target add aarch64-linux-android armv7-linux-androideabi \
  x86_64-linux-android i686-linux-android
export ANDROID_HOME=/path/to/android-sdk
export NDK_HOME=$ANDROID_HOME/ndk/<version>
export JAVA_HOME=/path/to/jdk17

# From this directory
cd apps/copythat-mobile/src-tauri
cargo tauri android init
cargo tauri android build
```

Output: `apps/copythat-mobile/src-tauri/gen/android/app/build/outputs/apk/`
contains the `.apk`. Sign with your release keystore for Play
Store distribution; or sideload via `adb install`.

## What gets shipped

- The same Svelte UI as the PWA at `apps/copythat-mobile/src/`.
- `tauri.conf.json` carries the `dev.copythat.mobile` identifier +
  the icon (`icons/icon.png`, copied from the desktop).
- A minimal `ping` IPC command for the JS ↔ Rust health check.

## What's NOT shipped today

- Native push integration (APNs / FCM) — the desktop already
  handles push dispatch via `copythat_mobile::notify`; the mobile
  binary just receives the OS's native push and deep-links into the
  PWA UI. Wiring is a follow-up.
- File-provider / scoped-storage extensions (mobile-as-source) —
  out of scope for the v1 companion (documented in Phase 41+).
- Native UI affordances beyond what the Svelte UI already offers.
  The PWA's UI is mobile-first by construction.
