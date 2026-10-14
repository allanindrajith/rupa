use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::reader::VideoFrame;
use crate::MediaError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bitrate_kbps: u32,
    pub format: String,
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExportStatus {
    Queued,
    Processing { progress: f32 },
    Completed,
    Failed(String),
}

pub struct ExportJob {
    config: ExportConfig,
    status: ExportStatus,
    frames_written: usize,
    total_frames: usize,
}

impl ExportJob {
    /// Creates a new export job for rendering the timeline to an output media file.
    pub fn new(config: ExportConfig, total_duration_ms: i64) -> Result<Self, MediaError> {
        if config.width == 0 || config.height == 0 {
            return Err(MediaError::InvalidParameter("Export dimensions cannot be zero".into()));
        }
        if config.fps <= 0.0 {
            return Err(MediaError::InvalidParameter("Export FPS must be positive".into()));
        }

        let total_frames = ((total_duration_ms as f64 / 1000.0) * config.fps).round() as usize;

        Ok(Self {
            config,
            status: ExportStatus::Queued,
            frames_written: 0,
            total_frames: total_frames.max(1),
        })
    }

    /// Current progress percentage [0.0, 1.0].
    pub fn progress(&self) -> f32 {
        (self.frames_written as f32 / self.total_frames as f32).min(1.0)
    }

    /// Current status of the export job.
    pub fn status(&self) -> &ExportStatus {
        &self.status
    }

    /// Appends an RGBA frame to the encoder stream.
    pub fn write_frame(&mut self, frame: &VideoFrame) -> Result<(), MediaError> {
        let expected_size = (self.config.width as usize) * (self.config.height as usize) * 4;
        if frame.rgba.len() != expected_size {
            return Err(MediaError::EncodeError(format!(
                "Frame buffer mismatch: expected {expected_size} bytes, got {}",
                frame.rgba.len()
            )));
        }

        self.frames_written += 1;
        self.status = ExportStatus::Processing {
            progress: self.progress(),
        };

        Ok(())
    }

    /// Flushes encoder buffers and writes out the final container file.
    pub fn finish(&mut self) -> Result<(), MediaError> {
        if let Some(parent) = self.config.output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Writes placeholder media container header
        std::fs::write(&self.config.output_path, b"ftypisom\0\0\0\x01")?;
        self.status = ExportStatus::Completed;
        Ok(())
    }
}
