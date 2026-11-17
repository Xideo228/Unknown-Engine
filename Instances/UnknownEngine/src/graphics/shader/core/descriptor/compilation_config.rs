#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderOptimization {
    None,
    Preference,
    Size
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderDebugInfo {
    None,
    LineDirectives,
    Full
}