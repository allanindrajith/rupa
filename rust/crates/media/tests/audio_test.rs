use media::audio::{AudioMixer, AudioTrackClip};

#[test]
fn test_mix_timeline_summing_and_gain() {
    let clip1 = AudioTrackClip {
        start_ms: 0,
        duration_ms: 100,
        volume: 1.0,
        samples: vec![0.5; 48], // 48 samples
    };

    let clip2 = AudioTrackClip {
        start_ms: 0,
        duration_ms: 100,
        volume: 0.5,
        samples: vec![0.4; 48],
    };

    // 480 samples at 48000 Hz = 10ms
    let mixed = AudioMixer::mix_timeline(&[clip1, clip2], 10, 48000).expect("mix must succeed");
    assert_eq!(mixed.len(), 480);

    // Initial 48 samples should be 0.5 * 1.0 + 0.4 * 0.5 = 0.7
    for s in &mixed[..48] {
        assert!((s - 0.7).abs() < 1e-4);
    }

    // Remaining samples are zero
    for s in &mixed[48..] {
        assert_eq!(*s, 0.0);
    }
}

#[test]
fn test_mix_timeline_clamping() {
    let clip1 = AudioTrackClip {
        start_ms: 0,
        duration_ms: 10,
        volume: 1.0,
        samples: vec![0.8; 48],
    };

    let clip2 = AudioTrackClip {
        start_ms: 0,
        duration_ms: 10,
        volume: 1.0,
        samples: vec![0.8; 48],
    };

    let mixed = AudioMixer::mix_timeline(&[clip1, clip2], 10, 48000).unwrap();
    // 0.8 + 0.8 = 1.6 clamped to 1.0
    for s in &mixed[..48] {
        assert_eq!(*s, 1.0);
    }
}
