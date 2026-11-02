use super::shader_stage::ShaderStage;

#[derive(Debug, Clone)]
pub struct ShaderSource {
    pub stage: ShaderStage,
    pub source: String
}

impl ShaderSource {
    pub fn new(stage: ShaderStage, source: String) -> Self {
        Self {
            stage,
            source: source.into()
        }
    }
}