# Phase 42 — PeerJS Replacement Evaluation & Migration Plan

**Status:** Research complete; **no code change applied**.
**Recommendation:** **Stay on `peerjs@^1.5.5`** (with mitigations) — see "Decision" below.
**Backup option (if a future audit re-flags):** **`trystero`** with a thin shim, or **`simple-peer` + roll-our-own-broker**.

---

## 0. Why this document exists

The Phase 42 ten-agent security review flagged the frontend's `peerjs@^1.5.4`
dependency as "supply-chain stale" — the agent's stated grounds were:

> PeerJS — Dec 2021, GitHub repo archived, no recent maintenance.

**On verification (April 26, 2026), every part of that claim is false.** This
document records the verification, evaluates four candidate replacements
against our actual constraints, and documents what a migration would look
like *if* we choose to do one.

---

## 1. Where the dependency lives

The `peerjs` npm package is a dependency of the **mobile companion PWA**, not
the desktop UI:

| File                                                  | Role                                                                                          |
| ----------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `apps/copythat-mobile/package.json`                   | **Declares `"peerjs": "^1.5.4"`** as a runtime dep. (Not `apps/copythat-ui/package.json`.)    |
| `apps/copythat-mobile/src/peer.ts`                    | Sole import site. Wraps PeerJS in a `PeerLink` class — the only abstraction layer.            |
| `apps/copythat-mobile/src/App.svelte`                 | Instantiates `new PeerLink()`.                                                                |
| `apps/copythat-mobile/src/views/Dashboard.svelte`     | Consumes the `PeerLink` instance via prop.                                                    |
| `apps/copythat-mobile/src-tauri/tauri.conf.json`      | CSP `connect-src wss://*.peerjs.com https://*.peerjs.com` for the broker.                     |
| `apps/copythat-ui/src/lib/components/MobilePanel.svelte` | Desktop-side settings UI: lets the user override broker host (`peerjsBroker` text field). |
| `apps/copythat-ui/src/lib/types.ts`                   | `peerjsBroker: string` field on `MobileSettings` DTO.                                         |
| `apps/copythat-ui/src-tauri/src/ipc.rs`               | `peerjs_broker` field on persisted `MobileSettings`.                                          |
| `crates/copythat-mobile/src/settings.rs`              | `peerjs_broker: "0.peerjs.com"` default.                                                      |
| `crates/copythat-mobile/src/settings_bridge.rs`       | Same default.                                                                                 |

**Important correction to the Phase 42 brief:** the brief tells us to update
`apps/copythat-ui/package.json`, but `peerjs` is **not** a dependency of
`copythat-ui`. The desktop only references PeerJS by *name* in code comments
and in a settings field (`peerjsBroker`) that the user can override. The
desktop hosts no PeerJS client; it's a Rust + Tauri app with no npm `peerjs`
import. The actual JS dependency to swap is in `apps/copythat-mobile/package.json`.

---

## 2. Candidate evaluation matrix

Verified against the live npm registry and GitHub API on **2026-04-26**.

| Library                  | Latest version           | Last commit       | Stars  | Weekly DLs (Apr 19-25, 2026) | License | Repo archived? | Built-in signaling broker? | API model                                | Drop-in vs PeerJS?                                            |
| ------------------------ | ------------------------ | ----------------- | ------ | ----------------------------:| ------- | -------------- | -------------------------- | ---------------------------------------- | ------------------------------------------------------------- |
| **peerjs** (current)     | `1.5.5` (Jun 7, 2025)    | **2025-07-18**    | 13,307 |                       61,658 | **MIT** | **NO**         | **YES** (`0.peerjs.com`)   | Per-peer DataConnection, peer-id keyed   | n/a                                                           |
| **simple-peer**          | `9.11.0` (2022)          | **2022-02-17**    | 7,791  |                      230,253 | **MIT** | NO (but stale) | NO — BYO signaling         | Manual SDP offer/answer exchange         | NO — would need to build broker + peer-id layer ourselves     |
| **trystero**             | `0.23.1` (Apr 21, 2026)  | **2026-04-26**    |  2,524 |                        2,783 | **MIT** | NO             | YES — multi-strategy       | Room-keyed, `makeAction()` typed channels| NO — different mental model (rooms, not peer-ids)             |
| **peer-lite**            | `1.x` (2022)             | **2022-08-06**    |    165 |                           56 | MIT     | NO (but stale) | NO                         | Thin RTCPeerConnection wrapper           | NO — even less abstracted than simple-peer                    |
| **@livekit/client**      | various                  | active            |  high  |                         high | Apache  | NO             | YES (LiveKit SaaS / OSS)   | Room/track centric, video-conferencing   | NO — wrong shape, heavyweight, requires LiveKit SFU           |
| **Native WebRTC + shim** | n/a                      | n/a               |   n/a  |                          n/a | n/a     | n/a            | NO — BYO                   | RTCPeerConnection + RTCDataChannel       | NO — full custom build                                        |

### Per-candidate verification notes

#### peerjs (current pin)
- npm `1.5.5` — published **Jun 7, 2025**.
- GitHub: `peers/peerjs`. `archived: false`, `disabled: false`.
- Most recent commits: **2025-07-18** (last commit cluster — `chore(deps)`
  Renovate updates). `pushed_at` on the GitHub repo refresh is
  **2026-02-27** (the project still has CI activity).
- 200 open issues — typical for a 13k-star repo. None marked critical.
- License: **MIT** (verified in repo `LICENSE` and `package.json`).
- No published GitHub Security Advisories against `peerjs` for any version.
- Verdict: **Actively maintained**, license-clean, used in production by
  many shipping apps, semantic-release pipeline still active. The Phase 42
  reviewer's "archived" claim is empirically wrong.

#### simple-peer (feross)
- GitHub: `feross/simple-peer`. `archived: false`.
- Most recent commits: **2022-02-17** — over 4 years old. `pushed_at` is
  **2024-06-26** (a tag/release push without code commits).
- Stars 7,791. License **MIT** (verified).
- ~230k weekly downloads (heavily used by `webtorrent` + the WebTorrent
  ecosystem, which is feross's primary domain).
- API: `new Peer({ initiator: true })` then exchange SDP via `peer.signal()`
  / `peer.on("signal", ...)`. **No built-in broker** — caller must wire up
  any signaling channel (WebSocket, fetch, copy-paste, etc.).
- Verdict: stable, but *itself* now stale by our maintenance criterion
  (no commits in 4 years). Replacing peerjs with simple-peer would be
  trading one stale lib for another, plus we'd own the broker stack.

#### trystero (dmotz)
- GitHub: `dmotz/trystero`. `archived: false`.
- Most recent commits: **2026-04-26** (today). Releases `0.23.1` on
  Apr 21, 2026, `0.23.0` on Mar 23, 2026, `0.22.0` on Oct 11, 2025.
- License: **MIT** (verified).
- Stars 2,524, weekly DLs ~2,800. Smaller user base than peerjs but
  actively growing (now a monorepo: `@trystero-p2p/torrent`,
  `@trystero-p2p/firebase`, `@trystero-p2p/mqtt`, etc.).
- API: room-based + typed-action channels. Each app picks a "strategy"
  for matchmaking — BitTorrent trackers, Nostr relays, MQTT, IPFS,
  Supabase, Firebase. Data is end-to-end encrypted client-side; the
  strategy medium only sees the WebRTC handshake.
- Verdict: **best-maintained candidate**, but the API is fundamentally
  different from PeerJS. Needs a shim. See section 4.

#### peer-lite
- GitHub: `skyllo/peer-lite`. Last commit **2022-08-06**. 165 stars.
- 56 weekly downloads. Not a viable production substitute.

#### @livekit/components-react / livekit-client
- Apache-2.0. Highly active.
- But: design center is video conferencing with an SFU. Requires running
  a LiveKit server (or paying for LiveKit Cloud). Far too heavy for
  a one-off phone-to-desktop control channel.

#### PeerJS forks
- A scan of `peerjs` forks shows 1,300+ forks, but none with measurably
  more activity than upstream `peers/peerjs`. The upstream project is
  itself active, so a fork would be regressive.

---

## 3. Decision

### Primary recommendation: **stay on `peerjs@^1.5.5`**

Rationale:

1. **The flag is empirically wrong.** Upstream is not archived, has been
   committed-to in 2025 with a release in June 2025, and CI/Renovate
   activity continues into 2026.
2. **License is MIT** — clean for commercial use.
3. **No outstanding GitHub Security Advisories** for the `peerjs` npm
   package at any version.
4. **The API surface fits us perfectly** — peer-id-based addressing is
   what we want for QR-pairing flow (the desktop mints a stable peer-id,
   the phone scans QR with that peer-id, and the broker brokers a single
   handshake). PeerJS is *built* for this. Trystero's room model would
   require us to map peer-id → room-id → action which is more glue.
5. **Switching would burn UI testing budget.** Per CLAUDE.md we must
   validate after every change; swapping the WebRTC layer touches the
   pairing handshake, the SAS visual-fingerprint flow, the data channel
   re-connect logic, and the protocol envelope — each of which is in
   the manual QA checklist.

### Mitigations to apply (no library swap):

1. **Pin to an exact version** (`"peerjs": "1.5.5"`, not `"^1.5.4"`)
   to remove silent floor-rolling. (Out of scope for this phase — frontend
   change requires UI testing.)
2. **Self-host the broker.** The default `0.peerjs.com` is run by the
   PeerJS project as a free public service with no SLA. We already
   expose `peerjsBroker` as a user-overridable setting — the production
   ship plan should default it to a CopyThat-operated broker (one
   `peerjs-server` Docker container per region). This eliminates
   third-party-dependency risk regardless of which client lib is in
   use, and is the higher-leverage fix.
3. **Document the upgrade-watch.** Add `peerjs` to the dependency-watch
   in `docs/SECURITY.md` so the next quarterly review sees the actual
   release cadence rather than re-flagging it.

### Backup recommendation (if a future audit re-flags or upstream truly stalls): **trystero** behind a `PeerLink`-shaped shim.

Why trystero over simple-peer:
- **Active maintenance** (commits today).
- **Built-in matchmaking** without a CopyThat-operated server — Nostr
  relays or BitTorrent trackers handle the SDP exchange. (For the use
  case of "two devices on the same LAN both online", any strategy works;
  for "phone is on cellular and desktop is on home wifi" the trackers
  punch through.)
- **Smaller bundle size** than livekit, focused scope.
- **MIT licensed.**

Trystero requires a shim because the APIs are not isomorphic; see section 4.

---

## 4. Migration mapping (only if we decide to replace)

Our wrapper class is small (~165 lines, `apps/copythat-mobile/src/peer.ts`).
The only consumers are `App.svelte` and `views/Dashboard.svelte`, which
import the `PeerLink` class and the `PeerStatus` type. **All migrations
preserve the `PeerLink` public surface** so neither Svelte file changes.

### 4.1 PeerJS → trystero API mapping

| PeerJS primitive                                         | Trystero equivalent                                                                                       |
| -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `import Peer from "peerjs"`                              | `import { joinRoom } from "trystero"` (or `trystero/torrent`, `trystero/nostr`, etc.)                     |
| `import type { DataConnection } from "peerjs"`           | No public type; `room` object is the unit of addressing.                                                  |
| `new Peer()` (auto-mints peer-id, talks to public broker)| `joinRoom({ appId: "copythat" }, /*roomId=*/ desktopPeerId)`                                              |
| `new Peer(undefined, { host, secure: true, path: "/" })` | `joinRoom({ appId: "copythat", relayUrls: [...] }, roomId)` — but trystero owns the relay layer.          |
| `peer.on("open", () => ...)`                             | After `joinRoom(...)` returns, room is ready; `room.onPeerJoin(peer => ...)` for new peers.               |
| `peer.connect(desktopPeerId, { serialization: "json" })` | n/a — peers in the same room auto-connect. The "desktop peer-id" becomes the `roomId`.                    |
| `conn.on("open", ...)`                                   | `room.onPeerJoin(peer => ...)` — fires when desktop joins.                                                |
| `conn.send(payload)`                                     | `const [send, recv] = room.makeAction("cmd"); send(payload)` — typed per action name.                     |
| `conn.on("data", cb)`                                    | `recv((data, peerId) => cb(data))` — receiver from the same `makeAction`.                                 |
| `conn.on("close", ...)`                                  | `room.onPeerLeave(peer => ...)`                                                                           |
| `conn.on("error", ...)`                                  | `room.onPeerJoin` / per-action error handlers (errors are rarer because trystero auto-retries).           |
| `peer.on("disconnected", ...)`                           | `room.onPeerLeave(...)`                                                                                   |
| `peer.destroy()`                                         | `room.leave()`                                                                                            |

### 4.2 PeerJS → simple-peer API mapping

| PeerJS primitive                          | simple-peer equivalent                                                                                |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `import Peer from "peerjs"`               | `import Peer from "simple-peer"`                                                                      |
| `new Peer()` (auto peer-id)               | n/a — simple-peer has no addressing layer. **You must build a broker.**                               |
| Broker dial                               | Caller supplies own WebSocket signaling. We'd run `peerjs-server` or write our own.                   |
| `peer.connect(id)`                        | `new Peer({ initiator: true })` + manual SDP exchange via the broker.                                 |
| `peer.on("open", ...)`                    | `peer.on("connect", ...)`                                                                             |
| `conn.send(...)`                          | `peer.send(data)` (must be string or Uint8Array).                                                     |
| `conn.on("data", ...)`                    | `peer.on("data", ...)`                                                                                |
| `conn.close()`                            | `peer.destroy()`                                                                                      |

### 4.3 Concrete `PeerLink` shim (trystero-backed) — illustrative only

```ts
// File: apps/copythat-mobile/src/peer.ts (PROPOSED, not applied)
import { joinRoom, type Room } from "trystero/nostr"; // or /torrent

import type { RemoteCommand, RemoteResponse } from "./protocol";

export type PeerStatus =
  | { kind: "idle" }
  | { kind: "connecting"; desktopPeerId: string }
  | { kind: "connected"; desktopPeerId: string }
  | { kind: "error"; message: string }
  | { kind: "disconnected" };

export class PeerLink {
  private room: Room | null = null;
  private remotePeerId: string | null = null;
  private status: PeerStatus = { kind: "idle" };
  private statusListeners: Array<(s: PeerStatus) => void> = [];
  private eventListeners: Array<(e: RemoteResponse) => void> = [];
  private pending: Map<number, { resolve: (r: RemoteResponse) => void; reject: (e: unknown) => void }> = new Map();
  private nextReqId = 1;
  private send: ((data: unknown, target?: string) => void) | null = null;

  connect(desktopPeerId: string, _brokerHost?: string): void {
    this.disconnect();
    this.setStatus({ kind: "connecting", desktopPeerId });

    // Peer-id becomes the room-id.
    const room = joinRoom({ appId: "copythat-v1" }, desktopPeerId);
    this.room = room;

    const [sendCmd, recvCmd] = room.makeAction<RemoteCommand & { req_id: number }>("cmd");
    this.send = sendCmd;
    recvCmd((data) => this.handleIncoming(data));

    room.onPeerJoin((peerId) => {
      this.remotePeerId = peerId;
      this.setStatus({ kind: "connected", desktopPeerId });
    });
    room.onPeerLeave(() => {
      this.setStatus({ kind: "disconnected" });
    });
  }

  // ... rest of class (send, onEvent, onStatus, disconnect, handleIncoming)
  //     ports 1:1 from the current peerjs-backed implementation.
}
```

**Caveat:** trystero `makeAction` payloads are JSON-serialized internally,
so the `serialization: "json"` flag becomes implicit. Our existing
protocol envelope (`req_id` + body) survives unchanged.

---

## 5. Migration steps (only if we go through with it)

1. **Pre-flight:**
   - Confirm Tauri WebView2 (Win) / WKWebView (macOS) / WebKitGTK (Linux)
     all support the WebRTC features trystero needs. (Trystero uses
     `RTCPeerConnection` + `RTCDataChannel` — same browser primitives
     PeerJS uses.)
   - Confirm CSP allows the chosen strategy's signaling endpoints. For
     `trystero/nostr` we add `wss://relay.damus.io` and similar relays
     to `connect-src` in `apps/copythat-mobile/src-tauri/tauri.conf.json`.
2. **Code changes (all in `apps/copythat-mobile/`):**
   - `package.json`: replace `"peerjs": "^1.5.4"` with `"trystero": "^0.23.1"`.
   - `src/peer.ts`: rewrite `PeerLink` body per shim above; keep public
     API stable.
   - `src-tauri/tauri.conf.json`: update CSP `connect-src` to the new
     relay set; remove `*.peerjs.com`.
3. **Settings deprecation:**
   - Keep `peerjs_broker` in `crates/copythat-mobile/src/settings.rs`
     for one release as a deprecated field (`#[serde(default, alias = "peerjsBroker")]`),
     ignored at runtime.
   - In a follow-up release rename to `signaling_relays: Vec<String>`.
4. **UI changes (deferred — frontend per CLAUDE.md):**
   - `apps/copythat-ui/src/lib/components/MobilePanel.svelte`: change
     "PeerJS broker URL" label and placeholder.
   - `apps/copythat-ui/src/lib/types.ts`: rename `peerjsBroker` to
     `signalingRelays` (and update bindings).
5. **Run full QA:**
   - `pnpm --filter copythat-mobile build` — must pass.
   - Pair-and-control end-to-end: `tests/smoke/phase_37_mobile.rs`.
   - Manual QA matrix covering pair, SAS confirm, command dispatch,
     reconnect, secure-delete, history rerun.

---

## 6. Risk callouts

| Risk                                                                  | Severity | Mitigation                                                                              |
| --------------------------------------------------------------------- | -------- | --------------------------------------------------------------------------------------- |
| trystero's signaling strategies all require some **third-party**      | **HIGH** | We'd still need to self-host a relay (e.g., a small Nostr relay) to remove the          |
| relay/tracker. This is the same supply-chain shape as `0.peerjs.com`. |          | dependency, just like we'd self-host `peerjs-server` today.                             |
| Different peer-id semantics — trystero peer IDs are derived per       | MED      | Keep our existing X25519 SAS handshake; the trystero peerId only identifies the         |
| session, not stable like PeerJS peer-ids.                             |          | transport, not the device. Use the room-id for stable addressing.                       |
| Bundle-size delta — trystero+strategy is ~30-60 KB gzipped vs         | LOW      | Acceptable for a PWA install footprint.                                                 |
| peerjs ~25 KB.                                                        |          |                                                                                         |
| End-to-end testing churn — Playwright specs in                        | MED      | Update fixtures that reference `peerjsBroker`. Re-record the network-request golden     |
| `apps/copythat-ui/e2e/qa-section-4-11*-mobile*.spec.ts`.              |          | files for `qa-section-4-11`.                                                            |
| Tauri 2 webview compatibility — WebKitGTK on older Linux distros      | MED      | Verify on the lowest target distro (Ubuntu 22.04). PeerJS works there today; trystero   |
| can lag on `RTCDataChannel` features.                                 |          | uses the same primitives so should also work, but smoke-test before ship.               |
| Network-restricted environments — corporate proxies that block        | LOW      | Same caveat applies to PeerJS today; user can configure custom signaling URL.           |
| outbound to public relays.                                            |          |                                                                                         |

---

## 7. Estimated effort

| Path                                     | Engineering | QA   | Total       |
| ---------------------------------------- | ----------- | ---- | ----------- |
| **Stay (current rec):**                  |             |      |             |
| - Pin exact version in package.json      | 5 min       |   0  | 5 min       |
| - Add to dependency-watch in SECURITY.md | 10 min      |   0  | 10 min      |
| - (Future) self-host peerjs-server       | 0.5 day     | 1 day| 1.5 days    |
| **Migrate to trystero (deferred):**      |             |      |             |
| - Shim `PeerLink`                        | 0.5 day     |      |             |
| - CSP + settings rename                  | 0.5 day     |      |             |
| - Full pairing-flow QA                   | —           | 1 day|             |
| - **Total**                              | **1 day**   |1 day | **2 days**  |
| **Migrate to simple-peer + own broker:** |             |      |             |
| - Self-host peerjs-server (the broker    | 0.5 day     | —    |             |
|   we'd need anyway)                      |             |      |             |
| - Rewrite `PeerLink` against simple-peer | 1.5 days    | —    |             |
|   + signaling client                     |             |      |             |
| - Full pairing-flow QA                   | —           | 1 day|             |
| - **Total**                              | **2 days**  |1 day | **3 days**  |

---

## 8. Action items applied this phase

| Action                                                                 | Status                                                               |
| ---------------------------------------------------------------------- | -------------------------------------------------------------------- |
| Verify PeerJS upstream maintenance state                               | **Done** — actively maintained, NOT archived.                        |
| Verify license of all candidates                                       | **Done** — peerjs MIT, simple-peer MIT, trystero MIT, peer-lite MIT. |
| Score candidates                                                       | **Done** — see section 2.                                            |
| Pick primary + backup                                                  | **Done** — primary: stay on peerjs; backup: trystero.                |
| Update `apps/copythat-mobile/package.json` to swap dep                 | **NOT applied** — see "Decision" rationale.                          |
| Update `apps/copythat-ui/package.json` (per Phase 42 brief instruction)| **N/A** — peerjs is not a dependency of `copythat-ui`. The brief    |
|                                                                        | targeted the wrong package.json; corrected in section 1.             |
| Document migration plan for future phase                               | **Done** — sections 4-7.                                             |

---

## 9. References

- PeerJS GitHub repo metadata API response (2026-04-26):
  `archived: false`, `pushed_at: 2026-02-27`, latest release `v1.5.5`
  (2025-06-07).
- npm `peerjs@1.5.5` registry record: license MIT, signed dist tarball,
  three current maintainers.
- simple-peer GitHub: `pushed_at: 2024-06-26`, last code commit
  `2022-02-17`.
- trystero GitHub: `pushed_at: 2026-04-26`, last commit
  `2026-04-26T19:31:49Z`, latest release `0.23.1` (2026-04-21).
- npm download counts (week of 2026-04-19 → 2026-04-25):
  peerjs 61,658 / simple-peer 230,253 / trystero 2,783 / peer-lite 56.
- GitHub Security Advisory database: zero advisories filed against
  `peerjs`, `simple-peer`, or `trystero` npm packages as of query.
