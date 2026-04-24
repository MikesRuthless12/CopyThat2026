//! Phase 35 — zstd compression stage.
//!
//! Two public entry points:
//!
//! - [`should_compress`] — pure function that decides whether a
//!   given file extension is elegible under a
//!   [`CompressionPolicy`]. The engine uses this up-front to decide
//!   whether to append `.zst` to the destination name.
//! - [`compressed_writer`] — wraps an inner `Write` in a
//!   [`CompressionSink`] that tracks pre/post-compression byte
//!   counts and flushes the zstd encoder's epilogue on drop.
//!
//! The [`DEFAULT_DENY_EXTENSIONS`] list is the canonical
//! "already-compressed" media types that zstd shouldn't waste CPU
//! on — media (jpg / mp4 / webm / mkv / flac), archives (zip / 7z /
//! gz / xz), and distributable blobs (pdf / docx / apk / msi / exe
//! / iso).

use std::io::{self, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::error::{CryptError, Result};
use crate::policy::{CompressionLevel, CompressionPolicy};

/// Canonical already-compressed media + archive extensions. Matches
/// the Phase 35 brief's `deny_extensions` default. Lower-case + no
/// leading dot so the policy's case-insensitive match is cheap.
pub const DEFAULT_DENY_EXTENSIONS: &[&str] = &[
    // Images
    "jpg", "jpeg", "png", "gif", "webp", "avif", "heic", "heif", // Audio
    "mp3", "m4a", "aac", "flac", "ogg", "opus", "wav", // Video
    "mp4", "mov", "mkv", "webm", "avi", "wmv", // Compressed archives
    "zip", "7z", "gz", "tgz", "bz2", "xz", "zstd", "zst", "lz4", "br", "rar",
    // Office / distribution blobs (internally zipped)
    "pdf", "docx", "xlsx", "pptx", "keynote", "numbers", "pages", "epub",
    // Installers / disk images
    "apk", "msi", "exe", "dmg", "wim", "iso", "img",
];

/// Accumulates compression metrics for one file. Shared between the
/// sink's drop (which writes the zstd epilogue) and the engine
/// event emitter (which fires [`CompressionSavings`] after the
/// encoder finalises).
#[derive(Debug, Default, Clone)]
pub struct CompressionMetrics {
    inner: Arc<MetricsInner>,
}

#[derive(Debug, Default)]
struct MetricsInner {
    input_bytes: AtomicU64,
    output_bytes: AtomicU64,
    /// `true` once the sink's Drop runs. The engine reads this to
    /// know the zstd epilogue landed before snapshotting the
    /// `output_bytes`. Serialised via Mutex rather than atomic
    /// because it's checked exactly once.
    finalised: Mutex<bool>,
}

impl CompressionMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bytes read from the source (pre-compression).
    pub fn input_bytes(&self) -> u64 {
        self.inner.input_bytes.load(Ordering::Acquire)
    }

    /// Bytes written to the inner `Write` (post-compression). Stable
    /// once [`Self::is_finalised`] returns true.
    pub fn output_bytes(&self) -> u64 {
        self.inner.output_bytes.load(Ordering::Acquire)
    }

    /// `true` once the encoder's epilogue has been flushed. The
    /// sink's `Drop` (or an explicit `finish()` call) flips this.
    pub fn is_finalised(&self) -> bool {
        *self.inner.finalised.lock().expect("metrics mutex poisoned")
    }

    /// Compute the ratio output / input. Returns `None` when
    /// `input_bytes` is zero (e.g. no bytes flowed through yet).
    pub fn ratio(&self) -> Option<f64> {
        let input = self.input_bytes();
        if input == 0 {
            return None;
        }
        Some(self.output_bytes() as f64 / input as f64)
    }

    /// Bytes saved by compression — `input - output` clamped at
    /// zero (when zstd produced a larger output than the input,
    /// which can happen for pre-compressed media at very low
    /// levels).
    pub fn bytes_saved(&self) -> u64 {
        self.input_bytes().saturating_sub(self.output_bytes())
    }

    fn bump_input(&self, n: u64) {
        self.inner.input_bytes.fetch_add(n, Ordering::AcqRel);
    }

    fn bump_output(&self, n: u64) {
        self.inner.output_bytes.fetch_add(n, Ordering::AcqRel);
    }

    fn mark_finalised(&self) {
        *self.inner.finalised.lock().expect("metrics mutex poisoned") = true;
    }
}

/// Snapshot of the counters at a single moment. Used by the engine
/// to emit `CopyEvent::CompressionSavings` once the encoder has
/// finalised.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompressionSinkStats {
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub ratio: f64,
    pub bytes_saved: u64,
}

impl CompressionSinkStats {
    pub fn from_metrics(m: &CompressionMetrics) -> Option<Self> {
        let input_bytes = m.input_bytes();
        if input_bytes == 0 {
            return None;
        }
        let output_bytes = m.output_bytes();
        Some(Self {
            input_bytes,
            output_bytes,
            ratio: output_bytes as f64 / input_bytes as f64,
            bytes_saved: input_bytes.saturating_sub(output_bytes),
        })
    }
}

/// Decide whether to compress a file with the given extension under
/// `policy`. Helper for the engine — call up-front before opening
/// the destination writer so `.zst` can be appended to the path
/// when we're going to compress.
pub fn should_compress(policy: &CompressionPolicy, file_ext: &str) -> Option<CompressionLevel> {
    policy.effective_level(file_ext)
}

/// Wrap `inner` in a zstd encoder at `level`, instrumented with a
/// [`CompressionMetrics`] handle the caller shares with the engine
/// for later event emission. The returned writer must be dropped
/// (or explicitly finished) before the caller consumes the final
/// compressed bytes — zstd writes an epilogue frame.
pub fn compressed_writer<W>(
    inner: W,
    level: CompressionLevel,
    metrics: CompressionMetrics,
) -> Result<CompressionSink<W>>
where
    W: Write,
{
    let counting = CountingWriter {
        inner,
        metrics: metrics.clone(),
    };
    let encoder = zstd::stream::Encoder::new(counting, level.as_i32())
        .map_err(|e| CryptError::Zstd(e.to_string()))?;
    Ok(CompressionSink {
        encoder: Some(encoder),
        metrics,
    })
}

/// Owns the zstd encoder + forwards writes into it. Drop finalises
/// the encoder (writes the epilogue) and marks the metrics as
/// finalised so the engine can emit its `CompressionSavings` event.
pub struct CompressionSink<W: Write> {
    // `Option` so `Drop::drop` can `take` the encoder and call
    // `finish()` (consuming it) without leaving a half-dropped
    // instance behind.
    encoder: Option<zstd::stream::Encoder<'static, CountingWriter<W>>>,
    metrics: CompressionMetrics,
}

impl<W: Write> CompressionSink<W> {
    /// Explicit finish — ends the zstd stream, flushes the
    /// underlying writer, and returns the inner `W` + final
    /// metrics. Callers that need the underlying writer back
    /// (e.g. to chain into age's encryptor in the opposite
    /// order) use this instead of relying on `Drop`.
    pub fn finish(mut self) -> Result<(W, CompressionMetrics)> {
        let encoder = self
            .encoder
            .take()
            .expect("compression sink already finished");
        let counting = encoder
            .finish()
            .map_err(|e| CryptError::Zstd(e.to_string()))?;
        self.metrics.mark_finalised();
        Ok((counting.inner, self.metrics.clone()))
    }
}

impl<W: Write> Write for CompressionSink<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let encoder = self
            .encoder
            .as_mut()
            .expect("compression sink already finished");
        let n = encoder.write(buf)?;
        self.metrics.bump_input(n as u64);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(e) = self.encoder.as_mut() {
            e.flush()?;
        }
        Ok(())
    }
}

impl<W: Write> Drop for CompressionSink<W> {
    fn drop(&mut self) {
        // Caller didn't call `finish()` — run the finalisation
        // anyway so the inner file isn't left with a truncated
        // zstd stream. Errors are swallowed (Drop can't return
        // Result) but the metrics won't flip to finalised in that
        // case, which is a signal to the engine to skip the
        // savings event.
        if let Some(encoder) = self.encoder.take()
            && encoder.finish().is_ok()
        {
            self.metrics.mark_finalised();
        }
    }
}

/// Wraps the *inner* writer (the one zstd writes compressed bytes
/// into) so we can tally `output_bytes` without touching the
/// encoder. Input bytes are counted one layer up (in the
/// [`CompressionSink::write`] impl).
struct CountingWriter<W: Write> {
    inner: W,
    metrics: CompressionMetrics,
}

impl<W: Write> Write for CountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.metrics.bump_output(n as u64);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn default_deny_extensions_include_common_media() {
        let set: HashSet<&str> = DEFAULT_DENY_EXTENSIONS.iter().copied().collect();
        for ext in ["jpg", "mp4", "zip", "7z", "gz", "pdf"] {
            assert!(set.contains(ext), "expected `{ext}` in default deny set");
        }
    }

    #[test]
    fn should_compress_honours_smart_deny_list() {
        let policy = CompressionPolicy::smart();
        assert!(should_compress(&policy, "txt").is_some());
        assert!(should_compress(&policy, "log").is_some());
        assert!(should_compress(&policy, "jpg").is_none());
    }

    #[test]
    fn compress_round_trips_with_metrics() {
        let metrics = CompressionMetrics::new();
        let mut buf: Vec<u8> = Vec::new();
        let mut sink = compressed_writer(&mut buf, CompressionLevel(3), metrics.clone()).unwrap();
        let payload: Vec<u8> = std::iter::repeat_n(b'a', 64 * 1024).collect();
        sink.write_all(&payload).unwrap();
        sink.finish().unwrap();

        assert!(metrics.is_finalised());
        assert_eq!(metrics.input_bytes(), payload.len() as u64);
        assert!(metrics.output_bytes() < payload.len() as u64);
        assert!(metrics.bytes_saved() > 0);

        let round_trip = zstd::decode_all(&buf[..]).unwrap();
        assert_eq!(round_trip.len(), payload.len());
        assert_eq!(round_trip, payload);
    }
}
