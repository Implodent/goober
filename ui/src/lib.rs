use modifier::Modifier;
#[cfg(feature = "skia")]
#[doc(hidden)]
pub use skia_safe;
pub mod modifier;
pub mod render;
pub mod text;
pub mod unit;

pub use goober_runtime as runtime;
use goober_runtime::Oco;
use render::*;
use text::Font;
use unit::*;

pub trait View {
    fn modifiers(&self) -> Option<Box<dyn Iterator<Item = &'_ dyn Modifier>>> {
        None
    }
    fn size(&self) -> unit::ISize;
    fn render(&self, renderer: &mut dyn Renderer, context: &RenderContext);
}
