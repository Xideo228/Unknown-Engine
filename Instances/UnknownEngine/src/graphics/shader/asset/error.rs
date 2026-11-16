use crate::graphics::shader::ShaderStage;

pub enum ShaderAssetError {
    EmptySources,
    DuplicateStage(ShaderStage)
}