use skia_safe::Matrix;

use super::*;

pub struct WithCanvas<F, M> {
    f: F,
    measure: M,
}

impl<F: Fn(&Canvas), M: Fn(&MeasureContext) -> IRect> View for WithCanvas<F, M> {
    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        MeasureResult::new((self.measure)(context))
    }

    fn ev(&self, _event: &Event, _how: &RenderContext) {}

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        canvas.save();
        canvas.concat(&Matrix::translate((
            how.offset.x as f32,
            how.offset.y as f32,
        )));

        (self.f)(canvas);

        canvas.restore();
    }
}

pub fn with_canvas<F: Fn(&Canvas), M: Fn(&MeasureContext) -> IRect>(
    f: F,
    measure: M,
) -> WithCanvas<F, M> {
    WithCanvas { f, measure }
}

pub struct Rectangle {
    rect: IRect,
    paint: Paint,
}

impl View for Rectangle {
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        canvas.draw_irect(self.rect.with_offset(how.offset), &self.paint);
    }
    fn ev(&self, _event: &Event, _how: &RenderContext) {}
    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        MeasureResult::new(self.rect.with_offset(context.offset))
    }
}

pub fn rectangle(rect: IRect, paint: impl IntoPaint) -> Rectangle {
    Rectangle {
        rect,
        paint: paint.into_paint(),
    }
}
