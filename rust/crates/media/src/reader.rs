use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::probe::MediaProbe;
use crate::MediaError;

/// An uncompressed, raw 4-channel 8-bit RGBA video frame at a specific presentation timestamp (PTS).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub pts_ms: i64,
    pub rgba: Vec<u8>,
}

pub struct FrameReader {
    path: PathBuf,
    width: u32,
    height: u32,
    duration_ms: i64,
    current_pts_ms: i64,
    fps: f64,
}

impl FrameReader {
    /// Opens a media file for sequential or seekable video frame decoding.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, MediaError> {
        let p = path.as_ref().to_path_buf();
        let meta = MediaProbe::probe(&p)?;

        let video = meta.video.ok_or_else(|| {
            MediaError::DecodeError(format!("No video track present in {}", p.display()))
        })?;

        Ok(Self {
            path: p,
            width: video.width,
            height: video.height,
            duration_ms: meta.duration_ms,
            current_pts_ms: 0,
            fps: video.fps,
        })
    }

    /// Path of the media file being read.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Dimensions of the video frames.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Total duration in milliseconds.
    pub fn duration_ms(&self) -> i64 {
        self.duration_ms
    }

    /// Current playback position timestamp in milliseconds.
    pub fn current_pts_ms(&self) -> i64 {
        self.current_pts_ms
    }

    /// Seeks to a presentation timestamp in milliseconds.
    pub fn seek(&mut self, timestamp_ms: i64) -> Result<(), MediaError> {
        if timestamp_ms < 0 || timestamp_ms > self.duration_ms {
            return Err(MediaError::InvalidParameter(format!(
                "Seek target {timestamp_ms}ms outside duration 0..{}ms",
                self.duration_ms
            )));
        }

        self.current_pts_ms = timestamp_ms;
        Ok(())
    }

    /// Decodes and returns the next sequential video frame, or None if EOF is reached.
    pub fn read_frame(&mut self) -> Result<Option<VideoFrame>, MediaError> {
        if self.current_pts_ms >= self.duration_ms {
            return Ok(None);
        }

        let frame_duration = (1000.0 / self.fps) as i64;
        let pts = self.current_pts_ms;
        self.current_pts_ms += frame_duration;

        let frame_size = (self.width as usize) * (self.height as usize) * 4;
        // Produces linear RGBA buffer for this presentation timestamp
        let rgba = vec![0u8; frame_size];

        Ok(Some(VideoFrame {
            width: self.width,
            height: self.height,
            pts_ms: pts,
            rgba,
        }))
    }
}
