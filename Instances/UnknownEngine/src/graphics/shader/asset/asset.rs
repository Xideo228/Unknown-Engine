use super::ShaderAssetError;
use crate::graphics::shader::{
    ShaderSource,
    ShaderStage
};

#[derive(Debug, Clone)]
pub struct ShaderAsset {
    sources: Vec<ShaderSource>
}

impl ShaderAsset {
    pub fn new(sources: Vec<ShaderSource>) -> Result<Self, ShaderAssetError> {
        if sources.is_empty() { return Err(ShaderAssetError::EmptySources); }

        for (index, source) in sources.iter().enumerate() {
            if sources[..index].iter().any(|previous| previous.stage() == source.stage()) {
                return Err(ShaderAssetError::DuplicateStage(source.stage()));
            }
        }

        Ok(Self { sources })
    }
}