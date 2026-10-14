use speech::{AudioBuffer, CaptionSegment, CaptionWord, VoiceProfile};

#[test]
fn test_caption_segment_serialization() {
    let segment = CaptionSegment {
        start_ms: 100,
        end_ms: 1500,
        text: "Hello Rupa".to_string(),
        words: vec![
            CaptionWord {
                start_ms: 100,
                end_ms: 600,
                word: "Hello".to_string(),
            },
            CaptionWord {
                start_ms: 650,
                end_ms: 1500,
                word: "Rupa".to_string(),
            },
        ],
    };

    let serialized = serde_json::to_string(&segment).expect("must serialize");
    assert!(serialized.contains("Hello Rupa"));

    let deserialized: CaptionSegment = serde_json::from_str(&serialized).expect("must deserialize");
    assert_eq!(deserialized, segment);
}

#[test]
fn test_audio_buffer_duration_calculation() {
    // 16,000 samples at 16,000 Hz = exactly 1,000 ms
    let buf = AudioBuffer::new_mono(16000, vec![0.0; 16000]);
    assert_eq!(buf.duration_ms(), 1000);

    // 24,000 samples at 24,000 Hz = exactly 1,000 ms
    let buf24 = AudioBuffer::new_mono(24000, vec![0.0; 48000]);
    assert_eq!(buf24.duration_ms(), 2000);
}

#[test]
fn test_voice_profile_serialization() {
    let voice = VoiceProfile {
        id: "af_heart".to_string(),
        name: "Heart".to_string(),
        language: "en-us".to_string(),
        gender: "female".to_string(),
    };

    let json = serde_json::to_string(&voice).expect("must serialize");
    assert!(json.contains("af_heart"));
}
