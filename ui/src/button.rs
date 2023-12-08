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

    fn measure(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        self.child.measure(current_node, taffy)
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
            #[cfg(feature = "terminal")]
            Event::Terminal(crossterm::event::Event::Mouse(crossterm::event::MouseEvent {
                row,
                column,
                kind: crossterm::event::MouseEventKind::Down(button),
                modifiers: _,
            })) if taffy_rect_contains(
                &taffy_rect(how.layout.size, how.layout.location),
                &Point {
                    x: *row as _,
                    y: *column as _,
                },
            ) =>
            {
                (self.on_click)(
                    Point {
                        x: *row as f32,
                        y: *column as f32,
                    },
                    (*button).into(),
                )
            }
            _ => {}
        };

        self.child.ev(&event, how)
    }

    fn style(&self) -> Style {
        self.child.style()
    }

    #[cfg(feature = "terminal")]
    fn render_terminal(
        &self,
        renderer: &mut Terminal,
        how: &RenderContext,
    ) -> Result<(), std::io::Error> {
        self.child.render_terminal(renderer, how)
    }

    #[cfg(feature = "terminal")]
    fn measure_terminal(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        self.child.measure_terminal(current_node, taffy)
    }

    #[cfg(feature = "terminal")]
    fn style_terminal(&self) -> Style {
        self.child.style_terminal()
    }
}

pub fn button<C, H: Fn(Point, MouseButton)>(child: C, on_click: H) -> Button<C, H> {
    Button { child, on_click }
}
