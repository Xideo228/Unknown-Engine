use std::path::PathBuf;

pub enum ShaderSourceKind {
    File(PathBuf),
    Memory(String),
    //Asset(AssetPath)
}