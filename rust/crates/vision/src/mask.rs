use serde::{Deserialize, Serialize};
use crate::VisionError;

/// An 8-bit single-channel grayscale cutout mask (0 = fully removed/transparent, 255 = fully visible).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CutoutMask {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl CutoutMask {
    /// Creates a new mask verifying buffer size matches width * height.
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Result<Self, VisionError> {
        let expected = (width as usize) * (height as usize);
        if data.len() != expected {
            return Err(VisionError::BufferMismatch {
                expected,
                actual: data.len(),
            });
        }

        Ok(Self { width, height, data })
    }

    /// Creates an empty (all 0 / fully transparent) mask.
    pub fn blank(width: u32, height: u32) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            data: vec![0; size],
        }
    }

    /// Creates a full (all 255 / fully opaque) mask.
    pub fn full(width: u32, height: u32) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            data: vec![255; size],
        }
    }

    /// Inverts the mask in-place (foreground becomes background and vice-versa).
    pub fn invert(&mut self) {
        for pixel in &mut self.data {
            *pixel = 255 - *pixel;
        }
    }

    /// Applies a hard binary threshold to soft alpha mask values.
    pub fn threshold(&mut self, cutoff: u8) {
        for pixel in &mut self.data {
            *pixel = if *pixel >= cutoff { 255 } else { 0 };
        }
    }

    /// Multiplies the alpha channel of an existing 4-channel RGBA frame buffer by this mask.
    pub fn apply_to_rgba(&self, rgba: &mut [u8]) -> Result<(), VisionError> {
        let expected_len = (self.width as usize) * (self.height as usize) * 4;
        if rgba.len() != expected_len {
            return Err(VisionError::BufferMismatch {
                expected: expected_len,
                actual: rgba.len(),
            });
        }

        for (i, &mask_val) in self.data.iter().enumerate() {
            let alpha_idx = (i * 4) + 3;
            let current_alpha = rgba[alpha_idx] as u16;
            let new_alpha = ((current_alpha * (mask_val as u16)) / 255) as u8;
            rgba[alpha_idx] = new_alpha;
        }

        Ok(())
    }

    /// Serializes the 8-bit mask into a standard PNG byte array.
    pub fn to_png_bytes(&self) -> Result<Vec<u8>, VisionError> {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, self.width, self.height);
            encoder.set_color(png::ColorType::Grayscale);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder
                .write_header()
                .map_err(|e| VisionError::PngError(e.to_string()))?;
            writer
                .write_image_data(&self.data)
                .map_err(|e| VisionError::PngError(e.to_string()))?;
        }
        Ok(bytes)
    }

    /// Parses an 8-bit grayscale PNG byte array into a CutoutMask.
    pub fn from_png_bytes(bytes: &[u8]) -> Result<Self, VisionError> {
        let decoder = png::Decoder::new(bytes);
        let mut reader = decoder
            .read_info()
            .map_err(|e| VisionError::PngError(e.to_string()))?;

        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader
            .next_frame(&mut buf)
            .map_err(|e| VisionError::PngError(e.to_string()))?;

        let width = info.width;
        let height = info.height;
        let expected_size = (width as usize) * (height as usize);

        let data = match info.color_type {
            png::ColorType::Grayscale => {
                buf.truncate(expected_size);
                buf
            }
            png::ColorType::Rgba => {
                // Extract alpha or luminance
                let mut gray = Vec::with_capacity(expected_size);
                for chunk in buf.chunks_exact(4) {
                    gray.push(chunk[3]); // Use alpha channel
                }
                gray
            }
            _ => {
                return Err(VisionError::PngError(
                    "Unsupported PNG format; expected Grayscale or RGBA".to_string(),
                ));
            }
        };

        Self::new(width, height, data)
    }
}
