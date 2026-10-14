//! Rupa On-Device Speech Engine
//!
//! Provides on-device transcription (Whisper) and text-to-speech synthesis (Kokoro)
//! for privacy-first, zero-cloud desktop and web operation.

use serde::{Deserialize, Serialize};

pub mod models;
pub mod transcribe;
pub mod tts;

/// Individual recognized word with millisecond timestamp bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptionWord {
    pub start_ms: i64,
    pub end_ms: i64,
    pub word: String,
}

/// A subtitle / caption segment containing full text and word-level breakdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptionSegment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
    pub words: Vec<CaptionWord>,
}

/// Raw linear floating-point PCM audio buffer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioBuffer {
    pub sample_rate: u32,
    pub channels: u16,
    pub samples: Vec<f32>,
}

impl AudioBuffer {
    /// Creates a new mono audio buffer with specified sample rate.
    pub fn new_mono(sample_rate: u32, samples: Vec<f32>) -> Self {
        Self {
            sample_rate,
            channels: 1,
            samples,
        }
    }

    /// Duration in milliseconds.
    pub fn duration_ms(&self) -> i64 {
        if self.sample_rate == 0 || self.channels == 0 {
            return 0;
        }
        let total_frames = self.samples.len() / self.channels as usize;
        (total_frames as i64 * 1000) / self.sample_rate as i64
    }
}

/// Metadata describing an available offline TTS voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceProfile {
    pub id: String,
    pub name: String,
    pub language: String,
    pub gender: String,
}

/// Errors surfaced by the speech engine during model loading, transcription, or synthesis.
#[derive(Debug, thiserror::Error)]
pub enum SpeechError {
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

    #[error("Transcription failure: {0}")]
    TranscriptionError(String),

    #[error("TTS synthesis failure: {0}")]
    TtsError(String),

    #[error("Invalid audio format: {0}")]
    InvalidAudio(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
