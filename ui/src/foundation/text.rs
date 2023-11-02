use super::*;

#[cfg(feature = "skia")]
pub use skia_safe::Font;

#[cfg(not(feature = "skia"))]
compile_error!("uhh sorry no skia isn't supported right now :(");

#[derive(Clone, Debug, PartialEq)]
pub struct Text<F> {
    pub text: F,
    pub font: Font,
    pub paint: Paint,
}

impl<F> Text<F> {
    pub fn font(self, font: Font) -> Self {
        Self { font, ..self }
    }
    pub fn paint(self, paint: impl Into<Paint>) -> Self {
        Self {
            paint: paint.into(),
            ..self
        }
    }
}

impl<F: Fn() -> Oco<'static, str>> View for Text<F> {
    fn size(&self) -> unit::ISize {
        let text = (self.text)();
        self.font
            .measure_str(&text, Some(&self.paint))
            .1
            .round()
            .size()
    }
    fn render(&self, renderer: &mut dyn Renderer, context: &RenderContext) {
        let text = (self.text)();
        renderer.draw_text(&text, context.position.into(), &self.font, &self.paint);
    }
}

pub fn text<F>(text: F) -> Text<F> {
    Text {
        text,
        font: Font::default(),
        paint: Paint::default(),
    }
}
