#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderMacroValue {
    Defined(String),
    Undefined
}