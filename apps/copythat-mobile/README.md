# Copy That mobile companion (PWA)

Phase 37 — phone-side companion app. Connects to the desktop's
PeerJS peer-id over WebRTC and drives the full `/control` surface
(pause/resume/cancel jobs, resolve collisions, browse + rerun
history, secure delete, live globals stats).

Distributed as a **Progressive Web App**. Users open the deployed
URL in their phone browser, scan the desktop's pairing QR with
their camera, paste the resulting `cthat-pair://…` URL into the
PWA's pair screen, and tap "Add to Home Screen" — the manifest's
icons match the desktop tray icon so the home-screen launcher is
visually consistent. No App Store gate.

## Develop

```sh
cd apps/copythat-mobile
pnpm install
pnpm dev
```

The dev server is at http://localhost:5173. Open it on your phone
(same Wi-Fi network) to test against a real handset, or in your
desktop browser's mobile emulation mode to test the layout.

## Build

```sh
pnpm build
```

Output lands in `dist/`. Deploy to any static host (Cloudflare
Pages, Netlify, GitHub Pages, S3 + CloudFront, …). The vite-plugin-pwa
plugin generates `dist/manifest.webmanifest` + `dist/sw.js` so the
deployed URL is installable.

## Architecture

- `src/peer.ts` — PeerJS data-channel wrapper. `connect`, typed
  request/reply via `req_id`, streaming-event listener, `disconnect`
  for the Exit button.
- `src/protocol.ts` — TypeScript mirror of
  `crates/copythat-mobile/src/server.rs`'s `RemoteCommand` /
  `RemoteResponse` enums. Keep in lockstep.
- `src/App.svelte` — top-level state machine (unpaired ↔
  connecting ↔ connected ↔ unreachable).
- `src/views/Pairing.svelte` — first-run pair flow.
- `src/views/Dashboard.svelte` — live globals + job control +
  history. Subscribes to streaming `globals_tick` /
  `job_progress` / `job_completed` / `job_failed` events.
- `src/views/Unreachable.svelte` — desktop-offline state. The PWA
  refuses to do anything else without an active data channel.

## Threat model

- The PWA only controls the desktop while the desktop is online +
  has registered with the configured PeerJS broker. Going offline
  takes the entire control surface down (the PWA can't queue
  commands).
- Pairing happens explicitly: user scans QR + confirms 4-emoji
  SAS. No silent enrolment.
- Data channel runs over WebRTC's DTLS — confidentiality + integrity
  at the transport.
- Exit button cleanly disconnects so a closed tab can't accidentally
  leak a session.

## Production deployment

Until the desktop UI lets the user point at a self-hosted PeerJS
broker, the PWA targets `0.peerjs.com` (PeerJS's public broker).
For commercial deployment, run a self-hosted broker via
`docker run peerjs/peerjs-server` and override the broker URL on
both ends.

## What's intentionally not here

- Native iOS / Android Tauri Mobile binary — the PWA path is
  sufficient for v1; the App Store distribution is a Phase 37
  follow-up if the user community demands native features
  (background pushes via APNs/FCM, file-provider extension for
  uploading from the phone, etc.). The desktop's `notify.rs` push
  signers are wired today so the follow-up just plugs them into
  the manifest.
