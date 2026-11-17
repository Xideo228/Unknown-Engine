mod ids;
mod shader_id;
mod shader_version;

mod source;
mod descriptor;

pub use ids::*;
pub use shader_id::ShaderId;

pub use shader_version::{
    ShaderInterfaceGeneration,
    ShaderVersion
};