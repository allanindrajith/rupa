use serde::{Deserialize, Serialize};
use crate::dispatcher::CommandDispatcher;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: serde_json::Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }
}

pub struct RpcHandler {
    dispatcher: CommandDispatcher,
}

impl RpcHandler {
    pub fn new() -> Self {
        Self {
            dispatcher: CommandDispatcher::new(),
        }
    }

    /// Processes a JSON-RPC 2.0 request.
    pub fn handle(&mut self, req: &JsonRpcRequest) -> JsonRpcResponse {
        if req.jsonrpc != "2.0" {
            return JsonRpcResponse::error(req.id.clone(), -32600, "Invalid Request: expected jsonrpc 2.0");
        }

        match req.method.as_str() {
            "rupa.command" => {
                if let Some(ref p) = req.params {
                    match serde_json::from_value(p.clone()) {
                        Ok(cmd) => match self.dispatcher.dispatch(cmd) {
                            Ok(resp) => JsonRpcResponse::success(req.id.clone(), serde_json::to_value(resp).unwrap()),
                            Err(e) => JsonRpcResponse::error(req.id.clone(), -32000, e.to_string()),
                        },
                        Err(e) => JsonRpcResponse::error(req.id.clone(), -32602, format!("Invalid params: {e}")),
                    }
                } else {
                    JsonRpcResponse::error(req.id.clone(), -32602, "Missing params")
                }
            }
            "rupa.get_state" => {
                match self.dispatcher.dispatch(crate::commands::TimelineCommand::GetState) {
                    Ok(resp) => JsonRpcResponse::success(req.id.clone(), serde_json::to_value(resp).unwrap()),
                    Err(e) => JsonRpcResponse::error(req.id.clone(), -32000, e.to_string()),
                }
            }
            unknown => JsonRpcResponse::error(req.id.clone(), -32601, format!("Method not found: {unknown}")),
        }
    }
}

impl Default for RpcHandler {
    fn default() -> Self {
        Self::new()
    }
}
