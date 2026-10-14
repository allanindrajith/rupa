use api::rpc::{JsonRpcRequest, RpcHandler};

#[test]
fn test_handle_json_rpc_command() {
    let mut handler = RpcHandler::new();

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(1),
        method: "rupa.command".to_string(),
        params: Some(serde_json::json!({
            "action": "add_clip",
            "params": {
                "track_id": 1,
                "source_path": "/video.mp4",
                "start_ms": 0,
                "duration_ms": 3000
            }
        })),
    };

    let resp = handler.handle(&req);
    assert_eq!(resp.jsonrpc, "2.0");
    assert_eq!(resp.id, serde_json::json!(1));
    assert!(resp.error.is_none());
    assert!(resp.result.is_some());
}

#[test]
fn test_handle_unknown_method() {
    let mut handler = RpcHandler::new();
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: serde_json::json!(2),
        method: "rupa.non_existent".to_string(),
        params: None,
    };

    let resp = handler.handle(&req);
    assert!(resp.error.is_some());
    assert_eq!(resp.error.unwrap().code, -32601); // Method not found
}
