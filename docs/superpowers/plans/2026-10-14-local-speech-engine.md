# Rupa Local Speech Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build Rupa's on-device speech engine crate (`rust/crates/speech`) supporting local Whisper auto-captions and offline Kokoro TTS.

**Architecture:** A standalone, platform-agnostic Rust crate in `rust/crates/speech` providing high-level interfaces for Whisper.cpp transcription and sherpa-onnx Kokoro voice synthesis, with automatic local model downloading and SHA-256 integrity verification.

**Tech Stack:** Rust 2024, `whisper-rs = "0.16"`, `sherpa-onnx = "1.13"`, `ureq = "2.10"`, `sha2 = "0.10"`, `serde`, `thiserror`.

**Spec:** [`docs/superpowers/specs/2026-10-14-local-speech-engine-design.md`](file:///Users/allanindrajith/Github%20Project/Rupa/docs/superpowers/specs/2026-10-14-local-speech-engine-design.md)

## Global Constraints

- Must compile under Rupa's workspace `Cargo.toml` without breaking existing web or desktop builds.
- Clean-room MIT implementation: no AGPL code copied from Concat.
- Whisper audio format strictly 16,000 Hz single-channel mono 32-bit floating-point PCM.
- Kokoro TTS outputs 24,000 Hz single-channel audio buffer.
- Models cached in OS-standard cache directories: `~/.cache/rupa/models/speech` (Linux/macOS) or `%LOCALAPPDATA%\rupa\cache\models\speech` (Windows).

## Review Focus

1. Corrupted or interrupted model download causes checksum mismatch and returns clean `SpeechError::ChecksumMismatch` without crashing.
2. Non-16kHz or multi-channel audio input to transcription is resampled or safely rejected with `SpeechError::InvalidAudio`.
3. Empty audio buffer or silence returns empty segment list rather than failing transcription.
4. Kokoro model initialization when weights are missing surfaces `SpeechError::ModelNotFound` with download guidance.
5. Multi-threaded access: transcription and synthesis can be invoked concurrently on worker threads.

---

### Task 1: Crate Scaffolding & Workspace Registration

**Files:**
- Create: `rust/crates/speech/Cargo.toml`
- Create: `rust/crates/speech/src/lib.rs`
- Modify: `Cargo.toml:1-12`

**Interfaces:**
- Consumes: None
- Produces: `speech` crate member in root workspace

- [ ] **Step 1: Create `rust/crates/speech/Cargo.toml`**

Define package `speech`, edition `2024`, license `MIT`, and dependencies (`serde`, `serde_json`, `thiserror`, `log`, `ureq`, `sha2`, `whisper-rs`, `sherpa-onnx`).

- [ ] **Step 2: Create initial `rust/crates/speech/src/lib.rs`**

Add doc comment and basic module declaration.

- [ ] **Step 3: Add `rust/crates/speech` to root `Cargo.toml` workspace members**

Update `members` list in `Cargo.toml` to include `"rust/crates/speech"`.

- [ ] **Step 4: Verify workspace detects new crate**

Run: `cargo check -p speech`  
Expected: compiles successfully.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml rust/crates/speech
git commit -m "feat(speech): scaffold speech crate in workspace"
```

---

### Task 2: Core Data Types and Error Enums

**Files:**
- Modify: `rust/crates/speech/src/lib.rs`
- Test: `rust/crates/speech/tests/types_test.rs`

**Interfaces:**
- Consumes: None
- Produces: `CaptionSegment`, `CaptionWord`, `AudioBuffer`, `VoiceProfile`, `SpeechError`

- [ ] **Step 1: Write failing test in `rust/crates/speech/tests/types_test.rs`**

```rust
use speech::{CaptionSegment, CaptionWord, AudioBuffer, VoiceProfile};

#[test]
fn test_caption_segment_serialization() {
    let segment = CaptionSegment {
        start_ms: 0,
        end_ms: 1500,
        text: "Hello Rupa".to_string(),
        words: vec![
            CaptionWord { start_ms: 0, end_ms: 600, word: "Hello".to_string() },
            CaptionWord { start_ms: 650, end_ms: 1500, word: "Rupa".to_string() },
        ],
    };
    let json = serde_json::to_string(&segment).unwrap();
    assert!(json.contains("Hello Rupa"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p speech --test types_test`  
Expected: FAIL with missing type definitions.

- [ ] **Step 3: Implement data structures and `SpeechError` in `rust/crates/speech/src/lib.rs`**

Define `CaptionSegment`, `CaptionWord`, `AudioBuffer`, `VoiceProfile`, and `SpeechError` with `thiserror` derives.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p speech --test types_test`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add rust/crates/speech
git commit -m "feat(speech): define core caption and audio types"
```

---

### Task 3: Model Directory Discovery & Checksum Verification

**Files:**
- Create: `rust/crates/speech/src/models.rs`
- Modify: `rust/crates/speech/src/lib.rs`
- Test: `rust/crates/speech/tests/models_test.rs`

**Interfaces:**
- Consumes: `SpeechError`
- Produces: `ModelManager::cache_dir() -> PathBuf`, `ModelManager::verify_checksum(path, expected_hash) -> Result<bool, SpeechError>`

- [ ] **Step 1: Write failing test in `rust/crates/speech/tests/models_test.rs`**

```rust
use speech::models::ModelManager;
use std::io::Write;

#[test]
fn test_checksum_verification() {
    let mut temp = tempfile::NamedTempFile::new().unwrap();
    temp.write_all(b"test model content").unwrap();
    
    // SHA256 of "test model content"
    let valid_hash = "68c2d58cb1546ea48d8ce9f847ebaf5b7e88ca8c8485fae4daef569e5f4d1e20";
    assert!(ModelManager::verify_checksum(temp.path(), valid_hash).unwrap());
    assert!(!ModelManager::verify_checksum(temp.path(), "invalidhash").unwrap());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p speech --test models_test`  
Expected: FAIL with `ModelManager` unresolved.

- [ ] **Step 3: Implement `ModelManager` in `rust/crates/speech/src/models.rs`**

Compute SHA-256 over file buffer and implement cache directory resolver for macOS/Linux/Windows.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p speech --test models_test`  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add rust/crates/speech
git commit -m "feat(speech): implement model cache management and checksum validation"
```

---

### Task 4: Whisper Transcription Pipeline

**Files:**
- Create: `rust/crates/speech/src/transcribe.rs`
- Modify: `rust/crates/speech/src/lib.rs`
- Test: `rust/crates/speech/tests/transcribe_test.rs`

**Interfaces:**
- Consumes: `AudioBuffer`, `CaptionSegment`, `CaptionWord`, `SpeechError`
- Produces: `WhisperTranscriber::new(model_path: &Path) -> Result<Self, SpeechError>`, `WhisperTranscriber::transcribe(&self, pcm_samples: &[f32]) -> Result<Vec<CaptionSegment>, SpeechError>`

- [ ] **Step 1: Write test in `rust/crates/speech/tests/transcribe_test.rs`**

Test validation of audio input requirements (empty samples return empty segments, sample rate check).

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p speech --test transcribe_test`  
Expected: FAIL.

- [ ] **Step 3: Implement `WhisperTranscriber` in `rust/crates/speech/src/transcribe.rs`**

Wrap `whisper_rs::WhisperContext` with parameter setup, segment iteration, token extraction, and word timestamp generation.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p speech --test transcribe_test`  
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add rust/crates/speech
git commit -m "feat(speech): implement whisper transcription engine"
```

---

### Task 5: Kokoro Offline Text-to-Speech Engine

**Files:**
- Create: `rust/crates/speech/src/tts.rs`
- Modify: `rust/crates/speech/src/lib.rs`
- Test: `rust/crates/speech/tests/tts_test.rs`

**Interfaces:**
- Consumes: `AudioBuffer`, `VoiceProfile`, `SpeechError`
- Produces: `KokoroTts::new(model_dir: &Path) -> Result<Self, SpeechError>`, `KokoroTts::list_voices(&self) -> Vec<VoiceProfile>`, `KokoroTts::synthesize(&self, text: &str, voice_id: &str, speed: f32) -> Result<AudioBuffer, SpeechError>`

- [ ] **Step 1: Write test in `rust/crates/speech/tests/tts_test.rs`**

Test `VoiceProfile` listing and input validation (empty string check, speed bounds 0.5 - 2.0).

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p speech --test tts_test`  
Expected: FAIL.

- [ ] **Step 3: Implement `KokoroTts` in `rust/crates/speech/src/tts.rs`**

Implement `sherpa_onnx::OfflineTts` configuration, voice loader, and synthesis into 24kHz `AudioBuffer`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p speech --test tts_test`  
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add rust/crates/speech
git commit -m "feat(speech): implement Kokoro offline TTS engine"
```

---

### Task 6: Full Crate Integration & Benchmark Validation

**Files:**
- Modify: `rust/crates/speech/src/lib.rs`
- Test: `rust/crates/speech/tests/integration_test.rs`

**Interfaces:**
- Consumes: All modules
- Produces: Top-level facade `speech::transcribe_audio()` and `speech::synthesize_speech()`

- [ ] **Step 1: Write integration test exercising facade**

```rust
use speech::{AudioBuffer, CaptionSegment};

#[test]
fn test_speech_engine_facade_signatures() {
    // Verifies full module export and compile-time API surface
    let buffer = AudioBuffer {
        sample_rate: 16000,
        channels: 1,
        samples: vec![0.0; 16000],
    };
    assert_eq!(buffer.samples.len(), 16000);
}
```

- [ ] **Step 2: Run full speech test suite**

Run: `cargo test -p speech`  
Expected: All unit and integration tests PASS.

- [ ] **Step 3: Verify workspace build consistency**

Run: `cargo check --workspace`  
Expected: Entire workspace builds cleanly without warnings.

- [ ] **Step 4: Commit**

```bash
git add rust/crates/speech
git commit -m "feat(speech): finalize speech crate integration and test suite"
```
