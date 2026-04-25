// Phase 38 follow-up — stable per-PWA-install identity.
//
// The desktop side stores `phone_public_key_hex` per pairing
// entry; previously the PWA hardcoded `"00".repeat(32)` for every
// device, so every "paired" phone collided on the same identity
// and the SAS protocol degenerated into theatre. This module
// mints a 32-byte CSPRNG identity on first run, persists it in
// localStorage as hex, and reuses it across launches so the
// desktop can match a returning phone against its stored
// pairings.
//
// The identity is *not* a key-agreement keypair. The phone-desktop
// channel is WebRTC DTLS, which already provides confidentiality
// and integrity at the transport layer. The hex bytes here serve
// only as an identity tag the user (via SAS confirmation) binds
// to a specific physical phone. Adding `@noble/curves` for a real
// X25519 keypair is unnecessary for the current threat model and
// would inflate the PWA bundle by ~5 KB without changing the
// security properties.

const STORAGE_KEY = "copythat:phone-identity-v1";

/** Hex-encoded 32-byte identity tag for this phone install. */
export type PhoneIdentity = string;

function bytesToHex(bytes: Uint8Array): string {
  let s = "";
  for (let i = 0; i < bytes.length; i++) {
    s += bytes[i].toString(16).padStart(2, "0");
  }
  return s;
}

function isValidIdentity(s: unknown): s is PhoneIdentity {
  return typeof s === "string" && /^[0-9a-f]{64}$/.test(s);
}

/**
 * Returns this PWA install's stable identity, minting and persisting
 * one on first call. Subsequent calls (across launches) return the
 * same value.
 *
 * Throws when the host has neither localStorage nor crypto. Both
 * are available in every browser that meets the PWA's minimum
 * compatibility floor; throwing is preferred to silently
 * downgrading to a non-CSPRNG fallback.
 */
export function getOrMintPhoneIdentity(): PhoneIdentity {
  if (typeof localStorage === "undefined") {
    throw new Error("phone identity: localStorage unavailable");
  }
  const existing = localStorage.getItem(STORAGE_KEY);
  if (isValidIdentity(existing)) {
    return existing;
  }
  if (
    typeof crypto === "undefined" ||
    typeof crypto.getRandomValues !== "function"
  ) {
    throw new Error("phone identity: crypto.getRandomValues unavailable");
  }
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  const hex = bytesToHex(bytes);
  try {
    localStorage.setItem(STORAGE_KEY, hex);
  } catch (e) {
    // Storage quota / private-mode lockout. Carry on with the
    // freshly-minted value for this session; the user will be
    // prompted to re-pair next launch when the value differs.
    console.warn("phone identity: localStorage write failed", e);
  }
  return hex;
}

/**
 * 32-glyph SAS emoji table — must match the Rust constant
 * `copythat_mobile::pairing::SAS_EMOJI_TABLE` byte-for-byte so
 * both sides render the same string when fed the same inputs.
 */
const SAS_EMOJI_TABLE: readonly string[] = [
  "🐶", "🐱", "🦊", "🐼", "🐨", "🐸", "🦁", "🐯",
  "🐮", "🐷", "🐵", "🦄", "🦋", "🐢", "🐧", "🦉",
  "🌱", "🌳", "🌲", "🍀", "🌸", "🌻", "🌙", "⭐",
  "🌈", "🍎", "🍋", "🍇", "🍉", "🍓", "🍒", "🍌",
];

/**
 * Compute the four-emoji SAS string the user compares against the
 * desktop's rendering. Inputs are the seed from the pairing QR,
 * the desktop's public key (also from the QR), and this phone's
 * identity (`getOrMintPhoneIdentity`).
 *
 * Algorithm matches the desktop side
 * (`copythat_mobile::pairing::sas_fingerprint`): SHA-256 over
 * `seed || desktop_pubkey || phone_pubkey`, take the first 4
 * bytes, render each as an emoji from the 32-glyph table indexed
 * by `byte % 32`. Result is the 4 emojis concatenated (no
 * separator) — the desktop joins with a single space; the PWA
 * normalises whitespace before comparison so both forms match.
 */
export async function computeSasEmoji(
  seedHex: string,
  desktopPubkeyHex: string,
  phonePubkeyHex: string,
): Promise<string> {
  const seed = hexToBytes(seedHex);
  const desktop = hexToBytes(desktopPubkeyHex);
  const phone = hexToBytes(phonePubkeyHex);
  const concat = new Uint8Array(seed.length + desktop.length + phone.length);
  concat.set(seed, 0);
  concat.set(desktop, seed.length);
  concat.set(phone, seed.length + desktop.length);
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", concat));
  return [0, 1, 2, 3]
    .map((i) => SAS_EMOJI_TABLE[digest[i] % SAS_EMOJI_TABLE.length])
    .join("");
}

function hexToBytes(hex: string): Uint8Array {
  const trimmed = hex.replace(/[^0-9a-fA-F]/g, "");
  if (trimmed.length % 2 !== 0) {
    throw new Error(`hex string has odd length: ${hex}`);
  }
  const out = new Uint8Array(trimmed.length / 2);
  for (let i = 0; i < out.length; i++) {
    out[i] = parseInt(trimmed.substr(i * 2, 2), 16);
  }
  return out;
}
