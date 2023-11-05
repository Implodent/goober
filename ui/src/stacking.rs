use super::*;

pub struct StackX<V, A> {
    views: V,
    arrangement: A,
    alignment: alignment::Horizontal,
}

impl<V, A> StackX<V, A> {
    pub fn align(self, alignment: alignment::Horizontal) -> Self {
        Self { alignment, ..self }
    }
    pub fn arrange<A2>(self, arrangement: A2) -> StackX<V, A2> {
        StackX {
            views: self.views,
            arrangement,
            alignment: self.alignment,
        }
    }
}

impl<V: Views, A: arrangement::Horizontal> View for StackX<V, A> {
    fn ev(&self, event: &Event, how: &RenderContext) {
        self.views.iter().for_each(|x| x.ev(event, how));
    }

    fn measure(&self, context: &MeasureContext) -> MeasureResult {
        let children = self
            .views
            .iter()
            .map(|x| x.measure(context))
            .collect::<Vec<_>>();

        MeasureResult::new(
            children
                .iter()
                .zip(
                    self.arrangement.arrange(
                        context.density,
                        context.space.width,
                        children
                            .iter()
                            .map(|x| x.rect.width() + x.advance_width)
                            .collect::<Vec<i32>>(),
                    ),
                )
                .fold(
                    IRect::new_empty(),
                    |rect, (MeasureResult { rect: view, .. }, offset)| {
                        rect.with_adjustment(view.left, view.top, view.right + offset, view.bottom)
                    },
                ),
        )
    }

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        let children = self.views.iter().collect::<Vec<_>>();
        let _measure = (*how).into();
        let children_measured = children
            .iter()
            .map(|x| x.measure(&_measure))
            .collect::<Vec<_>>();

        let offsets = self.arrangement.arrange(
            how.density,
            how.space.width,
            children_measured
                .iter()
                .map(|x| x.rect.width() + x.advance_width)
                .collect::<Vec<i32>>(),
        );

        let aligned = self.alignment.align(
            children_measured
                .iter()
                .zip(offsets.iter())
                .fold(0, |size, (mr, arranged)| mr.rect.width() + arranged + size),
            how.space.width,
        );

        for (view, offset) in children.into_iter().zip(offsets) {
            view.render(
                canvas,
                &RenderContext {
                    offset: IPoint {
                        x: how.offset.x + offset + aligned,
                        ..how.offset
                    },
                    ..*how
                },
            );
        }
    }
}

pub fn stack_x<V: Views>(views: V) -> StackX<V, arrangement::BuiltinHorizontal> {
    StackX {
        views,
        arrangement: arrangement::BuiltinHorizontal::Start,
        alignment: alignment::Horizontal::Start,
    }
}
