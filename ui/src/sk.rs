use skia_safe::{Color, Color4f};

use super::*;

pub trait IntoPaint {
    fn into_paint(self) -> Paint;
}

impl IntoPaint for Color {
    fn into_paint(self) -> Paint {
        Paint::new(Color4f::from(self), None)
    }
}

impl IntoPaint for Color4f {
    fn into_paint(self) -> Paint {
        Paint::new(self, None)
    }
}

impl IntoPaint for Paint {
    fn into_paint(self) -> Paint {
        self
    }
}

pub trait IntoIRect {
    fn into_irect(self) -> IRect;
}

impl IntoIRect for i32 {
    fn into_irect(self) -> IRect {
        IRect::new(self, self, self, self)
    }
}

impl IntoIRect for (i32, i32) {
    fn into_irect(self) -> IRect {
        let (width, height) = self;
        IRect {
            left: width,
            top: height,
            right: width,
            bottom: height,
        }
    }
}

impl IntoIRect for (i32, i32, i32, i32) {
    fn into_irect(self) -> IRect {
        let (left, right, top, bottom) = self;
        IRect {
            left,
            right,
            top,
            bottom,
        }
    }
}

impl IntoIRect for IRect {
    fn into_irect(self) -> IRect {
        self
    }
}
