use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectCategory {
    Color,
    Distort,
    Blur,
    Stylize,
    Transition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParamType {
    Float,
    Int,
    Vec2,
    Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectParam {
    pub key: String,
    pub label: String,
    pub param_type: ParamType,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectManifest {
    pub id: String,
    pub name: String,
    pub category: EffectCategory,
    pub description: String,
    pub params: Vec<EffectParam>,
    pub wgsl: String,
}

impl EffectManifest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        category: EffectCategory,
        description: impl Into<String>,
        params: Vec<EffectParam>,
        wgsl: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category,
            description: description.into(),
            params,
            wgsl: wgsl.into(),
        }
    }
}
