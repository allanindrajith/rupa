//! Rupa On-Device AI Vision Engine
//!
//! Provides on-device portrait segmentation, background removal, and smart cutout
//! masks for privacy-first, zero-cloud desktop and web operation.

pub mod mask;
pub mod models;
pub mod segment;

pub use mask::CutoutMask;
pub use segment::SegmentationEngine;


/// Errors surfaced by the vision engine during model execution, mask processing, or image encoding.
#[derive(Debug, thiserror::Error)]
pub enum VisionError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Failed to download model: {0}")]
    DownloadError(String),

    #[error("Checksum mismatch for {model}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        model: String,
        expected: String,
        actual: String,
    },

    #[error("Invalid dimensions: expected {expected:?}, got {actual:?}")]
    InvalidDimensions {
        expected: (u32, u32),
        actual: (u32, u32),
    },

    #[error("Buffer length mismatch: expected {expected} bytes, got {actual} bytes")]
    BufferMismatch {
        expected: usize,
        actual: usize,
    },

    #[error("PNG encoding/decoding error: {0}")]
    PngError(String),

    #[error("Inference failure: {0}")]
    InferenceError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
