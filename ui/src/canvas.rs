use super::*;

pub struct WithCanvas<F> {
    f: F,
    style: Style,
}

impl<F: Fn(&Canvas)> View for WithCanvas<F> {
    fn style(&self) -> Style {
        self.style.clone()
    }
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        canvas.save();
        canvas.translate(how.layout.location.into_sk());

        (self.f)(canvas);

        canvas.restore();
    }
    #[cfg(feature = "terminal")]
    fn render_terminal(
        &self,
        _renderer: &mut Terminal,
        _how: &RenderContext,
    ) -> Result<(), std::io::Error> {
        panic!("WithCanvas component cannot render to a terminal.")
    }
}

pub fn with_canvas<F: Fn(&Canvas)>(f: F, style: Style) -> WithCanvas<F> {
    WithCanvas { f, style }
}

pub struct Rectangle {
    rect: Rect<Dp>,
    paint: Paint,
}

impl View for Rectangle {
    fn style(&self) -> Style {
        Style {
            size: Size {
                width: Dimension::Points(self.rect.grid_axis_sum(AbsoluteAxis::Horizontal).0),
                height: Dimension::Points(self.rect.grid_axis_sum(AbsoluteAxis::Vertical).0),
            },
            ..Default::default()
        }
    }

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        canvas.draw_rect(
            self.rect.map(|x| how.density.pixels(x)).into_sk(),
            self.paint.as_ref(),
        );
    }

    #[cfg(feature = "terminal")]
    fn render_terminal(
            &self,
            _renderer: &mut Terminal,
            _how: &RenderContext,
        ) -> Result<(), std::io::Error> {
        todo!("Rectangle (currently) cannot render to a terminal.")
    }
}

pub fn rectangle(rect: impl IntoRect<Dp>, paint: impl IntoPaint) -> Rectangle {
    Rectangle {
        rect: rect.into_rect(),
        paint: paint.into_paint(),
    }
}
