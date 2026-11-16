use crate::graphics::shader::{
    instance::handle::ShaderInstanceHandle,
    ShaderStage
};

pub struct ShaderInstance {
    handle: ShaderInstanceHandle,
    stages: Vec<ShaderStage>
}