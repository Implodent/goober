use super::*;

pub struct Button<C, H> {
    #[doc(hidden)]
    pub child: C,
    #[doc(hidden)]
    pub on_click: H,
}

impl<C: View, H: Fn(Point, MouseButton)> View for Button<C, H> {
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.child.render(canvas, how)
    }

    fn ev(&self, event: &Event, how: &RenderContext) {
        match event {
            Event::Click(click, button)
                if self
                    .child
                    .measure(&(*how).into())
                    .rect
                    .contains(IPoint::from((click.x as i32, click.y as i32))) =>
            {
                (self.on_click)(*click, *button)
            }
            _ => {}
        };

        self.child.ev(&event, how)
    }

    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        self.child.measure(context)
    }
}

pub fn button<C, H: Fn(Point, MouseButton)>(child: C, on_click: H) -> Button<C, H> {
    Button { child, on_click }
}
