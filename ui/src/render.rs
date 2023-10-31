use super::*;

pub struct RenderContext {
    pub position: Point,
}

pub trait Renderer {
    fn draw_text(&mut self, text: &str, position: Point, font: &Font, paint: &Paint);
}

#[cfg(feature = "skia")]
impl Renderer for skia_safe::Canvas {
    fn draw_text(&mut self, text: &str, position: Point, font: &Font, paint: &Paint) {
        self.draw_str(text, position, font, paint);
    }
}
#[cfg(feature = "skia")]
impl Renderer for skia_safe::Surface {
    fn draw_text(&mut self, text: &str, position: Point, font: &Font, paint: &Paint) {
        self.canvas().draw_str(text, position, font, paint);
    }
}

#[cfg(feature = "skia")]
pub use skia_safe::{Color, Paint};
