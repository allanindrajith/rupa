use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use crate::SpeechError;

pub struct ModelManager;

impl ModelManager {
    /// Discovers or creates the OS-standard local cache directory for speech models.
    pub fn cache_dir() -> Result<PathBuf, SpeechError> {
        let base_dir = if let Ok(custom) = std::env::var("RUPA_CACHE_DIR") {
            PathBuf::from(custom)
        } else if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".cache").join("rupa").join("models").join("speech")
        } else if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            PathBuf::from(appdata).join("rupa").join("cache").join("models").join("speech")
        } else {
            PathBuf::from(".").join(".rupa_cache").join("models").join("speech")
        };

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }

        Ok(base_dir)
    }

    /// Verifies the SHA-256 integrity checksum of a model file against an expected hex string.
    pub fn verify_checksum(path: &Path, expected_hex: &str) -> Result<bool, SpeechError> {
        if !path.exists() {
            return Err(SpeechError::ModelNotFound(path.display().to_string()));
        }

        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 65536];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        let calculated_hex = format!("{:x}", result);

        Ok(calculated_hex.eq_ignore_ascii_case(expected_hex))
    }

    /// Downloads a model file from URL, saves to target path, and verifies its checksum.
    pub fn download_model(url: &str, target_path: &Path, expected_hex: &str) -> Result<(), SpeechError> {
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let resp = ureq::get(url)
            .call()
            .map_err(|e| SpeechError::DownloadError(format!("HTTP error {e}")))?;

        let mut reader = resp.into_reader();
        let temp_path = target_path.with_extension("download");
        let mut out_file = File::create(&temp_path)?;

        let mut buffer = [0u8; 65536];
        loop {
            let bytes_read = reader.read(&mut buffer)
                .map_err(|e| SpeechError::DownloadError(format!("Read error {e}")))?;
            if bytes_read == 0 {
                break;
            }
            out_file.write_all(&buffer[..bytes_read])?;
        }
        out_file.flush()?;
        drop(out_file);

        let matches = Self::verify_checksum(&temp_path, expected_hex)?;
        if !matches {
            let _ = fs::remove_file(&temp_path);
            return Err(SpeechError::ChecksumMismatch {
                model: target_path.display().to_string(),
                expected: expected_hex.to_string(),
                actual: "mismatch".to_string(),
            });
        }

        fs::rename(&temp_path, target_path)?;
        Ok(())
    }
}
