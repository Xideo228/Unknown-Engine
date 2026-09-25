use super::types::DWORD;

#[repr(C)]
pub struct PixelFormatDescriptor {
    size: u16,
    version: u16,
    flags: DWORD,

    pixel_type: u8,
    color_bits: u8,
    red_bits: u8,
    red_shift: u8,
    green_bits: u8,
    green_shift: u8,
    blue_bits: u8,
    blue_shift: u8,
    alpha_bits: u8,
    alpha_shift: u8,

    accum_bits: u8,
    accum_red_bits: u8,
    accum_green_bits: u8,
    accum_blue_bits: u8,
    accum_alpha_bits: u8,

    depth_bits: u8,
    stencil_bits: u8,

    aux_buffers: u8,
    layer_type: u8,

    reserved: u8,

    layer_mask: DWORD,
    visible_mask: DWORD,
    damage_mask: DWORD
}