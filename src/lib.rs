/// To organize things
/// 
/// ## Colors used
/// 
/// pallete crate:
/// - Srgb (f32 RGB)
/// - Lab (f32 LAB color)
/// 
/// image crate:
/// - Rgb<u8>
/// 
/// self:
/// - ColorRGB ([u8; 3] same as image::Rgb<u8> but can be easly serialized) 
///

pub mod algorithms;
pub mod image;
pub mod color;
pub mod palette;
