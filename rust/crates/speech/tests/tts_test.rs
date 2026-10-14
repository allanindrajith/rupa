use speech::tts::KokoroTts;

#[test]
fn test_missing_model_dir_returns_error() {
    let result = KokoroTts::new("/path/that/does/not/exist");
    assert!(result.is_err());
}

#[test]
fn test_list_voices() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tts = KokoroTts::new(temp_dir.path()).unwrap();

    let voices = tts.list_voices();
    assert!(!voices.is_empty(), "must list bundled voice profiles");
    assert!(voices.iter().any(|v| v.id == "af_heart"));
}

#[test]
fn test_speed_bounds_validation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tts = KokoroTts::new(temp_dir.path()).unwrap();

    // Too slow (< 0.5)
    let too_slow = tts.synthesize("Hello", "af_heart", 0.2);
    assert!(too_slow.is_err());

    // Too fast (> 2.0)
    let too_fast = tts.synthesize("Hello", "af_heart", 3.0);
    assert!(too_fast.is_err());
}

#[test]
fn test_unknown_voice_rejected() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tts = KokoroTts::new(temp_dir.path()).unwrap();

    let res = tts.synthesize("Hello", "non_existent_voice", 1.0);
    assert!(res.is_err());
}

#[test]
fn test_empty_text_returns_empty_audio() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tts = KokoroTts::new(temp_dir.path()).unwrap();

    let res = tts.synthesize("   ", "af_heart", 1.0).unwrap();
    assert!(res.samples.is_empty());
    assert_eq!(res.sample_rate, 24000);
}

#[test]
fn test_synthesize_valid_text() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tts = KokoroTts::new(temp_dir.path()).unwrap();

    let res = tts.synthesize("Hello Rupa", "af_heart", 1.0).unwrap();
    assert!(!res.samples.is_empty());
    assert_eq!(res.sample_rate, 24000);
    assert_eq!(res.channels, 1);
}
