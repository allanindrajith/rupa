use crate::manifest::{EffectCategory, EffectManifest, EffectParam, ParamType};

/// In-memory catalog of built-in shader effects and presets.
pub struct EffectCatalogue {
    manifests: Vec<EffectManifest>,
}

impl EffectCatalogue {
    /// Initializes the catalog with built-in CapCut-grade WGSL effect packages.
    pub fn new() -> Self {
        let mut manifests = Vec::new();

        // 1. Color Adjustment
        manifests.push(EffectManifest::new(
            "rupa.adjust",
            "Basic Adjustments",
            EffectCategory::Color,
            "Fine-tune brightness, contrast, saturation, and color temperature.",
            vec![
                EffectParam { key: "brightness".into(), label: "Brightness".into(), param_type: ParamType::Float, min: -1.0, max: 1.0, default: 0.0, unit: None },
                EffectParam { key: "contrast".into(), label: "Contrast".into(), param_type: ParamType::Float, min: 0.0, max: 2.0, default: 1.0, unit: None },
                EffectParam { key: "saturation".into(), label: "Saturation".into(), param_type: ParamType::Float, min: 0.0, max: 2.0, default: 1.0, unit: None },
                EffectParam { key: "temperature".into(), label: "Temperature".into(), param_type: ParamType::Float, min: -1.0, max: 1.0, default: 0.0, unit: None },
            ],
            r#"
struct Params { brightness: f32, contrast: f32, saturation: f32, temperature: f32 }
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var c = textureSample(t_diffuse, s_diffuse, uv);
    c = vec4<f32>(c.rgb + vec3<f32>(params.brightness), c.a);
    c = vec4<f32>((c.rgb - 0.5) * params.contrast + 0.5, c.a);
    let gray = dot(c.rgb, vec3<f32>(0.299, 0.587, 0.114));
    c = vec4<f32>(mix(vec3<f32>(gray), c.rgb, params.saturation), c.a);
    return c;
}
            "#.trim(),
        ));

        // 2. Chromatic Aberration
        manifests.push(EffectManifest::new(
            "rupa.chromatic-aberration",
            "Chromatic Aberration",
            EffectCategory::Distort,
            "Separates red and blue channels toward the screen edges.",
            vec![
                EffectParam { key: "shift".into(), label: "Shift".into(), param_type: ParamType::Float, min: 0.0, max: 30.0, default: 5.0, unit: Some("px".into()) },
            ],
            r#"
struct Params { shift: f32 }
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let scale = params.shift / 1000.0;
    let from_center = uv - vec2<f32>(0.5);
    let r = textureSample(t_diffuse, s_diffuse, 0.5 + from_center * (1.0 + scale)).r;
    let g = textureSample(t_diffuse, s_diffuse, uv).g;
    let b = textureSample(t_diffuse, s_diffuse, 0.5 + from_center * (1.0 - scale)).b;
    let a = textureSample(t_diffuse, s_diffuse, uv).a;
    return vec4<f32>(r, g, b, a);
}
            "#.trim(),
        ));

        // 3. Vignette
        manifests.push(EffectManifest::new(
            "rupa.vignette",
            "Vignette",
            EffectCategory::Color,
            "Darkens or colors the outer borders of the frame.",
            vec![
                EffectParam { key: "intensity".into(), label: "Intensity".into(), param_type: ParamType::Float, min: 0.0, max: 1.0, default: 0.5, unit: None },
                EffectParam { key: "roundness".into(), label: "Roundness".into(), param_type: ParamType::Float, min: 0.0, max: 1.0, default: 0.5, unit: None },
            ],
            r#"
struct Params { intensity: f32, roundness: f32 }
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var c = textureSample(t_diffuse, s_diffuse, uv);
    let dist = length(uv - vec2<f32>(0.5));
    let vig = smoothstep(0.8, 0.2, dist * (1.0 + params.roundness));
    return vec4<f32>(c.rgb * mix(1.0, vig, params.intensity), c.a);
}
            "#.trim(),
        ));

        // 4. Film Grain
        manifests.push(EffectManifest::new(
            "rupa.film-grain",
            "Film Grain",
            EffectCategory::Stylize,
            "Simulates organic 35mm photographic silver-halide film grain.",
            vec![
                EffectParam { key: "amount".into(), label: "Amount".into(), param_type: ParamType::Float, min: 0.0, max: 1.0, default: 0.25, unit: None },
            ],
            r#"
struct Params { amount: f32 }
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var c = textureSample(t_diffuse, s_diffuse, uv);
    let noise = fract(sin(dot(uv, vec2<f32>(12.9898, 78.233))) * 43758.5453) - 0.5;
    return vec4<f32>(c.rgb + vec3<f32>(noise * params.amount), c.a);
}
            "#.trim(),
        ));

        // 5. RGB Split & Glitch
        manifests.push(EffectManifest::new(
            "rupa.rgb-split",
            "RGB Split Glitch",
            EffectCategory::Distort,
            "Horizontal digital channel dislocation glitch effect.",
            vec![
                EffectParam { key: "distance".into(), label: "Distance".into(), param_type: ParamType::Float, min: 0.0, max: 0.1, default: 0.02, unit: None },
            ],
            r#"
struct Params { distance: f32 }
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let r = textureSample(t_diffuse, s_diffuse, vec2<f32>(uv.x + params.distance, uv.y)).r;
    let g = textureSample(t_diffuse, s_diffuse, uv).g;
    let b = textureSample(t_diffuse, s_diffuse, vec2<f32>(uv.x - params.distance, uv.y)).b;
    let a = textureSample(t_diffuse, s_diffuse, uv).a;
    return vec4<f32>(r, g, b, a);
}
            "#.trim(),
        ));

        // 6. Cross Dissolve (Transition)
        manifests.push(EffectManifest::new(
            "rupa.cross-dissolve",
            "Cross Dissolve",
            EffectCategory::Transition,
            "Smooth linear opacity transition between outgoing and incoming clips.",
            vec![
                EffectParam { key: "progress".into(), label: "Progress".into(), param_type: ParamType::Float, min: 0.0, max: 1.0, default: 0.5, unit: None },
            ],
            r#"
struct Params { progress: f32 }
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let from_color = textureSample(t_from, s_from, uv);
    let to_color = textureSample(t_to, s_to, uv);
    return mix(from_color, to_color, params.progress);
}
            "#.trim(),
        ));

        Self { manifests }
    }

    /// Returns all registered effect manifests.
    pub fn list(&self) -> &[EffectManifest] {
        &self.manifests
    }

    /// Looks up an effect manifest by its unique ID.
    pub fn get(&self, id: &str) -> Option<&EffectManifest> {
        self.manifests.iter().find(|m| m.id == id)
    }

    /// Filters manifests by category.
    pub fn by_category(&self, category: EffectCategory) -> Vec<&EffectManifest> {
        self.manifests.iter().filter(|m| m.category == category).collect()
    }
}

impl Default for EffectCatalogue {
    fn default() -> Self {
        Self::new()
    }
}
