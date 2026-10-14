use crate::pipeline::EffectsError;

/// A 3D Color Look Up Table parsed from a standard `.cube` file.
#[derive(Debug, Clone, PartialEq)]
pub struct CubeLut {
    pub title: Option<String>,
    pub size: u32,
    pub domain_min: [f32; 3],
    pub domain_max: [f32; 3],
    pub data: Vec<[f32; 3]>,
}

impl CubeLut {
    /// Parses a standard DaVinci / Adobe `.cube` 3D LUT string.
    pub fn parse(content: &str) -> Result<Self, EffectsError> {
        let mut title = None;
        let mut size = None;
        let mut domain_min = [0.0, 0.0, 0.0];
        let mut domain_max = [1.0, 1.0, 1.0];
        let mut data = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "TITLE" => {
                    title = Some(parts[1..].join(" ").trim_matches('"').to_string());
                }
                "LUT_3D_SIZE" => {
                    if let Ok(s) = parts[1].parse::<u32>() {
                        size = Some(s);
                    }
                }
                "DOMAIN_MIN" if parts.len() >= 4 => {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        parts[1].parse::<f32>(),
                        parts[2].parse::<f32>(),
                        parts[3].parse::<f32>(),
                    ) {
                        domain_min = [r, g, b];
                    }
                }
                "DOMAIN_MAX" if parts.len() >= 4 => {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        parts[1].parse::<f32>(),
                        parts[2].parse::<f32>(),
                        parts[3].parse::<f32>(),
                    ) {
                        domain_max = [r, g, b];
                    }
                }
                _ if parts.len() >= 3 => {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        parts[0].parse::<f32>(),
                        parts[1].parse::<f32>(),
                        parts[2].parse::<f32>(),
                    ) {
                        data.push([r, g, b]);
                    }
                }
                _ => {}
            }
        }

        let lut_size = size.ok_or_else(|| {
            EffectsError::InvalidInput("Missing LUT_3D_SIZE in .cube file".to_string())
        })?;

        let expected_count = (lut_size as usize).pow(3);
        if data.len() != expected_count {
            return Err(EffectsError::InvalidInput(format!(
                "Invalid .cube data: expected {expected_count} entries for size {lut_size}, got {}",
                data.len()
            )));
        }

        Ok(Self {
            title,
            size: lut_size,
            domain_min,
            domain_max,
            data,
        })
    }

    /// Evaluates the 3D LUT at normalized RGB coordinates [0.0, 1.0].
    pub fn sample(&self, r: f32, g: f32, b: f32) -> [f32; 3] {
        let n = self.size as f32;
        if n < 2.0 {
            return [r, g, b];
        }

        // Clamp to domain
        let cr = ((r - self.domain_min[0]) / (self.domain_max[0] - self.domain_min[0])).clamp(0.0, 1.0);
        let cg = ((g - self.domain_min[1]) / (self.domain_max[1] - self.domain_min[1])).clamp(0.0, 1.0);
        let cb = ((b - self.domain_min[2]) / (self.domain_max[2] - self.domain_min[2])).clamp(0.0, 1.0);

        let max_idx = (self.size - 1) as usize;
        let ir = ((cr * (n - 1.0)).round() as usize).min(max_idx);
        let ig = ((cg * (n - 1.0)).round() as usize).min(max_idx);
        let ib = ((cb * (n - 1.0)).round() as usize).min(max_idx);

        let index = ir + ig * (self.size as usize) + ib * (self.size as usize).pow(2);
        self.data[index]
    }
}
