use api::mcp::McpServer;

#[test]
fn test_mcp_list_tools() {
    let server = McpServer::new();
    let tools = server.list_tools();
    assert!(tools.len() >= 6, "must list core tools");

    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(tool_names.contains(&"rupa_add_clip"));
    assert!(tool_names.contains(&"rupa_split_clip"));
    assert!(tool_names.contains(&"rupa_generate_captions"));
    assert!(tool_names.contains(&"rupa_generate_tts"));
    assert!(tool_names.contains(&"rupa_remove_background"));
    assert!(tool_names.contains(&"rupa_export_video"));
}

#[test]
fn test_mcp_call_tool() {
    let mut server = McpServer::new();

    // Call rupa_add_clip
    let res = server.call_tool("rupa_add_clip", serde_json::json!({
        "track_id": 0,
        "source_path": "/path/sample.mov",
        "start_ms": 500,
        "duration_ms": 4000
    })).expect("tool call must succeed");

    assert_eq!(res["success"], true);
    assert!(res["data"]["clip_id"].as_str().unwrap().contains("clip_"));

    // Call rupa_generate_tts
    let tts_res = server.call_tool("rupa_generate_tts", serde_json::json!({
        "text": "Subtitles test",
        "voice_id": "af_heart"
    })).expect("tts tool call must succeed");

    assert_eq!(tts_res["success"], true);
}
