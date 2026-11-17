use std::num::NonZeroU64;

macro_rules! define_shader_id {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(NonZeroU64);

        impl $name {
            #[inline]
            pub(crate) const fn new(value: NonZeroU64) -> Self {
                Self(value)
            }

            #[inline]
            pub const fn get(self) -> u64 {
                self.0.get()
            }
        }
    };
}

define_shader_id!(ShaderVersionId, "Unique identifier of an immutable shader version");
define_shader_id!(ShaderInterfaceId, "Unique identifier of a shader interface");
define_shader_id!(ShaderPassId, "Unique identifier of a shader pass");
define_shader_id!(ShaderVariantId, "Unique identifier of a shader variant");