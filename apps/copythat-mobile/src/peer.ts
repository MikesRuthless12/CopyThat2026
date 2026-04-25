// PeerJS data-channel wrapper with reconnect + typed message
// dispatch.

import type { DataConnection } from "peerjs";
import Peer from "peerjs";

import type { RemoteCommand, RemoteResponse } from "./protocol";

export type PeerStatus =
  | { kind: "idle" }
  | { kind: "connecting"; desktopPeerId: string }
  | { kind: "connected"; desktopPeerId: string }
  | { kind: "error"; message: string }
  | { kind: "disconnected" };

export type PeerEventListener = (event: RemoteResponse) => void;

/// Manages the PeerJS lifecycle from the phone side.
///
/// - `connect(desktopPeerId)` opens a data channel against the
///   given peer-id (looked up via the PeerJS broker).
/// - `send(cmd)` writes a typed `RemoteCommand`; resolves with the
///   matching `RemoteResponse` paired by request id.
/// - Streaming `RemoteResponse`s the desktop pushes (job_progress,
///   globals_tick, etc.) flow through the listener registered via
///   `onEvent`.
/// - `disconnect()` closes the data channel + the PeerJS client.
///   The phone-side Exit button calls this.
export class PeerLink {
  private peer: Peer | null = null;
  private conn: DataConnection | null = null;
  private status: PeerStatus = { kind: "idle" };
  private statusListeners: Array<(status: PeerStatus) => void> = [];
  private eventListeners: PeerEventListener[] = [];
  private pendingRequests: Map<
    number,
    { resolve: (resp: RemoteResponse) => void; reject: (err: unknown) => void }
  > = new Map();
  private nextRequestId = 1;

  connect(desktopPeerId: string, brokerHost?: string): void {
    this.disconnect();
    this.setStatus({ kind: "connecting", desktopPeerId });
    const opts = brokerHost
      ? { host: brokerHost, secure: true, path: "/" }
      : undefined;
    this.peer = opts
      ? new Peer(undefined as unknown as string, opts)
      : new Peer();

    this.peer.on("open", () => {
      const conn = this.peer!.connect(desktopPeerId, {
        serialization: "json",
      });
      this.conn = conn;
      conn.on("open", () => {
        this.setStatus({ kind: "connected", desktopPeerId });
      });
      conn.on("data", (data: unknown) => {
        this.handleIncoming(data);
      });
      conn.on("close", () => {
        this.setStatus({ kind: "disconnected" });
      });
      conn.on("error", (err) => {
        this.setStatus({ kind: "error", message: String(err) });
      });
    });

    this.peer.on("error", (err) => {
      const message = err && (err as { type?: string }).type === "peer-unavailable"
        ? "Desktop is not online — start Copy That on your computer."
        : `${err}`;
      this.setStatus({ kind: "error", message });
    });

    this.peer.on("disconnected", () => {
      this.setStatus({ kind: "disconnected" });
    });
  }

  /// Send a typed RemoteCommand and resolve with the matching
  /// RemoteResponse. The desktop replies inline by including the
  /// same `req_id` field on the response object — matched by the
  /// `pendingRequests` map below.
  async send(cmd: RemoteCommand, timeoutMs = 10_000): Promise<RemoteResponse> {
    if (!this.conn || this.status.kind !== "connected") {
      throw new Error("not connected");
    }
    const reqId = this.nextRequestId++;
    const wire = { req_id: reqId, ...cmd };
    return new Promise<RemoteResponse>((resolve, reject) => {
      this.pendingRequests.set(reqId, { resolve, reject });
      this.conn!.send(wire);
      setTimeout(() => {
        if (this.pendingRequests.has(reqId)) {
          this.pendingRequests.delete(reqId);
          reject(new Error("timeout"));
        }
      }, timeoutMs);
    });
  }

  onEvent(listener: PeerEventListener): () => void {
    this.eventListeners.push(listener);
    return () => {
      this.eventListeners = this.eventListeners.filter((l) => l !== listener);
    };
  }

  onStatus(listener: (status: PeerStatus) => void): () => void {
    this.statusListeners.push(listener);
    listener(this.status);
    return () => {
      this.statusListeners = this.statusListeners.filter((l) => l !== listener);
    };
  }

  getStatus(): PeerStatus {
    return this.status;
  }

  /// Phone-side Exit. Sends a `Goodbye`, then closes both the data
  /// channel and the PeerJS client so the next session has to
  /// rehandshake from scratch.
  async disconnect(): Promise<void> {
    if (this.conn && this.status.kind === "connected") {
      try {
        await this.send({ kind: "goodbye" }, 1000);
      } catch {
        // best-effort
      }
    }
    this.conn?.close();
    this.peer?.destroy();
    this.conn = null;
    this.peer = null;
    this.pendingRequests.forEach((p) =>
      p.reject(new Error("disconnected")),
    );
    this.pendingRequests.clear();
    this.setStatus({ kind: "idle" });
  }

  private setStatus(status: PeerStatus): void {
    this.status = status;
    this.statusListeners.forEach((l) => l(status));
  }

  private handleIncoming(data: unknown): void {
    if (!data || typeof data !== "object") return;
    const obj = data as Record<string, unknown>;
    const reqId = typeof obj.req_id === "number" ? obj.req_id : undefined;
    const resp = obj as RemoteResponse;
    if (reqId !== undefined && this.pendingRequests.has(reqId)) {
      const handler = this.pendingRequests.get(reqId)!;
      this.pendingRequests.delete(reqId);
      handler.resolve(resp);
    } else {
      // Streaming event (no matching request) — fire onEvent.
      this.eventListeners.forEach((l) => l(resp));
    }
  }
}
