#[cfg(feature = "skia")]
#[doc(hidden)]
pub use skia_safe;
pub mod render;
pub mod text;
pub mod unit;

use goober_runtime::Oco;
use render::*;
use text::Font;
use unit::*;

pub trait View {
    fn precalc_size(&self) -> unit::ISize;
    fn render(&self, renderer: &impl Renderer, context: &RenderContext);
}
