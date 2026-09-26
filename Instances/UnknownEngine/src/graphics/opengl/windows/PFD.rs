use crate::platforms::DWORD;

#[repr(C)]
pub struct PixelFormatDescriptor {
    pub size: u16,
    pub version: u16,
    pub flags: DWORD,

    pub pixel_type: u8,
    pub color_bits: u8,
    pub red_bits: u8,
    pub red_shift: u8,
    pub green_bits: u8,
    pub green_shift: u8,
    pub blue_bits: u8,
    pub blue_shift: u8,
    pub alpha_bits: u8,
    pub alpha_shift: u8,

    pub accum_bits: u8,
    pub accum_red_bits: u8,
    pub accum_green_bits: u8,
    pub accum_blue_bits: u8,
    pub accum_alpha_bits: u8,

    pub depth_bits: u8,
    pub stencil_bits: u8,

    pub aux_buffers: u8,
    pub layer_type: u8,

    pub reserved: u8,

    pub layer_mask: DWORD,
    pub visible_mask: DWORD,
    pub damage_mask: DWORD
}