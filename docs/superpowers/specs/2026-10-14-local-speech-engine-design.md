# Architecture Design: Rupa On-Device Speech Engine (`rust/crates/speech`)

**Date**: 2026-10-14  
**Status**: Proposed  
**Author**: Antigravity  
**Target**: `rust/crates/speech`  

---

## 1. Overview & Motivation

Rupa is migrating business and media logic into `rust/` to support both native desktop (`apps/desktop` via GPUI) and web (`apps/web` via Next.js and WASM/local bridge). 

Currently, caption generation and text-to-speech (TTS) either depend on external cloud APIs (e.g. ElevenLabs, OpenAI) or browser-native speech synthesis, causing privacy concerns, latency, and subscription requirements.

Inspired by the local-first architecture in [`jub0t/concat`](https://github.com/jub0t/concat.git), this design specifies Rupa's own clean-room, MIT-licensed Rust speech crate: `rust/crates/speech`. It provides:
1. **On-Device Auto-Captions**: Whisper speech-to-text with Apple Silicon Metal acceleration and CPU fallbacks.
2. **On-Device Text-to-Speech (TTS)**: High-quality natural voice synthesis via Kokoro using `sherpa-onnx`.
3. **Model Management**: Automatic downloading, caching (`~/.cache/rupa/models`), and SHA-256 checksum verification.

---

## 2. Crate Architecture

```
rust/crates/speech/
├── Cargo.toml
└── src/
    ├── lib.rs          # Public facade, traits, types, error handling
    ├── transcribe.rs   # whisper-rs Whisper transcription pipeline
    ├── tts.rs          # sherpa-onnx Kokoro TTS voice generation
    └── models.rs       # Model cache discovery, downloading, and checksum validation
```

### 2.1 Dependencies (`rust/crates/speech/Cargo.toml`)

```toml
[package]
name = "speech"
version = "0.1.0"
edition = "2024"
license = "MIT"
description = "On-device transcription (Whisper) and speech synthesis (Kokoro) for Rupa."

[dependencies]
log = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
ureq = "2.10"
sha2 = "0.10"

# Transcription: whisper.cpp bindings
whisper-rs = "0.16"

# Text-to-speech: sherpa-onnx bindings for Kokoro
[target.'cfg(not(any(windows, target_os = "android", target_os = "ios")))'.dependencies]
sherpa-onnx = "1.13"

[target.'cfg(any(windows, target_os = "android", target_os = "ios"))'.dependencies]
sherpa-onnx = { version = "1.13", default-features = false, features = ["shared"] }

[target.'cfg(target_os = "macos")'.dependencies]
whisper-rs = { version = "0.16", features = ["metal"] }
```

---

## 3. Data Structures & Interfaces (`src/lib.rs`)

### 3.1 Public Types

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionSegment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
    pub words: Vec<CaptionWord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptionWord {
    pub start_ms: i64,
    pub end_ms: i64,
    pub word: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBuffer {
    pub sample_rate: u32,
    pub channels: u16,
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceProfile {
    pub id: String,
    pub name: String,
    pub language: String,
    pub gender: String,
}
```

### 3.2 Error Types

```rust
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
}
```

---

## 4. Subsystems Detail

### 4.1 Whisper Transcription Pipeline (`src/transcribe.rs`)

1. **Input Requirements**: 16,000 Hz, 1-channel (mono) 32-bit floating-point PCM samples (`&[f32]`).
2. **Context Creation**:
   - Loads `ggml-tiny.bin` (39MB) for ultra-fast processing or `ggml-base.bin` (142MB) for higher accuracy.
   - Enables Metal hardware acceleration on macOS automatically via `whisper-rs`.
3. **Execution**:
   - Sets sampling strategy (Greedy or BeamSearch).
   - Iterates through segments and word-level timestamps (`whisper_full_n_segments`, `whisper_full_n_tokens`).
   - Produces `Vec<CaptionSegment>`.

### 4.2 Kokoro TTS Synthesis Pipeline (`src/tts.rs`)

1. **Model**: Kokoro v0.19 multi-lingual model running through `sherpa-onnx`.
2. **Features**:
   - Generates 24,000 Hz high-fidelity mono audio.
   - Built-in voices: American English (female/male), British English, and multi-lingual voices.
   - Speed control: 0.5x to 2.0x.
3. **Output**: `AudioBuffer` ready to be appended to Rupa's timeline or played back directly.

### 4.3 Model Cache & Downloader (`src/models.rs`)

1. **Cache Directory**:
   - macOS: `~/Library/Caches/rupa/models/speech`
   - Linux: `~/.cache/rupa/models/speech`
   - Windows: `%LOCALAPPDATA%\rupa\cache\models\speech`
2. **Verification**:
   - Hardcoded SHA-256 table for model binaries (`whisper-tiny.bin`, `kokoro-v0_19.onnx`, `voices.bin`).
   - Re-verifies on load; automatically redownloads corrupt files.

---

## 5. Timeline & App Integration

1. **Desktop App (`apps/desktop`)**:
   - Directly imports `speech` crate.
   - Spawns background worker thread (`std::thread` / task pool) for long running transcribes and TTS generation, keeping UI 60fps responsive.
2. **Timeline Insertion**:
   - `CaptionSegment` objects map directly to Rupa text subtitle track items with matching `start_ms` and `duration_ms`.
   - `AudioBuffer` saves as WAV in project assets cache and inserts as an audio clip.
3. **Web Fallback (`apps/web`)**:
   - If running as local PWA connected to Rupa local service: invokes native speech endpoint.
   - If standalone browser without local host: gracefully surfaces Web Speech API.

---

## 6. Verification & Test Plan

1. **Unit Tests**:
   - Model download and SHA-256 validation test against small test fixture.
   - PCM resampler validation (testing 44.1kHz / 48kHz downsampling to 16kHz).
2. **Integration Tests**:
   - Synthetic 1-second sine wave transcription test (expects no crash, valid empty or noise segment).
   - TTS synthesis test: synthesize `"Hello Rupa"` and verify `AudioBuffer` contains non-zero 24kHz samples.
