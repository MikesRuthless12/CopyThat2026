//! `ShapeSink` implementation that the engine attaches to
//! `CopyOptions::shape`.
//!
//! Stateless wrapper around an `Arc<Shape>` — the runner builds one
//! per job at enqueue time so the engine has the right limiter
//! handle baked in. Multiple in-flight copies sharing the same
//! [`Shape`] cooperatively share the bucket; the GCRA implementation
//! is fair under contention.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use copythat_core::ShapeSink;

use crate::shape::Shape;

/// Engine-ready bridge. Cloning is cheap (one Arc bump) so the
/// runner can hand a fresh sink to every queued job without paying
/// for the underlying GCRA state.
#[derive(Debug, Clone)]
pub struct CopyThatShapeSink {
    shape: Arc<Shape>,
}

impl CopyThatShapeSink {
    pub fn new(shape: Arc<Shape>) -> Self {
        Self { shape }
    }

    /// Surface the wrapped shape so the runner / IPC layer can read
    /// the current rate (for the header badge) without holding a
    /// second handle.
    pub fn shape(&self) -> Arc<Shape> {
        self.shape.clone()
    }
}

impl ShapeSink for CopyThatShapeSink {
    fn permit<'a>(&'a self, bytes: u64) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        // Capture the Arc<Shape> into the boxed future so the engine
        // can `.await` without holding a reference to the sink past
        // the call site. `Shape::permit` itself is `&self` and
        // internally uses `ArcSwap::load_full` so this future never
        // pins the bucket either.
        let shape = self.shape.clone();
        Box::pin(async move { shape.permit(bytes).await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::ByteRate;
    use std::time::Instant;

    #[tokio::test(flavor = "multi_thread")]
    async fn unlimited_sink_is_a_no_op() {
        let shape = Arc::new(Shape::new(None));
        let sink = CopyThatShapeSink::new(shape);
        let started = Instant::now();
        sink.permit(8 * 1024 * 1024).await;
        assert!(started.elapsed().as_millis() < 50);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn limited_sink_blocks_for_the_expected_duration() {
        // Same "drain the burst first" pattern as `Shape`'s unit
        // test — a fresh bucket has one second's worth of cells
        // ready, so the first ~8 MiB go through instantly.
        let shape = Arc::new(Shape::new(Some(ByteRate::mebibytes_per_second(8))));
        let sink = CopyThatShapeSink::new(shape);
        sink.permit(4 * 1024 * 1024).await;
        sink.permit(4 * 1024 * 1024).await;
        let started = Instant::now();
        sink.permit(4 * 1024 * 1024).await;
        let ms = started.elapsed().as_millis();
        assert!(
            (300..=2000).contains(&ms),
            "post-burst sink permit took {ms} ms (expected 300..=2000)"
        );
    }
}
