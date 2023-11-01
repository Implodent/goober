use super::*;

pub struct RenderContext {
    pub position: IPoint,
    pub density: Density,
}

pub trait Renderer {
    fn size(&mut self) -> ISize;
    fn draw_text(&mut self, text: &str, position: Point, font: &Font, paint: &Paint);
    fn draw_rect(&mut self, rect: IRect, paint: &Paint);
}

#[cfg(feature = "skia")]
impl Renderer for skia_safe::Canvas {
    fn size(&mut self) -> ISize {
        self.image_info().dimensions()
    }
    fn draw_text(&mut self, text: &str, position: Point, font: &Font, paint: &Paint) {
        self.draw_str(text, position, font, paint);
    }
    fn draw_rect(&mut self, rect: IRect, paint: &Paint) {
        self.draw_irect(rect, paint);
    }
}
#[cfg(feature = "skia")]
impl Renderer for skia_safe::Surface {
    fn size(&mut self) -> ISize {
        self.image_info().dimensions()
    }
    fn draw_text(&mut self, text: &str, position: Point, font: &Font, paint: &Paint) {
        self.canvas().draw_str(text, position, font, paint);
    }
    fn draw_rect(&mut self, rect: IRect, paint: &Paint) {
        self.canvas().draw_irect(rect, paint);
    }
}

use skia_safe::IRect;
#[cfg(feature = "skia")]
pub use skia_safe::{Color, Paint};
