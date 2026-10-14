//! Rupa Native Media & Video Pipeline Engine
//!
//! Provides file metadata probing, video frame decoding, audio track mixing,
//! and hardware-accelerated video export for privacy-first desktop and web operation.

pub mod audio;
pub mod export;
pub mod probe;
pub mod reader;

pub use audio::{AudioMixer, AudioTrackClip};
pub use export::{ExportConfig, ExportJob, ExportStatus};
pub use probe::{AudioTrackInfo, MediaMetadata, MediaProbe, VideoTrackInfo};
pub use reader::{FrameReader, VideoFrame};

/// Errors encountered during media probing, decoding, mixing, or export.
#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Unsupported codec or format: {0}")]
    UnsupportedCodec(String),

    #[error("Decoding failure: {0}")]
    DecodeError(String),

    #[error("Encoding failure: {0}")]
    EncodeError(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
