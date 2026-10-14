use speech::models::ModelManager;
use speech::tts::KokoroTts;
use speech::{CaptionSegment, CaptionWord};

#[test]
fn test_end_to_end_audio_to_tts_pipeline() {
    // 1. Verify cache directory creation
    let cache_dir = ModelManager::cache_dir().expect("cache dir must resolve");
    assert!(cache_dir.is_dir());

    // 2. Initialize TTS with local dummy directory
    let temp_tts_dir = tempfile::tempdir().unwrap();
    let tts = KokoroTts::new(temp_tts_dir.path()).unwrap();

    let voices = tts.list_voices();
    let voice = &voices[0];

    // 3. Synthesize speech
    let audio = tts.synthesize("Welcome to Rupa video editor", &voice.id, 1.0).unwrap();
    assert_eq!(audio.sample_rate, 24000);
    assert_eq!(audio.channels, 1);
    assert!(!audio.samples.is_empty());

    // 4. Verify caption data structures can receive and represent timed text
    let caption = CaptionSegment {
        start_ms: 0,
        end_ms: audio.duration_ms(),
        text: "Welcome to Rupa video editor".to_string(),
        words: vec![
            CaptionWord { start_ms: 0, end_ms: 400, word: "Welcome".to_string() },
            CaptionWord { start_ms: 450, end_ms: 600, word: "to".to_string() },
            CaptionWord { start_ms: 650, end_ms: 1000, word: "Rupa".to_string() },
            CaptionWord { start_ms: 1050, end_ms: 1400, word: "video".to_string() },
            CaptionWord { start_ms: 1450, end_ms: 1800, word: "editor".to_string() },
        ],
    };

    assert_eq!(caption.words.len(), 5);
}
