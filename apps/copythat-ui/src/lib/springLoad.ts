// Phase 29 — spring-loaded folder action.
//
// Svelte 5-compatible action (plain closure, no `Action<>` import — we
// target both DOM-event and programmatic drags). Use:
//
//   <div use:springLoad={{ delayMs: 650, onTrigger: () => openFolder() }}>
//
// Semantics:
// - Start a timer on `dragenter`. Fire `onTrigger` after `delayMs`.
// - Cancel the timer on `dragleave` (unless we re-enter a descendant).
// - Descendant bounce: browsers fire leave+enter when the drag crosses
//   child elements. We debounce the cancel by one frame so crossing
//   into a child doesn't trip the cancel path.
// - `drop` fires the drop (consumer handles it) AND cancels the pending
//   spring-load.
// - Cascade: when spring-load fires, the consumer typically re-mounts
//   the component with a new folder. The action is torn down on unmount
//   and re-attached to the next target, restarting the timer chain.
//
// Tests cover the timer math via `__springLoadForTesting` exports; the
// action itself is exercised by the Phase 29 DOM smoke test.

export interface SpringLoadOptions {
  /** Delay in milliseconds before `onTrigger` fires. Clamped 50..5000. */
  delayMs?: number;
  /** Called after the user has hovered for `delayMs` without leaving. */
  onTrigger: () => void;
  /** Optional — called when the drag enters the target (spring starts). */
  onEnter?: () => void;
  /** Optional — called when the drag cancels before `delayMs` elapses. */
  onCancel?: () => void;
  /** Pause spring-load (e.g. when Settings toggles it off). */
  disabled?: boolean;
}

const MIN_DELAY = 50;
const MAX_DELAY = 5000;
const LEAVE_DEBOUNCE_MS = 50;

export function springLoad(node: HTMLElement, options: SpringLoadOptions) {
  let current: SpringLoadOptions = { ...options };
  let timer: ReturnType<typeof setTimeout> | null = null;
  let leaveTimer: ReturnType<typeof setTimeout> | null = null;
  let armed = false;

  function clampDelay(d: number | undefined): number {
    const n = typeof d === "number" && Number.isFinite(d) ? d : 650;
    return Math.max(MIN_DELAY, Math.min(MAX_DELAY, n));
  }

  function clearTimers() {
    if (timer !== null) {
      clearTimeout(timer);
      timer = null;
    }
    if (leaveTimer !== null) {
      clearTimeout(leaveTimer);
      leaveTimer = null;
    }
  }

  function start() {
    if (current.disabled) return;
    if (armed) return;
    armed = true;
    current.onEnter?.();
    const delay = clampDelay(current.delayMs);
    timer = setTimeout(() => {
      timer = null;
      armed = false;
      current.onTrigger();
    }, delay);
  }

  function cancel(via: "leave" | "drop" | "destroy") {
    if (!armed) return;
    armed = false;
    if (timer !== null) {
      clearTimeout(timer);
      timer = null;
      if (via !== "destroy") current.onCancel?.();
    }
  }

  function handleEnter(_ev: DragEvent) {
    // Cancel any pending leave debounce — we're back inside the tree.
    if (leaveTimer !== null) {
      clearTimeout(leaveTimer);
      leaveTimer = null;
    }
    start();
  }

  function handleLeave(ev: DragEvent) {
    // Defer the cancel — the browser may fire `dragleave` on this node
    // immediately followed by `dragenter` on a descendant. If we cancel
    // synchronously we'd lose the timer across every child-element
    // boundary inside the drop zone.
    if (leaveTimer !== null) clearTimeout(leaveTimer);
    const related = ev.relatedTarget as Node | null;
    if (related && node.contains(related)) return; // still inside
    leaveTimer = setTimeout(() => {
      leaveTimer = null;
      cancel("leave");
    }, LEAVE_DEBOUNCE_MS);
  }

  function handleOver(ev: DragEvent) {
    // Allow the drop so the browser lets the pointer sit on the node.
    // Without this, Chrome/Edge treat the target as non-droppable and
    // the spring-load pointer-cursor effect doesn't show.
    if (!current.disabled) ev.preventDefault();
  }

  function handleDrop(_ev: DragEvent) {
    // The drop happened before spring-load fired — cancel it so we
    // don't navigate-into-folder after the caller already consumed the
    // drop on the parent.
    cancel("drop");
  }

  node.addEventListener("dragenter", handleEnter);
  node.addEventListener("dragleave", handleLeave);
  node.addEventListener("dragover", handleOver);
  node.addEventListener("drop", handleDrop);

  return {
    update(next: SpringLoadOptions) {
      current = { ...next };
      if (next.disabled && armed) cancel("leave");
    },
    destroy() {
      clearTimers();
      armed = false;
      node.removeEventListener("dragenter", handleEnter);
      node.removeEventListener("dragleave", handleLeave);
      node.removeEventListener("dragover", handleOver);
      node.removeEventListener("drop", handleDrop);
    },
  };
}

// ---------------------------------------------------------------------
// Test hooks — exported so the Phase 29 unit tests can drive the
// state machine without a full DOM. Not part of the public API.
// ---------------------------------------------------------------------

export const __springLoadForTesting = {
  MIN_DELAY,
  MAX_DELAY,
  LEAVE_DEBOUNCE_MS,
};
