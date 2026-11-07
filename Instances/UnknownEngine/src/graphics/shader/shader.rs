use super::shader_stage::ShaderStage;
use std::{
    fs, path::Path
};

#[derive(Debug, Clone)]
pub struct ShaderSource {
    pub stage: ShaderStage,
    pub source: String
}

impl ShaderSource {
    pub fn new(stage: ShaderStage, source: impl Into<String>) -> Self {
        Self {
            stage,
            source: source.into()
        }
    }

    pub fn from_file(stage: ShaderStage, path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();

        let source = fs::read_to_string(path).map_err(|error| {
            format!("Failed to read shader '{}': {}", path.display(), error)
        })?;

        Ok(Self {
            stage,
            source
        })
    }
}