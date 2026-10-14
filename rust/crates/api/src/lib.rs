//! Rupa Automation API & Model Context Protocol (MCP) Server
//!
//! Provides a headless command dispatcher, JSON-RPC protocol parser, and MCP tool
//! server enabling AI agents and external scripts to automate timeline editing.

pub mod commands;
pub mod dispatcher;
pub mod mcp;
pub mod rpc;

pub use commands::TimelineCommand;
pub use dispatcher::{CommandDispatcher, CommandResponse};
pub use mcp::McpServer;
pub use rpc::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

/// Errors surfaced by the API dispatcher and MCP server.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Unknown command or method: {0}")]
    UnknownMethod(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Execution failure: {0}")]
    ExecutionFailure(String),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
