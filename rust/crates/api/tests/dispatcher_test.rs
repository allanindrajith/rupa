use api::commands::TimelineCommand;
use api::dispatcher::CommandDispatcher;

#[test]
fn test_dispatch_typed_commands() {
    let mut dispatcher = CommandDispatcher::new();

    // 1. Add clip
    let res = dispatcher.dispatch(TimelineCommand::AddClip {
        track_id: 1,
        source_path: "/path/video.mp4".into(),
        start_ms: 0,
        duration_ms: 5000,
    }).unwrap();
    assert!(res.success);
    assert_eq!(res.data["clip_id"], "clip_1");

    // 2. Split clip
    let split_res = dispatcher.dispatch(TimelineCommand::SplitClip {
        clip_id: "clip_1".into(),
        split_ms: 2500,
    }).unwrap();
    assert!(split_res.success);
    assert_eq!(split_res.data["new_clip_id"], "clip_2");

    // 3. Generate TTS
    let tts_res = dispatcher.dispatch(TimelineCommand::GenerateTts {
        text: "Hello world".into(),
        voice_id: "af_heart".into(),
        speed: 1.0,
    }).unwrap();
    assert!(tts_res.success);
    assert!(tts_res.data["clip_id"].as_str().unwrap().contains("tts_clip"));
}

#[test]
fn test_dispatch_json_string() {
    let mut dispatcher = CommandDispatcher::new();
    let json_cmd = r#"{
        "action": "add_clip",
        "params": {
            "track_id": 0,
            "source_path": "/test/media.mp4",
            "start_ms": 100,
            "duration_ms": 2000
        }
    }"#;

    let res_str = dispatcher.dispatch_json(json_cmd).expect("must dispatch json string");
    assert!(res_str.contains("clip_1"));
    assert!(res_str.contains("Clip added to track 0"));
}
