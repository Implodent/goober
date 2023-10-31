#[cfg(not(feature = "skia"))]
pub type scalar = f32;

#[cfg(feature = "skia")]
pub use skia_safe::{ISize, Point, Size};

#[cfg(not(feature = "skia"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub scalar, pub scalar);
