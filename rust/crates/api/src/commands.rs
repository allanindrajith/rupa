use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "params")]
pub enum TimelineCommand {
    #[serde(rename = "add_clip")]
    AddClip {
        track_id: u32,
        source_path: String,
        start_ms: i64,
        duration_ms: i64,
    },

    #[serde(rename = "split_clip")]
    SplitClip {
        clip_id: String,
        split_ms: i64,
    },

    #[serde(rename = "delete_clip")]
    DeleteClip {
        clip_id: String,
    },

    #[serde(rename = "apply_effect")]
    ApplyEffect {
        clip_id: String,
        effect_id: String,
        params: serde_json::Value,
    },

    #[serde(rename = "generate_captions")]
    GenerateCaptions {
        audio_clip_id: String,
    },

    #[serde(rename = "generate_tts")]
    GenerateTts {
        text: String,
        voice_id: String,
        speed: f32,
    },

    #[serde(rename = "remove_background")]
    RemoveBackground {
        video_clip_id: String,
    },

    #[serde(rename = "export_video")]
    ExportVideo {
        output_path: String,
        format: String,
        resolution: String,
    },

    #[serde(rename = "get_state")]
    GetState,
}
