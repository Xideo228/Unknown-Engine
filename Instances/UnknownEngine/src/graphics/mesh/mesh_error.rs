#[derive(Debug)]
pub enum MeshError {
    EmptyLayout,
    InvalidAttributeSize,
    InvalidVertexData,
    InvalidIndex(usize)
}