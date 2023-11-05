use super::*;

pub struct StackX<V, A> {
    views: V,
    arrangement: A,
    alignment: alignment::Horizontal,
}

impl<V: Views, A: arrangement::Horizontal> View for StackX<V, A> {
    fn ev(&self, event: &Event, how: &RenderContext) {
        self.views.iter().for_each(|x| x.ev(event, how));
    }
    fn measure(&self, context: &MeasureContext) -> MeasureResult {}
    fn render(&self, canvas: &Canvas, how: &RenderContext) {}
}
