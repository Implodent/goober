#[cfg(not(feature = "skia"))]
pub type scalar = f32;

#[cfg(feature = "skia")]
pub use skia_safe::{IPoint, IRect, ISize, Point, Rect, Size};

#[cfg(not(feature = "skia"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub scalar, pub scalar);

/// Density specifies how much pixels are in a [`Dp`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Density(pub f32);

impl Density {
    pub fn pixels(&self, dp: Dp) -> f32 {
        dp.0 * self.0
    }

    pub fn round_to_pixels(&self, dp: Dp) -> i32 {
        let px = self.pixels(dp);

        px.round() as i32
    }

    // amazing
    pub fn dp(&self, pixels: impl IntoDp) -> Dp {
        (pixels.dp().0 / self.0).dp()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Dp(pub f32);

impl Dp {
    pub const ZERO: Self = Self(0f32);
}

pub trait IntoDp {
    fn dp(self) -> Dp;
}

impl IntoDp for f32 {
    #[inline(always)]
    fn dp(self) -> Dp {
        Dp(self)
    }
}

impl IntoDp for i32 {
    fn dp(self) -> Dp {
        Dp(self as f32)
    }
}
