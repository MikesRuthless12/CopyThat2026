/**
 * §4.6 Cloud (Phase 32) — Manual UI golden path.
 */

import { test } from "./fixtures/test";

test.describe("§4.6 Cloud (Phase 32)", () => {
  test.fixme(
    "Add S3 backend → Test connection → green",
    async ({ page: _page, tauri: _tauri }) => {
      // Open SettingsModal → Remotes tab. Click "Add backend" →
      // pick S3. Fill bucket / region / key. Click "Test
      // connection" → mock `cloud_test_connection` to return
      // `{ ok: true, latencyMs: 42 }`. Assert the success badge
      // renders. Click "Save" → `cloud_remote_save` invoked
      // with the form payload.
      //
      // Then: stage a copy whose dst points at the new backend
      // (`s3://bucket/path`); assert the progress flow surfaces
      // the same JobRow shape as a local copy (events arrive
      // identically — the cloud half is engine-side).
    },
  );

  test.fixme(
    "Add Dropbox backend via OAuth PKCE → backend listed",
    async ({ page: _page, tauri: _tauri }) => {
      // Click "Add backend" → "Dropbox". The OAuth flow opens an
      // external browser; mock `cloud_oauth_start` to return a
      // synthetic `redirectUrl`. Drive the completion via an
      // emitted `cloud-oauth-complete { backend: "dropbox", ok: true }`
      // event. Assert the backend list now includes the Dropbox
      // entry; assert no token text leaks into the rendered DOM.
    },
  );

  test.fixme(
    "Copy from a backend back to local → file lands",
    async ({ page: _page, tauri: _tauri }) => {
      // Drive a copy where `sources` is an `s3://...` URI and
      // `destination` is a local path. The DropStagingDialog
      // should accept the cloud path; `start_copy` fires with
      // the URI verbatim; the row renders with the cloud-source
      // icon. The actual download is engine-side.
    },
  );
});
