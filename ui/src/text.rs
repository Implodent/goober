use super::*;

#[cfg(feature = "skia")]
pub use skia_safe::Font;

#[cfg(not(feature = "skia"))]
compile_error!("uhh sorry no skia isn't supported right now :(");

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub text: Oco<'static, str>,
    pub font: Font,
    pub paint: Paint,
}

impl Text {
    pub fn text(self, text: impl Into<Oco<'static, str>>) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }
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

impl View for Text {
    fn precalc_size(&self) -> unit::ISize {
        self.font
            .measure_str(&self.text, Some(&self.paint))
            .1
            .round()
            .size()
    }
    fn render(&self, renderer: &impl Renderer, context: &RenderContext) {
        renderer.draw_text(&self.text, context.position, &self.font, &self.paint);
    }
}

pub fn text(text: impl Into<Oco<'static, str>>) -> Text {
    Text {
        text: text.into(),
        font: Font::default(),
        paint: Paint::default(),
    }
}
