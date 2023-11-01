use super::alignment;
use super::arrangement;
use super::*;

pub struct StackX<V: Views, A: arrangement::Horizontal> {
    views: V,
    alignment: alignment::Horizontal,
    arrangement: A,
}

impl<V: Views, A: arrangement::Horizontal> StackX<V, A> {
    pub fn alignment(self, alignment: alignment::Horizontal) -> Self {
        Self { alignment, ..self }
    }
    pub fn arrangement<A2: arrangement::Horizontal>(self, arrangement: A2) -> StackX<V, A2> {
        StackX {
            arrangement,
            alignment: self.alignment,
            views: self.views,
        }
    }
}

impl<V: Views, A: arrangement::Horizontal> View for StackX<V, A> {
    fn render(&self, renderer: &mut dyn Renderer, context: &RenderContext) {
        let views = self.views.iter().collect::<Vec<_>>();
        let sizes = views.iter().map(|x| x.size().width).collect::<Vec<_>>();
        let arranged = self.arrangement.arrange(
            context.density,
            renderer.size().width - context.position.x,
            sizes,
        );
        for (view, position) in views.into_iter().zip(arranged) {
            view.render(
                renderer,
                &RenderContext {
                    position: IPoint::new(context.position.x + position, context.position.y),
                    density: context.density,
                },
            )
        }
    }

    fn size(&self) -> unit::ISize {
        self.views
            .iter()
            .map(|x| x.size())
            .reduce(|mut a, b| {
                a.width += b.width;
                a.height = a.height.max(b.height);

                a
            })
            .unwrap_or_else(ISize::new_empty)
    }
}

pub fn stack_x<V: Views>(views: V) -> StackX<V, arrangement::BuiltinHorizontal> {
    StackX {
        views,
        alignment: alignment::Horizontal::Start,
        arrangement: arrangement::BuiltinHorizontal::Start,
    }
}
