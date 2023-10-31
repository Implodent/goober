use super::*;

pub struct RenderContext {
    pub position: Point,
}

pub trait Renderer {
    fn draw_text(
        &self,
        text: impl AsRef<str>,
        position: impl Into<Point>,
        font: &Font,
        paint: &Paint,
    );
}

#[cfg(feature = "skia")]
impl Renderer for skia_safe::Canvas {
    fn draw_text(
        &self,
        text: impl AsRef<str>,
        position: impl Into<Point>,
        font: &Font,
        paint: &Paint,
    ) {
        self.draw_str(text, position, font, paint);
    }
}

#[cfg(feature = "skia")]
pub use skia_safe::{Color, Paint};
