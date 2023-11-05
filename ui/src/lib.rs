pub mod alignment;
pub mod arrangement;
pub mod button;
pub mod canvas;
pub mod modifier;
mod sk;
pub mod stacking;
pub use sk::*;
pub mod text;

pub use skia_safe;
use skia_safe::{Canvas, Contains, Font, IPoint, IRect, ISize, Paint, Point};

#[derive(Copy, Clone, Debug)]
pub struct RenderContext {
    pub offset: IPoint,
    pub constraints: Constraints,
    pub space: ISize,
    pub density: Density,
}

impl Into<MeasureContext> for RenderContext {
    fn into(self) -> MeasureContext {
        MeasureContext {
            offset: self.offset,
            constraints: self.constraints,
            space: self.space,
            density: self.density,
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
    pub space: ISize,
    pub density: Density,
}

#[derive(Copy, Clone, Debug)]
pub struct MeasureResult {
    pub rect: IRect,
    pub advance_width: i32,
    pub advance_height: i32,
}

impl MeasureResult {
    pub fn new(rect: IRect) -> Self {
        Self {
            rect,
            advance_height: 0,
            advance_width: 0,
        }
    }
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

pub trait Views {
    fn iter(&self) -> Box<dyn Iterator<Item = &'_ (dyn View + '_)> + '_>;
}

impl Views for Vec<Box<dyn View>> {
    fn iter(&self) -> Box<dyn Iterator<Item = &'_ (dyn View + '_)> + '_> {
        Box::new(<[Box<dyn View>]>::iter(self.as_ref()).map(|x| x.as_ref()))
    }
}

impl<const N: usize> Views for [Box<dyn View>; N] {
    fn iter(&self) -> Box<dyn Iterator<Item = &'_ (dyn View + '_)> + '_> {
        Box::new(<[Box<dyn View>]>::iter(self).map(|x| x.as_ref()))
    }
}

macro_rules! views_for_tuple {
    () => {};
    ($head:ident $($X:ident) *) => {
        views_for_tuple!($($X)*);
        views_for_tuple!(~ $head $($X)*);
    };
    (~ $Head:ident $($X:ident)*) => {
        #[allow(non_snake_case)]
        impl<$Head: View, $($X: View),*> Views for ($Head, $($X,)*) {
            fn iter(&self) -> Box<dyn Iterator<Item = &'_ (dyn View + '_)> + '_> {
                let ($Head, $($X,)*) = self;

                Box::new([$Head as &'_ (dyn View + '_), $($X as &'_ (dyn View + '_),)*].into_iter())
            }
        }
    };
}

views_for_tuple!(A_ B_ C_ D_ E_ F_ G_ H_ I_ J_ K_ L_ M_ N_ O_ P_ Q_ R_ S_ T_ U_ V_ W_ X_ Y_ Z_);

/// Density specifies how much pixels are in a [`Dp`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Density(pub f32);

impl Density {
    pub fn pixels(&self, dp: Dp) -> f32 {
        dp.0 * self.0
    }

    pub fn round_to_pixels(&self, dp: Dp) -> i32 {
        let px = self.pixels(dp);

        px.round() as i32
    }

    // amazing
    pub fn dp(&self, pixels: impl IntoDp) -> Dp {
        (pixels.dp().0 / self.0).dp()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Dp(pub f32);

impl Dp {
    pub const ZERO: Self = Self(0f32);
}

pub trait IntoDp {
    fn dp(self) -> Dp;
}

impl IntoDp for f32 {
    #[inline(always)]
    fn dp(self) -> Dp {
        Dp(self)
    }
}

impl IntoDp for i32 {
    fn dp(self) -> Dp {
        Dp(self as f32)
    }
}
