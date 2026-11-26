use crate::graphics::shader::property::engine_semantic::EngineSemantic;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderSemantic {
    Engine(EngineSemantic),
    Game(String)
}