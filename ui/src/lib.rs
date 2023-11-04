pub mod button;
pub mod modifier;
mod sk;
pub use sk::*;
pub mod text;

pub use goober_runtime as runtime;
pub use skia_safe;
use skia_safe::{Canvas, Contains, Font, IPoint, IRect, ISize, Paint, Point};

#[derive(Copy, Clone, Debug)]
pub struct RenderContext {
    pub offset: IPoint,
    pub constraints: Constraints,
}

impl Into<MeasureContext> for RenderContext {
    fn into(self) -> MeasureContext {
        MeasureContext {
            offset: self.offset,
            constraints: self.constraints,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Click(Point, MouseButton),
    CursorMove(Point),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    Other(u16),
}

#[derive(Copy, Clone, Debug)]
pub struct Constraints {
    pub min: ISize,
    pub max: ISize,
}

impl Constraints {
    pub fn clamp(&self, size: ISize) -> ISize {
        ISize {
            width: size.width.clamp(self.min.width, self.max.width),
            height: size.height.clamp(self.min.height, self.max.height),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MeasureContext {
    pub offset: IPoint,
    pub constraints: Constraints,
}

#[derive(Copy, Clone, Debug)]
pub struct MeasureResult {
    pub rect: IRect,
}

pub trait View {
    #[doc(hidden)]
    /// Measure this view.
    fn measure(&self, context: &MeasureContext) -> MeasureResult;
    #[doc(hidden)]
    /// Render this view, with the provided canvas, and provided render context.
    fn render(&self, canvas: &Canvas, how: &RenderContext);
    #[doc(hidden)]
    /// Handle an event.
    /// If your component has children, be sure to forward that event to them, unless it is not desired.
    fn ev(&self, _event: &Event, _how: &RenderContext) {}
}

pub trait Modifier {
    #[doc(hidden)]
    /// Intercept measuring said `view`.
    /// The default implementation doesn't modify the measurement result from the `view` and just returns it as-is.
    fn measure(&self, view: &dyn View, context: &MeasureContext) -> MeasureResult {
        view.measure(context)
    }
    #[doc(hidden)]
    /// Intercept an event. This function is called when the view (with an applied modifier) receives an event.
    /// Be sure to let the view know about the event too, unless it is not desired.
    /// The default implementation just forwards the event to the view.
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        view.ev(event, how)
    }
    /// Modify the rendering of the view.
    /// Usually, with modifiers that do not need very precise control over the rendering could just do their thing and let the view render the rest.
    /// For those who need precise control, however, creating a Canvas is always feasible. You create one, let the view draw on it, and do your magical modifier things.
    #[doc(hidden)]
    fn render(&self, view: &dyn View, canvas: &Canvas, how: &RenderContext) {
        view.render(canvas, how)
    }
}
