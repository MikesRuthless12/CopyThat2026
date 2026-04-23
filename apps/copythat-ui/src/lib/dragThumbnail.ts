// Phase 29 — drag thumbnails via HTML5 `setDragImage`.
//
// Usage:
//
//   <div draggable="true" ondragstart={(ev) => applyDragThumbnail(ev, items)} />
//
// Where `items` is one or more paths (or image URLs). The helper
// renders a transient canvas offscreen, calls `setDragImage`, and
// cleans the canvas up on the next frame (the browser takes a
// snapshot synchronously inside `setDragImage`, so post-frame
// cleanup is safe).
//
// On Win11 the OS composites its own drag preview when the drag
// leaves the window — this helper controls only the in-app drag
// thumbnail. OS-native polish lives in the Tauri window flags
// (see src-tauri/src/lib.rs).

export interface DragThumbnailItem {
  /** Absolute path or URL. Displayed as the caption. */
  label: string;
  /** Optional image data URL for image-kind items. If present we
   *  paint it; otherwise we draw a generic file glyph. */
  imageDataUrl?: string;
}

const THUMB_WIDTH = 160;
const THUMB_HEIGHT = 40;
const STACK_OFFSET = 4;

export function applyDragThumbnail(
  ev: DragEvent,
  items: DragThumbnailItem[],
  enabled: boolean = true,
): void {
  if (!enabled) return;
  if (!ev.dataTransfer) return;
  if (items.length === 0) return;

  const canvas = document.createElement("canvas");
  const dpr = Math.min(window.devicePixelRatio || 1, 2);
  const stackDepth = Math.min(items.length, 3);
  canvas.width = (THUMB_WIDTH + (stackDepth - 1) * STACK_OFFSET) * dpr;
  canvas.height = (THUMB_HEIGHT + (stackDepth - 1) * STACK_OFFSET) * dpr;
  canvas.style.position = "absolute";
  canvas.style.top = "-9999px";
  canvas.style.left = "-9999px";
  canvas.style.width = `${THUMB_WIDTH + (stackDepth - 1) * STACK_OFFSET}px`;
  canvas.style.height = `${THUMB_HEIGHT + (stackDepth - 1) * STACK_OFFSET}px`;
  document.body.appendChild(canvas);

  const ctx = canvas.getContext("2d");
  if (!ctx) {
    // No canvas support — fall through silently; the OS will use
    // its own default drag image (the dragged element itself).
    document.body.removeChild(canvas);
    return;
  }
  ctx.scale(dpr, dpr);

  // Draw from bottom of the stack up so the topmost tile is the head.
  for (let i = stackDepth - 1; i >= 0; i--) {
    const offsetX = i * STACK_OFFSET;
    const offsetY = i * STACK_OFFSET;
    drawTile(ctx, items[Math.min(i, items.length - 1)], offsetX, offsetY);
  }
  // If there are more items than visible tiles, paint a "+N" badge.
  if (items.length > stackDepth) {
    const extra = items.length - stackDepth;
    ctx.fillStyle = "#dc2626";
    ctx.font = "bold 11px system-ui, -apple-system, Segoe UI, sans-serif";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    const bx = THUMB_WIDTH + (stackDepth - 1) * STACK_OFFSET - 16;
    const by = 12;
    ctx.beginPath();
    ctx.arc(bx, by, 10, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = "#fff";
    ctx.fillText(`+${extra}`, bx, by);
  }

  // setDragImage takes a snapshot synchronously; schedule removal.
  ev.dataTransfer.setDragImage(canvas, 12, 12);
  requestAnimationFrame(() => {
    try {
      document.body.removeChild(canvas);
    } catch {
      // Already removed.
    }
  });
}

function drawTile(
  ctx: CanvasRenderingContext2D,
  item: DragThumbnailItem,
  x: number,
  y: number,
) {
  // Tile card.
  ctx.fillStyle = "#ffffff";
  ctx.strokeStyle = "#d1d5db";
  ctx.lineWidth = 1;
  roundRect(ctx, x, y, THUMB_WIDTH, THUMB_HEIGHT, 4);
  ctx.fill();
  ctx.stroke();

  // Image swatch, or file glyph fallback.
  if (item.imageDataUrl) {
    const img = new Image();
    img.src = item.imageDataUrl;
    // Synchronous — for drag previews we accept "not yet loaded"
    // and fall back to the glyph on first-drag. Subsequent drags
    // the image will be cached.
    if (img.complete && img.naturalWidth > 0) {
      ctx.save();
      const iw = 28;
      const ih = 28;
      ctx.drawImage(img, x + 6, y + 6, iw, ih);
      ctx.restore();
    } else {
      drawGlyph(ctx, x + 6, y + 6, 28);
    }
  } else {
    drawGlyph(ctx, x + 6, y + 6, 28);
  }

  // Caption — truncate to fit.
  ctx.fillStyle = "#111827";
  ctx.font = "12px system-ui, -apple-system, Segoe UI, sans-serif";
  ctx.textAlign = "left";
  ctx.textBaseline = "middle";
  const label = truncate(item.label, 20);
  ctx.fillText(label, x + 40, y + THUMB_HEIGHT / 2);
}

function drawGlyph(ctx: CanvasRenderingContext2D, x: number, y: number, size: number) {
  ctx.save();
  ctx.fillStyle = "#e5e7eb";
  ctx.strokeStyle = "#9ca3af";
  ctx.lineWidth = 1;
  roundRect(ctx, x, y, size, size, 3);
  ctx.fill();
  ctx.stroke();
  ctx.fillStyle = "#6b7280";
  ctx.font = `${size * 0.55}px system-ui, -apple-system, Segoe UI`;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText("📄", x + size / 2, y + size / 2);
  ctx.restore();
}

function roundRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  w: number,
  h: number,
  r: number,
) {
  ctx.beginPath();
  ctx.moveTo(x + r, y);
  ctx.lineTo(x + w - r, y);
  ctx.quadraticCurveTo(x + w, y, x + w, y + r);
  ctx.lineTo(x + w, y + h - r);
  ctx.quadraticCurveTo(x + w, y + h, x + w - r, y + h);
  ctx.lineTo(x + r, y + h);
  ctx.quadraticCurveTo(x, y + h, x, y + h - r);
  ctx.lineTo(x, y + r);
  ctx.quadraticCurveTo(x, y, x + r, y);
  ctx.closePath();
}

function truncate(s: string, max: number): string {
  const basename = (p: string) => {
    const m = p.match(/[^\\/]+$/);
    return m ? m[0] : p;
  };
  const name = basename(s);
  if (name.length <= max) return name;
  return name.slice(0, max - 1) + "…";
}

// Test hooks.
export const __dragThumbnailForTesting = {
  THUMB_WIDTH,
  THUMB_HEIGHT,
  STACK_OFFSET,
  truncate,
};
