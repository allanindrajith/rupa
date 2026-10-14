use serde::{Deserialize, Serialize};
use crate::MediaError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackClip {
    pub start_ms: i64,
    pub duration_ms: i64,
    pub volume: f32,
    pub samples: Vec<f32>,
}

pub struct AudioMixer;

impl AudioMixer {
    /// Mixes multiple timeline audio clips into a unified linear single-channel float buffer,
    /// clamping final amplitude to [-1.0, 1.0].
    pub fn mix_timeline(
        clips: &[AudioTrackClip],
        output_duration_ms: i64,
        sample_rate: u32,
    ) -> Result<Vec<f32>, MediaError> {
        if output_duration_ms < 0 {
            return Err(MediaError::InvalidParameter("output_duration_ms must be >= 0".into()));
        }

        let total_samples = ((output_duration_ms as f64 / 1000.0) * sample_rate as f64).round() as usize;
        let mut mixed = vec![0.0f32; total_samples];

        for clip in clips {
            let clip_start_sample = ((clip.start_ms as f64 / 1000.0) * sample_rate as f64).round() as usize;
            for (idx, &sample) in clip.samples.iter().enumerate() {
                let target_idx = clip_start_sample + idx;
                if target_idx < total_samples {
                    mixed[target_idx] += sample * clip.volume;
                }
            }
        }

        // Clamp mixed amplitude to prevent digital clipping/distortion
        for s in &mut mixed {
            *s = s.clamp(-1.0, 1.0);
        }

        Ok(mixed)
    }
}
