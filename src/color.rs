use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use cliffy::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Add for Color {
    type Output = Color;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Color;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign for Color {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl Neg for Color {
    type Output = Color;

    #[inline]
    fn neg(self) -> Color {
        self * -1.0
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f32> for Color {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign<Color> for Color {
    #[inline]
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl Div<f32> for Color {
    type Output = Color;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

impl DivAssign<f32> for Color {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

impl Div<Color> for Color {
    type Output = Color;

    #[inline]
    fn div(self, rhs: Color) -> Self::Output {
        Self::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl DivAssign<Color> for Color {
    #[inline]
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl Div<Color> for f32 {
    type Output = Color;

    #[inline]
    fn div(self, rhs: Color) -> Self::Output {
        Color::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl Index<usize> for Color {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _i => panic!(
                "{} is not a valid Index for {}",
                _i,
                std::any::type_name::<Color>()
            ),
        }
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _i => panic!(
                "{} is not a valid Index for {}",
                _i,
                std::any::type_name::<Color>()
            ),
        }
    }
}

impl Into<[f32; 3]> for Color {
    #[inline]
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from(comps: [f32; 3]) -> Self {
        Self::new(comps[0], comps[1], comps[2])
    }
}

impl From<&[f32; 3]> for Color {
    #[inline]
    fn from(comps: &[f32; 3]) -> Self {
        Self::from(*comps)
    }
}

impl From<&mut [f32; 3]> for Color {
    #[inline]
    fn from(comps: &mut [f32; 3]) -> Self {
        Self::from(*comps)
    }
}

impl From<(f32, f32, f32)> for Color {
    #[inline]
    fn from(comps: (f32, f32, f32)) -> Self {
        Self::new(comps.0, comps.1, comps.2)
    }
}

impl From<&(f32, f32, f32)> for Color {
    #[inline]
    fn from(comps: &(f32, f32, f32)) -> Self {
        Self::from(*comps)
    }
}

impl From<Color> for (f32, f32) {
    #[inline]
    fn from(v: Color) -> Self {
        (v.r, v.g)
    }
}

impl From<Vec3> for Color {
    #[inline]
    fn from(vec3: Vec3) -> Self {
        Self::new(vec3.x, vec3.y, vec3.z)
    }
}
