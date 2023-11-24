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
                if taffy_rect_contains(
                    &taffy_rect(how.layout.size, how.layout.location),
                    &click,
                ) =>
            {
                (self.on_click)(*click, *button)
            }
            _ => {}
        };

        self.child.ev(&event, how)
    }

    fn style(&self) -> Style {
        self.child.style()
    }
}

pub fn button<C, H: Fn(Point, MouseButton)>(child: C, on_click: H) -> Button<C, H> {
    Button { child, on_click }
}
