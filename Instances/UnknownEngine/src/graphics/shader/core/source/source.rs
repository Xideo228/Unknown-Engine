use super::{
    shader_language::*,
    shader_stage::*,
    source_kind::*
};

pub struct ShaderSource {
    language: ShaderLanguage,
    stage: ShaderStage,
    source: ShaderSourceKind,
    entry_point: Option<String>
}

