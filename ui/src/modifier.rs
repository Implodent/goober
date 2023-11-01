use std::any::Any;

use crate::View;

mod builtin;
pub use builtin::*;

pub trait Modifier: Any {
    fn render(
        &self,
        view: &dyn View,
        renderer: &mut dyn crate::render::Renderer,
        context: &crate::render::RenderContext,
    ) {
        view.render(renderer, context);
    }

    fn size(&self, view: &dyn View) -> crate::unit::ISize {
        view.size()
    }
}

pub struct Modified<V: View, M: Modifier> {
    view: V,
    modifier: M,
}

impl<V: View, M: Modifier> View for Modified<V, M> {
    fn modifiers(&self) -> Option<Box<dyn Iterator<Item = &'_ (dyn Modifier + '_)>>> {
        Some(Box::new(
            self.view
                .modifiers()
                .into_iter()
                .flatten()
                .chain(std::iter::once(&self.modifier as &dyn Modifier)),
        ))
    }

    fn render(
        &self,
        renderer: &mut dyn crate::render::Renderer,
        context: &crate::render::RenderContext,
    ) {
        self.modifier.render(&self.view, renderer, context)
    }

    fn size(&self) -> crate::unit::ISize {
        self.modifier.size(&self.view)
    }
}
