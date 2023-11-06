use goober_runtime::{
    create_memo, MaybeSignal, SignalGet, SignalSet, SignalUpdate, SignalWith, WriteSignal,
};

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

    fn background<P: IntoPaint + Clone + 'static>(
        self,
        background: impl Into<MaybeSignal<P>>,
    ) -> Applied<Self, Background>
    where
        Self: Sized,
    {
        let sig: MaybeSignal<P> = background.into();
        self.modifier(Background(
            create_memo(move |_| sig.get().into_paint()).into(),
        ))
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

    fn align(self, alignment: alignment::Alignment) -> Applied<Self, Align>
    where
        Self: Sized,
    {
        self.modifier(Align(alignment))
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
pub struct Background(MaybeSignal<Paint>);

impl Modifier for Background {
    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        self.0
            .with(|paint| canvas.draw_irect(view.measure(&(*how).into()).rect, paint));

        view.render(canvas, how)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset(IPoint);

impl Modifier for Offset {
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        view.ev(
            event,
            &RenderContext {
                offset: how.offset + self.0,
                constraints: how.constraints,
                ..*how
            },
        )
    }
    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        view.render(
            canvas,
            &RenderContext {
                offset: how.offset + self.0,
                constraints: how.constraints,
                ..*how
            },
        )
    }
    fn measure(&self, view: &dyn View, context: &MeasureContext) -> MeasureResult {
        view.measure(&MeasureContext {
            offset: context.offset + self.0,
            constraints: context.constraints,
            ..*context
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Padding(IRect);

impl Modifier for Padding {
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        view.ev(
            event,
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
                ..*how
            },
        )
    }
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
                ..*how
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
            ..*context
        });

        MeasureResult {
            rect: mr
                .rect
                .with_adjustment(-self.0.left, -self.0.top, self.0.right, self.0.bottom),
            ..mr
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
            if dbg!(dbg!(view.measure(&(*how).into()).rect)
                .contains(IPoint::from((point.x as i32, point.y as i32))))
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

pub struct Align(alignment::Alignment);

impl Modifier for Align {
    fn measure(&self, view: &dyn View, context: &MeasureContext) -> MeasureResult {
        let mr = view.measure(context);

        let aligned = self.0.align(mr.size(), context.space);

        MeasureResult {
            rect: mr.rect.with_offset(aligned),
            ..mr
        }
    }

    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        let mr = view.measure(&(*how).into());
        let aligned = self.0.align(mr.size(), how.space);
        view.ev(
            event,
            &RenderContext {
                offset: how.offset + aligned,
                ..*how
            },
        )
    }

    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        let measured = view.measure(&(*how).into());
        let aligned = self.0.align(measured.size(), how.space);

        view.render(
            canvas,
            &RenderContext {
                offset: how.offset + aligned,
                space: ISize {
                    width: how.space.width - aligned.x,
                    height: how.space.height - aligned.y,
                },
                ..*how
            },
        )
    }
}
