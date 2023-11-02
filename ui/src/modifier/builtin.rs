use super::*;
use crate::foundation::alignment::Alignment;
use crate::render::*;
use crate::unit::*;

pub struct Background(pub Paint);

impl Modifier for Background {
    fn render(
        &self,
        view: &dyn View,
        renderer: &mut dyn crate::render::Renderer,
        context: &crate::render::RenderContext,
    ) {
        let size = view.size();

        renderer.draw_rect(IRect::from_pt_size(context.position, size), &self.0);
    }
}

pub struct Align(pub Alignment);

impl Modifier for Align {
    fn render(
        &self,
        view: &dyn View,
        renderer: &mut dyn crate::render::Renderer,
        context: &crate::render::RenderContext,
    ) {
        let size = view.size();
        let canvas_size = renderer.size();

        view.render(
            renderer,
            &RenderContext {
                position: context.position
                    + self.0.align(
                        size,
                        ISize {
                            width: canvas_size.width - size.width,
                            height: canvas_size.height - size.height,
                        },
                    ),
                density: context.density,
            },
        )
    }
}
