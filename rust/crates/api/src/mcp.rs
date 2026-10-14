use serde::{Deserialize, Serialize};
use crate::commands::TimelineCommand;
use crate::dispatcher::CommandDispatcher;
use crate::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub struct McpServer {
    dispatcher: CommandDispatcher,
    tools: Vec<McpTool>,
}

impl McpServer {
    /// Creates a new MCP tool server exposing Rupa's editor capabilities to AI agents.
    pub fn new() -> Self {
        let tools = vec![
            McpTool {
                name: "rupa_add_clip".to_string(),
                description: "Adds a video or audio clip to a timeline track".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "track_id": { "type": "integer" },
                        "source_path": { "type": "string" },
                        "start_ms": { "type": "integer" },
                        "duration_ms": { "type": "integer" }
                    },
                    "required": ["track_id", "source_path", "start_ms", "duration_ms"]
                }),
            },
            McpTool {
                name: "rupa_split_clip".to_string(),
                description: "Splits a timeline clip at a given timestamp in milliseconds".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "clip_id": { "type": "string" },
                        "split_ms": { "type": "integer" }
                    },
                    "required": ["clip_id", "split_ms"]
                }),
            },
            McpTool {
                name: "rupa_generate_captions".to_string(),
                description: "Runs on-device Whisper to generate auto-caption subtitle items for an audio clip".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "audio_clip_id": { "type": "string" }
                    },
                    "required": ["audio_clip_id"]
                }),
            },
            McpTool {
                name: "rupa_generate_tts".to_string(),
                description: "Synthesizes spoken speech using on-device Kokoro TTS into an audio clip".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "text": { "type": "string" },
                        "voice_id": { "type": "string", "default": "af_heart" },
                        "speed": { "type": "number", "default": 1.0 }
                    },
                    "required": ["text"]
                }),
            },
            McpTool {
                name: "rupa_remove_background".to_string(),
                description: "Runs on-device AI person segmentation and auto-cutout mask on a video clip".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "video_clip_id": { "type": "string" }
                    },
                    "required": ["video_clip_id"]
                }),
            },
            McpTool {
                name: "rupa_export_video".to_string(),
                description: "Exports the current timeline into an MP4/WebM video file".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "output_path": { "type": "string" },
                        "format": { "type": "string", "default": "mp4" },
                        "resolution": { "type": "string", "default": "1080p" }
                    },
                    "required": ["output_path"]
                }),
            },
        ];

        Self {
            dispatcher: CommandDispatcher::new(),
            tools,
        }
    }

    /// Lists all tools available through the Model Context Protocol.
    pub fn list_tools(&self) -> &[McpTool] {
        &self.tools
    }

    /// Invokes an MCP tool by name with JSON arguments.
    pub fn call_tool(&mut self, name: &str, arguments: serde_json::Value) -> Result<serde_json::Value, ApiError> {
        let cmd = match name {
            "rupa_add_clip" => {
                let track_id = arguments["track_id"].as_u64().unwrap_or(0) as u32;
                let source_path = arguments["source_path"].as_str().unwrap_or("").to_string();
                let start_ms = arguments["start_ms"].as_i64().unwrap_or(0);
                let duration_ms = arguments["duration_ms"].as_i64().unwrap_or(1000);
                TimelineCommand::AddClip { track_id, source_path, start_ms, duration_ms }
            }
            "rupa_split_clip" => {
                let clip_id = arguments["clip_id"].as_str().unwrap_or("").to_string();
                let split_ms = arguments["split_ms"].as_i64().unwrap_or(0);
                TimelineCommand::SplitClip { clip_id, split_ms }
            }
            "rupa_generate_captions" => {
                let audio_clip_id = arguments["audio_clip_id"].as_str().unwrap_or("").to_string();
                TimelineCommand::GenerateCaptions { audio_clip_id }
            }
            "rupa_generate_tts" => {
                let text = arguments["text"].as_str().unwrap_or("").to_string();
                let voice_id = arguments["voice_id"].as_str().unwrap_or("af_heart").to_string();
                let speed = arguments["speed"].as_f64().unwrap_or(1.0) as f32;
                TimelineCommand::GenerateTts { text, voice_id, speed }
            }
            "rupa_remove_background" => {
                let video_clip_id = arguments["video_clip_id"].as_str().unwrap_or("").to_string();
                TimelineCommand::RemoveBackground { video_clip_id }
            }
            "rupa_export_video" => {
                let output_path = arguments["output_path"].as_str().unwrap_or("output.mp4").to_string();
                let format = arguments["format"].as_str().unwrap_or("mp4").to_string();
                let resolution = arguments["resolution"].as_str().unwrap_or("1080p").to_string();
                TimelineCommand::ExportVideo { output_path, format, resolution }
            }
            unknown => return Err(ApiError::UnknownMethod(format!("Unknown tool: {unknown}"))),
        };

        let response = self.dispatcher.dispatch(cmd)?;
        Ok(serde_json::to_value(response)?)
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}
