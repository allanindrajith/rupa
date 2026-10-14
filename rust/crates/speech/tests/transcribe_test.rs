use speech::transcribe::WhisperTranscriber;

#[test]
fn test_missing_model_returns_model_not_found() {
    let result = WhisperTranscriber::new("/path/that/does/not/exist/ggml-tiny.bin");
    assert!(result.is_err());
}

#[test]
fn test_invalid_sample_rate_rejected() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let transcriber = WhisperTranscriber::new(temp.path()).unwrap();

    let samples = vec![0.0f32; 1000];
    // 44.1kHz instead of 16kHz
    let res = transcriber.transcribe(&samples, 44100);
    assert!(res.is_err(), "must reject non-16kHz sample rate");
}

#[test]
fn test_empty_audio_returns_empty_captions() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let transcriber = WhisperTranscriber::new(temp.path()).unwrap();

    let res = transcriber.transcribe(&[], 16000).unwrap();
    assert!(res.is_empty(), "empty audio must yield empty caption list");
}

#[test]
fn test_silence_returns_empty_captions() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let transcriber = WhisperTranscriber::new(temp.path()).unwrap();

    let silence = vec![0.0f32; 16000];
    let res = transcriber.transcribe(&silence, 16000).unwrap();
    assert!(res.is_empty(), "silence audio must yield empty caption list");
}
