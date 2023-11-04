use goober_runtime::Oco;
use skia_safe::{Color, Color4f};

use super::*;

pub struct Text<F> {
    pub text: F,
    pub font: Font,
    pub paint: Paint,
}

impl<F> Text<F> {
    pub fn font(self, font: impl Into<Font>) -> Self {
        Self {
            font: font.into(),
            ..self
        }
    }

    pub fn paint(self, paint: impl IntoPaint) -> Self {
        Self {
            paint: paint.into_paint(),
            ..self
        }
    }
}

impl<F: StrFn<'static>> View for Text<F> {
    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        MeasureResult {
            rect: IRect::from_pt_size(
                context.offset,
                self.font
                    .measure_str(self.text.oco().as_str(), Some(&self.paint))
                    .1
                    .round()
                    .size(),
            ),
        }
    }

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        let text = self.text.oco();
        let bounds = self.measure(&(*how).into()).rect;

        canvas.draw_str(
            text.as_str(),
            IPoint {
                x: how.offset.x,
                y: how.offset.y + bounds.height().abs(),
            },
            &self.font,
            &self.paint,
        );
    }
}

pub trait StrFn<'a> {
    fn oco(&self) -> Oco<'a, str>;
}

impl<'a> StrFn<'a> for &'a str {
    fn oco(&self) -> Oco<'a, str> {
        Oco::Borrowed(*self)
    }
}

impl<'a, F: Fn() -> O, O: Into<Oco<'a, str>>> StrFn<'a> for F {
    fn oco(&self) -> Oco<'a, str> {
        (self)().into()
    }
}

pub fn text<F: StrFn<'static>>(text: F) -> Text<F> {
    Text {
        text,
        font: Font::default(),
        paint: Paint::new(Color4f::from(Color::BLACK), None),
    }
}
