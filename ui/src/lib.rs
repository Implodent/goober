use std::ops::Add;

use taffy::{axis::AbsoluteAxis, prelude::*};
pub type Point<T = f32> = taffy::geometry::Point<T>;
pub mod alignment;
pub mod arrangement;
pub mod button;
pub mod canvas;
pub mod modifier;
#[cfg(feature = "skia")]
mod sk;
pub mod stacking;
#[cfg(feature = "terminal")]
pub mod terminal;
pub use sk::*;
#[cfg(feature = "terminal")]
pub use terminal::Terminal;
pub mod text;

#[cfg(feature = "skia")]
pub use skia_safe;
#[cfg(all(feature = "skia", not(feature = "terminal")))]
use skia_safe::Paint;
#[cfg(feature = "skia")]
use skia_safe::{Canvas, Font};

#[cfg(all(feature = "skia", feature = "terminal"))]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Paint {
    pub terminal: terminal::Paint,
    pub skia: skia_safe::Paint,
}

#[cfg(all(feature = "skia", feature = "terminal"))]
use skia_safe::Color as SkColor;

#[cfg(all(feature = "skia", feature = "terminal"))]
impl Paint {
    pub fn new<'a>(color: skia_safe::Color4f, color_space: impl Into<Option<&'a skia_safe::ColorSpace>>) -> Self {
        Self::from(skia_safe::Paint::new(color, color_space))
    }
}

#[cfg(all(feature = "skia", feature = "terminal"))]
impl AsRef<terminal::Paint> for Paint {
    fn as_ref(&self) -> &terminal::Paint {
        &self.terminal
    }
}

#[cfg(all(feature = "skia", feature = "terminal"))]
impl AsRef<skia_safe::Paint> for Paint {
    fn as_ref(&self) -> &skia_safe::Paint {
        &self.skia
    }
}

#[cfg(all(feature = "skia", feature = "terminal"))]
impl From<skia_safe::Paint> for Paint {
    fn from(skia: skia_safe::Paint) -> Self {
        use crossterm::style::*;
        use skia_safe::paint::Style;
        let color = match skia.color() {
            SkColor::RED => Some(Color::Red),
            SkColor::GRAY => Some(Color::Grey),
            SkColor::BLACK => Some(Color::Black),
            SkColor::WHITE => Some(Color::White),
            SkColor::BLUE => Some(Color::Blue),
            SkColor::GREEN => Some(Color::Green),
            SkColor::YELLOW => Some(Color::Yellow),
            SkColor::MAGENTA => Some(Color::Magenta),
            SkColor::CYAN => Some(Color::Cyan),
            SkColor::DARK_GRAY => Some(Color::DarkGrey),
            SkColor::TRANSPARENT => None,
            color => Some(Color::Rgb {
                r: color.r(),
                g: color.g(),
                b: color.b(),
            }),
        };
        Self {
            terminal: terminal::Paint {
                attributes: Attributes::default(),
                foreground_color: if let Style::StrokeAndFill
                | Style::Stroke = skia.style()
                {
                    color
                } else {
                    None
                },
                background_color: if let Style::StrokeAndFill | Style::Fill =
                    skia.style()
                {
                    color
                } else {
                    None
                },
                underline_color: None,
            },
            skia,
        }
    }
}

pub trait Apply {
    fn apply(self, applicant: impl FnOnce(&Self)) -> Self;
    fn apply_mut(self, applicant: impl FnOnce(&mut Self)) -> Self;
}

impl<T> Apply for T {
    fn apply(self, applicant: impl FnOnce(&Self)) -> Self {
        applicant(&self);
        self
    }
    fn apply_mut(mut self, applicant: impl FnOnce(&mut Self)) -> Self {
        applicant(&mut self);
        self
    }
}

#[doc(hidden)]
#[cfg(feature = "skia")]
pub trait Tf: Sized {
    type Sk;

    fn from_sk(sk: Self::Sk) -> Self;
    fn into_sk(self) -> Self::Sk;
    fn into_sk_dp(self, _density: Density) -> Self::Sk {
        self.into_sk()
    }
}

#[cfg(feature = "skia")]
impl Tf for taffy::geometry::Point<f32> {
    type Sk = skia_safe::Point;

    fn from_sk(sk: skia_safe::Point) -> Self {
        Self { x: sk.x, y: sk.y }
    }

    fn into_sk(self) -> skia_safe::Point {
        skia_safe::Point {
            x: self.x,
            y: self.y,
        }
    }

    fn into_sk_dp(self, density: Density) -> Self::Sk {
        skia_safe::Point {
            x: density.pixels(Dp(self.x)),
            y: density.pixels(Dp(self.y)),
        }
    }
}

#[cfg(feature = "skia")]
impl Tf for taffy::geometry::Size<f32> {
    type Sk = skia_safe::Size;
    fn from_sk(sk: Self::Sk) -> Self {
        Self {
            width: sk.width,
            height: sk.height,
        }
    }
    fn into_sk(self) -> Self::Sk {
        skia_safe::Size {
            width: self.width,
            height: self.height,
        }
    }
    fn into_sk_dp(self, density: Density) -> Self::Sk {
        skia_safe::Size {
            width: density.pixels(Dp(self.width)),
            height: density.pixels(Dp(self.height)),
        }
    }
}

#[cfg(feature = "skia")]
impl Tf for taffy::geometry::Rect<f32> {
    type Sk = skia_safe::Rect;
    fn from_sk(sk: Self::Sk) -> Self {
        Self {
            left: sk.left,
            top: sk.top,
            right: sk.right,
            bottom: sk.bottom,
        }
    }
    fn into_sk(self) -> Self::Sk {
        skia_safe::Rect {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
    fn into_sk_dp(self, density: Density) -> Self::Sk {
        skia_safe::Rect {
            left: density.pixels(Dp(self.left)),
            top: density.pixels(Dp(self.top)),
            right: density.pixels(Dp(self.right)),
            bottom: density.pixels(Dp(self.bottom)),
        }
    }
}

#[derive(Copy, Clone)]
pub struct RenderContext<'a> {
    pub density: Density,
    pub layout: Layout,
    pub taffy: &'a Taffy,
    pub this_node: Node,
    #[cfg(feature = "terminal")]
    pub is_terminal: bool
}

#[derive(Clone, Debug)]
pub enum Event {
    Click(Point, MouseButton),
    CursorMove(Point),
    #[cfg(feature = "terminal")]
    Terminal(crossterm::event::Event),
}

pub fn taffy_rect<T: Add<T, Output = T> + Copy>(size: Size<T>, point: Point<T>) -> Rect<T> {
    Rect {
        left: point.x,
        right: point.x + size.width,
        top: point.y + size.height,
        bottom: point.y,
    }
}

pub fn taffy_rect_contains<T: PartialOrd<T> + Copy>(rect: &Rect<T>, point: &Point<T>) -> bool {
    (rect.left..rect.right).contains(&point.x) && (rect.top..rect.bottom).contains(&point.y)
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

pub trait View {
    #[doc(hidden)]
    fn style(&self) -> Style;
    #[doc(hidden)]
    #[cfg(feature = "terminal")]
    fn style_terminal(&self) -> Style {
        self.style()
    }
    #[doc(hidden)]
    /// Measure this view and its children (if available).
    /// A current node ID is optionally provided, together with a mutable reference to the Taffy
    /// tree.
    fn measure(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        if let Some(current_node) = current_node {
            taffy.set_style(current_node, self.style()).unwrap();
            current_node
        } else {
            taffy.new_leaf(self.style()).unwrap()
        }
    }
    #[doc(hidden)]
    #[cfg(feature = "terminal")]
    fn measure_terminal(&self, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        self.measure(current_node, taffy)
    }
    #[doc(hidden)]
    #[cfg(feature = "skia")]
    /// Render this view, with the provided canvas, and provided render context.
    fn render(&self, canvas: &Canvas, how: &RenderContext);
    #[doc(hidden)]
    #[cfg(feature = "terminal")]
    fn render_terminal(
        &self,
        renderer: &mut Terminal,
        how: &RenderContext,
    ) -> Result<(), std::io::Error>;
    #[doc(hidden)]
    #[inline(always)]
    /// Handle an event.
    /// If your component has children, be sure to forward that event to them, unless it is not desired.
    fn ev(&self, _event: &Event, _how: &RenderContext) {}
}

pub trait Modifier {
    #[doc(hidden)]
    #[inline(always)]
    /// Intercept styling said `view`.
    fn style(&self, view: &dyn View) -> Style {
        view.style()
    }
    #[doc(hidden)]
    #[inline(always)]
    /// Intercept measuring said `view`.
    fn measure(&self, view: &dyn View, current_node: Option<Node>, taffy: &mut Taffy) -> Node {
        let node = view.measure(current_node, taffy);

        taffy.set_style(node, self.style(view)).unwrap();

        node
    }
    #[doc(hidden)]
    #[inline(always)]
    /// Intercept an event. This function is called when the view (with an applied modifier) receives an event.
    /// Be sure to let the view know about the event too, unless it is not desired.
    /// The default implementation just forwards the event to the view.
    fn ev(&self, view: &dyn View, event: &Event, how: &RenderContext) {
        view.ev(event, how)
    }
    /// Modify the rendering of the view.
    /// Usually, with modifiers that do not need very precise control over the rendering could just do their thing and let the view render the rest.
    /// For those who need precise control, however, creating a Canvas is always feasible.
    /// You create one, let the view draw on it, and do your magical modifier things.
    #[doc(hidden)]
    #[inline(always)]
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

use derive_more::*;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Add, Sub, Mul, Div, Default)]
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
