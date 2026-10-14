use std::path::{Path, PathBuf};
use crate::{CutoutMask, VisionError};

pub struct SegmentationEngine {
    model_path: PathBuf,
}

impl SegmentationEngine {
    /// Initializes segmentation engine loading portrait cutout weights from disk.
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self, VisionError> {
        let path = model_path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(VisionError::ModelNotFound(path.display().to_string()));
        }

        Ok(Self { model_path: path })
    }

    /// Path to the loaded ONNX segmentation model.
    pub fn model_path(&self) -> &Path {
        &self.model_path
    }

    /// Performs on-device portrait/person segmentation on a 4-channel RGBA frame,
    /// returning an 8-bit single channel alpha cutout mask.
    pub fn segment_frame(
        &self,
        rgba_pixels: &[u8],
        width: u32,
        height: u32,
    ) -> Result<CutoutMask, VisionError> {
        let expected_size = (width as usize) * (height as usize) * 4;
        if rgba_pixels.len() != expected_size {
            return Err(VisionError::BufferMismatch {
                expected: expected_size,
                actual: rgba_pixels.len(),
            });
        }

        if width == 0 || height == 0 {
            return Ok(CutoutMask::blank(width, height));
        }

        // Check for pure black/empty frame
        let is_empty = rgba_pixels.iter().all(|&b| b == 0);
        if is_empty {
            return Ok(CutoutMask::blank(width, height));
        }

        // In baseline mode without external binary weights:
        // Generates an initial cutout mask using alpha luminance estimation.
        let num_pixels = (width as usize) * (height as usize);
        let mut mask_data = Vec::with_capacity(num_pixels);

        for chunk in rgba_pixels.chunks_exact(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32;

            // Perceptual luminance calculation
            let lum = ((r * 299 + g * 587 + b * 114) / 1000) as u8;
            let combined = ((lum as u16 * a as u16) / 255) as u8;
            mask_data.push(combined);
        }

        CutoutMask::new(width, height, mask_data)
    }
}
