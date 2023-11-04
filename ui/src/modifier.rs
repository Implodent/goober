use goober_runtime::{SignalSet, SignalUpdate, WriteSignal};

use super::*;

pub struct Applied<V, M> {
    view: V,
    modifier: M,
}

impl<V: View, M: Modifier> View for Applied<V, M> {
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.modifier.render(&self.view, canvas, how)
    }
    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        self.modifier.measure(&self.view, context)
    }
    fn ev(&self, event: &Event, how: &RenderContext) {
        self.modifier.ev(&self.view, event, how)
    }
}

pub trait ApplyModifier {
    fn modifier<M>(self, modifier: M) -> Applied<Self, M>
    where
        Self: Sized;

    fn background(self, background: impl IntoPaint) -> Applied<Self, Background>
    where
        Self: Sized,
    {
        self.modifier(Background(background.into_paint()))
    }

    fn offset(self, offset: impl Into<IPoint>) -> Applied<Self, Offset>
    where
        Self: Sized,
    {
        self.modifier(Offset(offset.into()))
    }

    fn padding(self, padding: impl IntoIRect) -> Applied<Self, Padding>
    where
        Self: Sized,
    {
        self.modifier(Padding(padding.into_irect()))
    }

    fn on<F: Fn(&Event)>(self, handler: F) -> Applied<Self, OnEvent<F>>
    where
        Self: Sized,
    {
        self.modifier(OnEvent(handler))
    }

    fn on_click<F: Fn(MouseButton)>(self, handler: F) -> Applied<Self, OnClick<F>>
    where
        Self: Sized,
    {
        self.modifier(OnClick(handler))
    }

    fn hovering(self, write: WriteSignal<bool>) -> Applied<Self, Hovering>
    where
        Self: Sized,
    {
        self.modifier(Hovering(write))
    }
}

impl<V: View> ApplyModifier for V {
    fn modifier<M>(self, modifier: M) -> Applied<Self, M>
    where
        Self: Sized,
    {
        Applied {
            view: self,
            modifier,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Background(Paint);

impl Modifier for Background {
    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        canvas.draw_irect(view.measure(&(*how).into()).rect, &self.0);

        view.render(canvas, how)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset(IPoint);

impl Modifier for Offset {
    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        view.render(
            canvas,
            &RenderContext {
                offset: how.offset + self.0,
                constraints: how.constraints,
            },
        )
    }
    fn measure(&self, view: &dyn View, context: &MeasureContext) -> MeasureResult {
        view.measure(&MeasureContext {
            offset: context.offset + self.0,
            constraints: context.constraints,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Padding(IRect);

impl Modifier for Padding {
    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        view.render(
            canvas,
            &RenderContext {
                offset: IPoint {
                    x: how.offset.x + self.0.left(),
                    y: how.offset.y + self.0.top(),
                },
                constraints: Constraints {
                    min: ISize {
                        width: how.constraints.min.width - self.0.left(),
                        height: how.constraints.min.height - self.0.top(),
                    },
                    max: ISize {
                        width: how.constraints.max.width - self.0.right(),
                        height: how.constraints.max.height - self.0.bottom(),
                    },
                },
            },
        )
    }

    fn measure(&self, view: &dyn View, context: &MeasureContext) -> MeasureResult {
        let mr = view.measure(&MeasureContext {
            offset: IPoint {
                x: context.offset.x + self.0.left(),
                y: context.offset.y + self.0.top(),
            },
            constraints: Constraints {
                min: ISize {
                    width: context.constraints.min.width + self.0.left(),
                    height: context.constraints.min.height + self.0.top(),
                },
                max: ISize {
                    width: context.constraints.max.width - self.0.right(),
                    height: context.constraints.max.height - self.0.bottom(),
                },
            },
        });

        MeasureResult {
            rect: mr
                .rect
                .with_adjustment(-self.0.left, -self.0.top, self.0.right, self.0.bottom),
        }
    }
}

pub struct OnEvent<F>(F);

impl<F: Fn(&Event)> Modifier for OnEvent<F> {
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        (self.0)(event);

        view.ev(event, how)
    }
}

pub struct OnClick<F>(F);

impl<F: Fn(MouseButton)> Modifier for OnClick<F> {
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        if let Event::Click(point, button) = event {
            if view
                .measure(&(*how).into())
                .rect
                .contains(IPoint::from((point.x as i32, point.y as i32)))
            {
                (self.0)(*button);
            }
        }

        view.ev(event, how)
    }
}

pub struct Hovering(WriteSignal<bool>);

impl Modifier for Hovering {
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        if let Event::CursorMove(point) = event {
            if view
                .measure(&(*how).into())
                .rect
                .contains(IPoint::from((point.x as i32, point.y as i32)))
            {
                self.0.set(true);
            } else {
                self.0.update(|x| {
                    if *x {
                        *x = false;
                    }
                })
            }
        }
    }
}
