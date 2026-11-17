use std::num::NonZeroU64;
use super::ids::ShaderVersionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderInterfaceGeneration(NonZeroU64);

impl ShaderInterfaceGeneration {
    #[inline]
    pub(crate) const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

pub struct ShaderVersion {
    id: ShaderVersionId,
    generation: ShaderInterfaceGeneration
}

impl ShaderVersion {
    pub(crate) fn new(id: ShaderVersionId, generation: ShaderInterfaceGeneration) -> Self {
        Self {
            id, generation
        }
    }

    #[inline]
    pub const fn id(&self) -> ShaderVersionId {
        self.id
    }

    #[inline]
    pub const fn generation(&self) -> ShaderInterfaceGeneration {
        self.generation
    }
}