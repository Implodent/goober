use super::*;

pub struct StackX<V, A> {
    views: V,
    arrangement: A,
    alignment: alignment::Horizontal,
}

impl<V, A> StackX<V, A> {
    pub fn alignment(self, alignment: alignment::Horizontal) -> Self {
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
    fn style(&self) -> Style {
        Style {
            justify_items: Some(self.arrangement.justify()),
            gap: Size {
                width: self.arrangement.spacing(),
                height: TaffyZero::ZERO,
            },
            flex_direction: FlexDirection::Row,
            ..Default::default()
        }
    }

    fn measure(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        if let Some(current_node) = current_node {
            // clueless
            taffy.set_style(current_node, self.style()).unwrap();

            current_node
        } else {
            let mut taffies = vec![];

            for child in self.views.iter() {
                taffies.push(child.measure(None, taffy));
            }

            taffy.new_with_children(self.style(), &taffies).unwrap()
        }
    }

    fn ev(&self, event: &Event, how: &RenderContext) {
        self.views.iter().enumerate().for_each(|(index, view)| {
            let child_node = how.taffy.child(how.this_node, index);
            view.ev(
                event,
                &RenderContext {
                    layout: *how.taffy.layout(child_node).unwrap(),
                    this_node: child_node,
                    ..*how
                },
            )
        });
    }

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.views.iter().enumerate().for_each(|(index, view)| {
            let child_node = how.taffy.child(how.this_node, index);
            view.render(
                canvas,
                &RenderContext {
                    layout: *how.taffy.layout(child_node).unwrap(),
                    this_node: child_node,
                    ..*how
                },
            )
        })
    }

    #[cfg(feature = "terminal")]
    fn render_terminal(
        &self,
        renderer: &mut Terminal,
        how: &RenderContext,
    ) -> Result<(), std::io::Error> {
        for (index, view) in self.views.iter().enumerate() {
            let child_node = how.taffy.child(how.this_node, index);
            view.render_terminal(
                renderer,
                &RenderContext {
                    layout: *how.taffy.layout(child_node).unwrap(),
                    this_node: child_node,
                    ..*how
                },
            )?;
        }

        Ok(())
    }
}

pub fn stack_x<V: Views>(views: V) -> StackX<V, arrangement::BuiltinHorizontal> {
    StackX {
        views,
        arrangement: arrangement::BuiltinHorizontal::Start,
        alignment: alignment::Horizontal::Start,
    }
}

pub struct StackY<V, A> {
    views: V,
    arrangement: A,
    alignment: alignment::Vertical,
}

impl<V, A> StackY<V, A> {
    pub fn alignment(self, alignment: alignment::Vertical) -> Self {
        Self { alignment, ..self }
    }
    pub fn arrange<A2>(self, arrangement: A2) -> StackY<V, A2> {
        StackY {
            views: self.views,
            arrangement,
            alignment: self.alignment,
        }
    }
}

impl<V: Views, A: arrangement::Vertical> View for StackY<V, A> {
    fn style(&self) -> Style {
        Style {
            flex_direction: FlexDirection::Column,
            justify_items: Some(self.arrangement.align()),
            gap: Size {
                height: self.arrangement.spacing(),
                width: TaffyZero::ZERO,
            },
            ..Default::default()
        }
    }

    fn measure(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        if let Some(current_node) = current_node {
            // clueless
            taffy.set_style(current_node, self.style()).unwrap();

            current_node
        } else {
            let mut taffies = vec![];

            for child in self.views.iter() {
                taffies.push(child.measure(None, taffy));
            }

            taffy.new_with_children(self.style(), &taffies).unwrap()
        }
    }

    fn ev(&self, event: &Event, how: &RenderContext) {
        self.views.iter().enumerate().for_each(|(index, view)| {
            let child_node = how.taffy.child(how.this_node, index);
            view.ev(
                event,
                &RenderContext {
                    layout: *how.taffy.layout(child_node).unwrap(),
                    this_node: child_node,
                    ..*how
                },
            )
        });
    }

    fn render(&self, canvas: &Canvas, how: &RenderContext) {
        self.views.iter().enumerate().for_each(|(index, view)| {
            let child_node = how.taffy.child(how.this_node, index);
            view.render(
                canvas,
                &RenderContext {
                    layout: *how.taffy.layout(child_node).unwrap(),
                    this_node: child_node,
                    ..*how
                },
            )
        })
    }

    #[cfg(feature = "terminal")]
    fn render_terminal(
        &self,
        renderer: &mut Terminal,
        how: &RenderContext,
    ) -> Result<(), std::io::Error> {
        for (index, view) in self.views.iter().enumerate() {
            let child_node = how.taffy.child(how.this_node, index);
            view.render_terminal(
                renderer,
                &RenderContext {
                    layout: *how.taffy.layout(child_node).unwrap(),
                    this_node: child_node,
                    ..*how
                },
            )?;
        }

        Ok(())
    }
}

pub fn stack_y<V: Views>(views: V) -> StackY<V, arrangement::BuiltinVertical> {
    StackY {
        views,
        arrangement: arrangement::BuiltinVertical::Top,
        alignment: alignment::Vertical::Top,
    }
}
