use super::*;
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
