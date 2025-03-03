use std::ops::Deref;

use palette::{color_difference::Ciede2000, FromColor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct ColorRGB(pub [u8; 3]);

impl ColorRGB {
    pub fn red(&self) -> u8 {
        self.as_slice()[0]
    }
    
    pub fn green(&self) -> u8 {
        self.as_slice()[1]
    }
    
    pub fn blue(&self) -> u8 {
        self.as_slice()[2]
    }

    pub fn as_slice(&self) -> &[u8; 3] {
        &self.0
    }

    pub fn tuple(&self) -> (u8, u8, u8) {
        (self.red(), self.green(), self.blue())
    }

    pub fn from_rgbu8(rgbu8: image::Rgb<u8>) -> Self {
        Self::from(rgbu8)
    }

    pub fn from_srgb(srgb: palette::Srgb) -> Self {
        Self::from(srgb)
    }

    pub fn from_lab(lab: palette::Lab) -> Self {
        Self::from(lab)
    }

    pub fn to_rgbu8(&self) -> image::Rgb<u8> {
        (*self).into()
    }

    pub fn to_srgb(&self) -> palette::Srgb {
        (*self).into()
    }

    pub fn to_lab(&self) -> palette::Lab {
        (*self).into()
    }
    
    pub fn saturating_add(&self, other: &Self) -> Self {
        ColorRGB([
            self[0].saturating_add(other[0]),
            self[1].saturating_add(other[1]),
            self[2].saturating_add(other[2])
        ])
    }

    pub fn saturating_sub(&self, other: &Self) -> Self {
        ColorRGB([
            self[0].saturating_sub(other[0]),
            self[1].saturating_sub(other[1]),
            self[2].saturating_sub(other[2])
        ])
    }

    pub fn saturating_mul_scalar(&self, scalar: f32) -> Self {
        ColorRGB([
            (self[0] as f32 * scalar).round().clamp(0.0, 255.0) as u8,
            (self[1] as f32 * scalar).round().clamp(0.0, 255.0) as u8,
            (self[2] as f32 * scalar).round().clamp(0.0, 255.0) as u8,
        ])
    }

    /// Euclidean distance squared in RGB 3D space
    pub fn dist_squared_by_rgb(&self, other: &Self) -> u32 {
        self.0.iter()
            .zip(other.0.iter())
            .map(|(&a, &b)| (a as u32).abs_diff(b as u32).pow(2))
            .sum()
    }
    
    /// Euclidean distance in RGB 3D space
    pub fn dist_by_rgb(&self, other: &Self) -> f32 {
        (self.dist_squared_by_rgb(other) as f32).sqrt()
    }

    /// Converted to LAB, distance using CIEDE2000 
    pub fn dist_by_lab(&self, other: &Self) -> f32 {
        self.to_lab().difference(other.to_lab())
    }

}

impl Ord for ColorRGB {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_lab = self.to_lab();
        let other_lab = other.to_lab();
        self_lab.l.partial_cmp(&other_lab.l).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for ColorRGB {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for ColorRGB {
    type Target = [u8; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<image::Rgb<u8>> for ColorRGB {
    fn from(value: image::Rgb<u8>) -> Self {
        ColorRGB([
            value.0[0],
            value.0[1],
            value.0[2],
        ])
    }
}

impl From<palette::Srgb> for ColorRGB {
    fn from(value: palette::Srgb) -> Self {
        Self([
            (value.red * 255.0).round().clamp(0.0, 255.0) as u8,
            (value.green * 255.0).round().clamp(0.0, 255.0) as u8,
            (value.blue * 255.0).round().clamp(0.0, 255.0) as u8,
        ])
    }
}

impl From<palette::Lab> for ColorRGB {
    fn from(value: palette::Lab) -> Self {
        Self::from(palette::Srgb::from_color(value))
    }
}

impl From<ColorRGB> for image::Rgb<u8> {
    fn from(value: ColorRGB) -> Self {
        image::Rgb(*value.as_slice())
    }
}

impl From<ColorRGB> for palette::Srgb {
    fn from(value: ColorRGB) -> Self {
        Self::new(
            value.red() as f32 / 255.0,
            value.green() as f32 / 255.0,
            value.blue() as f32 / 255.0
        )
    }
}

impl From<ColorRGB> for palette::Lab {
    fn from(value: ColorRGB) -> Self {
        palette::Lab::from_color(palette::Srgb::from(value))
    }
}

pub mod manip {
    use super::ColorRGB;

    pub fn rgbu8_to_srgb(src: image::Rgb<u8>) -> palette::Srgb {
        ColorRGB::from(src).to_srgb()
    }

    pub fn rgbu8_to_lab(src: image::Rgb<u8>) -> palette::Lab {
        ColorRGB::from(src).to_lab()
    }

    pub fn srgb_to_rgbu8(src: palette::Srgb) -> image::Rgb<u8> {
        ColorRGB::from(src).to_rgbu8()
    }

    pub fn lab_to_rgbu8(src: palette::Lab) -> image::Rgb<u8> {
        ColorRGB::from(src).to_rgbu8()
    }

    pub fn lab_add(left: &palette::Lab, right: &palette::Lab) -> palette::Lab {
        palette::Lab::new(
            left.l + right.l,
            left.a + right.a,
            left.b + right.b
        )
    }

    pub fn lab_mut_add(left: &mut palette::Lab, right: &palette::Lab) {
        left.l += right.l;
        left.a += right.a;
        left.b += right.b;
    }

    pub fn lab_sub(left: &palette::Lab, right: &palette::Lab) -> palette::Lab {
        palette::Lab::new(
            left.l - right.l,
            left.a - right.a,
            left.b - right.b
        )
    }

    pub fn lab_mul_scalar(left: &palette::Lab, scalar: f32) -> palette::Lab {
        palette::Lab::new(
            left.l * scalar,
            left.a * scalar,
            left.b * scalar
        )
    }
    
    pub fn srgb_add(left: &palette::Srgb, right: &palette::Srgb) -> palette::Srgb {
        palette::Srgb::new(
            left.red + right.red,
            left.green + right.green,
            left.blue + right.blue
        )
    }

    pub fn srgb_sub(left: &palette::Srgb, right: &palette::Srgb) -> palette::Srgb {
        palette::Srgb::new(
            left.red - right.red,
            left.green - right.green,
            left.blue - right.blue
        )
    }

    pub fn srgb_mul_scalar(left: &palette::Srgb, scalar: f32) -> palette::Srgb {
        palette::Srgb::new(
            left.red * scalar,
            left.green * scalar,
            left.blue * scalar
        )
    }
}

#[test]
fn test_convertion_to_lab() {
    let color = ColorRGB([255, 0, 0]);
    let lab_color = palette::Lab::from(color.clone());
    let recreated_color = ColorRGB::from(lab_color.clone());
    assert_eq!(color, recreated_color, "Failed! color={color:?}, lab_color={lab_color:?}, recreated_color={recreated_color:?}.");
}