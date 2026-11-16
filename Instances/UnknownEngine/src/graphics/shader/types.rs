use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Compute
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderSourceFormat {
    Glsl,
    SpirV
}