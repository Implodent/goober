use goober_runtime::{MaybeSignal, SignalGet, SignalSet, SignalUpdate, SignalWith, WriteSignal};

use super::*;

pub struct Applied<V, M> {
    view: V,
    modifier: M,
}

impl<V: View, M: Modifier> View for Applied<V, M> {
    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.modifier.render(&self.view, canvas, how)
    }
    fn style(&self) -> Style {
        self.modifier.style(&self.view)
    }
    fn measure(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        self.modifier.measure(&self.view, current_node, taffy)
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
        self.modifier(Background(sig.map(IntoPaint::into_paint)))
    }

    fn offset<P: IntoRect<LengthPercentageAuto> + Clone + 'static>(
        self,
        offset: impl Into<MaybeSignal<P>>,
    ) -> Applied<Self, Offset>
    where
        Self: Sized,
    {
        let sig: MaybeSignal<P> = offset.into();
        self.modifier(Offset(sig.map(IntoRect::into_rect)))
    }

    fn padding<R: IntoRect<LengthPercentage> + Clone + 'static>(
        self,
        padding: impl Into<MaybeSignal<R>>,
    ) -> Applied<Self, Padding>
    where
        Self: Sized,
    {
        let sig: MaybeSignal<R> = padding.into();

        self.modifier(Padding(sig.map(IntoRect::into_rect)))
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

    fn align(self, alignment: impl Into<MaybeSignal<alignment::Alignment>>) -> Applied<Self, Align>
    where
        Self: Sized,
    {
        self.modifier(Align(alignment.into()))
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
        self.0.with(|paint| {
            canvas.draw_rect(taffy_rect(how.layout.size, how.layout.location).into_sk(), paint)
        });

        view.render(canvas, how)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset(MaybeSignal<Rect<LengthPercentageAuto>>);

impl Modifier for Offset {
    fn style(&self, view: &dyn View) -> Style {
        view.style().apply_mut(|style| style.inset = self.0.get())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Padding(MaybeSignal<Rect<LengthPercentage>>);

impl Modifier for Padding {
    fn style(&self, view: &dyn View) -> Style {
        view.style().apply_mut(|style| style.padding = self.0.get())
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
            if taffy_rect_contains(&taffy_rect(how.layout.size, how.layout.location), &point) {
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
            if taffy_rect_contains(&taffy_rect(how.layout.size, how.layout.location), &point) {
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

pub struct Align(MaybeSignal<alignment::Alignment>);

impl Modifier for Align {
    fn style(&self, view: &dyn View) -> Style {
        let align = self.0.get();
        view.style().apply_mut(|style| {
            style.justify_self = Some(align.horizontal.into());
            style.align_items = Some(align.vertical.into());
        })
    }
}
