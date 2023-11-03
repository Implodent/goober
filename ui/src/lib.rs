pub use goober_runtime as runtime;
use goober_runtime::OcoFn;
pub use skia_safe;
use skia_safe::{Canvas, Font, IPoint, Paint};

pub struct RenderContext {
    pub offset: IPoint,
}

pub trait View {
    fn render(&self, canvas: &Canvas, how: &RenderContext);
}

pub struct Text<F> {
    pub text: F,
    pub font: Font,
    pub paint: Paint,
}

impl<F: OcoFn<'static, str>> View for Text<F> {
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        canvas.draw_str(
            self.text.get().as_str(),
            how.offset,
            &self.font,
            &self.paint,
        );
    }
}
