use goober_runtime::{MaybeSignal, Oco, SignalGet, SignalWith};
use skia_safe::{Color, Color4f};

use super::*;

pub struct Text {
    pub text: MaybeSignal<Oco<'static, str>>,
    pub font: MaybeSignal<Font>,
    pub paint: MaybeSignal<Paint>,
}

impl Text {
    pub fn font(self, font: impl Into<MaybeSignal<Font>>) -> Self {
        Self {
            font: font.into(),
            ..self
        }
    }

    pub fn font_size(self, size: impl Into<MaybeSignal<f32>>) -> Self {
        let size: MaybeSignal<f32> = size.into();
        Self {
            font: MaybeSignal::derive(move || {
                self.font
                    .get()
                    .with_size(size.get())
                    .expect("font wasn't able to scale up")
            }),
            ..self
        }
    }

    pub fn paint<P: IntoPaint + Clone + 'static>(self, paint: impl Into<MaybeSignal<P>>) -> Self {
        let paint: MaybeSignal<P> = paint.into();
        Self {
            paint: paint.map(IntoPaint::into_paint),
            ..self
        }
    }
}

impl View for Text {
    fn style(&self) -> Style {
        Style {
            size: self.text.with(|text| {
                self.font.with(|font| {
                    self.paint.with(|paint| {
                        Size::from_sk(font.measure_str(text.as_str(), Some(paint)).1.size()).map(Dimension::Points)
                    })
                })
            }),
            ..Default::default()
        }
    }

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.text.with(|text| {
            self.font.with(|font| {
                self.paint.with(|paint| {
                    canvas.draw_str(
                        text.as_str(),
                        Point {
                            x: how.layout.location.x,
                            y: how.layout.location.y,
                        }.into_sk(),
                        font,
                        paint,
                    )
                })
            });
        })
    }
}

pub trait StrFn<'a> {
    fn sig(self) -> MaybeSignal<Oco<'a, str>>;
}

impl<'a> StrFn<'a> for &'a str {
    fn sig(self) -> MaybeSignal<Oco<'a, str>> {
        MaybeSignal::Static(Oco::Borrowed(self))
    }
}

impl<'a> StrFn<'a> for String {
    fn sig(self) -> MaybeSignal<Oco<'a, str>> {
        MaybeSignal::Static(Oco::Owned(self))
    }
}

impl<'a, F: Fn() -> Oco<'a, str> + 'static> StrFn<'a> for F {
    fn sig(self) -> MaybeSignal<Oco<'a, str>> {
        MaybeSignal::derive(self)
    }
}

pub fn text(text: impl StrFn<'static>) -> Text {
    Text {
        text: text.sig(),
        font: MaybeSignal::Static(Font::default()),
        paint: MaybeSignal::Static(Paint::new(Color4f::from(Color::BLACK), None)),
    }
}
