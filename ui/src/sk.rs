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

pub trait IntoRect<T> {
    fn into_rect(self) -> Rect<T>;
}

impl<T> IntoRect<T> for Rect<T> {
    fn into_rect(self) -> Rect<T> {
        self
    }
}

impl<T: TaffyZero> IntoRect<T> for (T, T) {
    fn into_rect(self) -> Rect<T> {
        Rect {
            left: self.0,
            right: T::ZERO,
            top: self.1,
            bottom: T::ZERO,
        }
    }
}

impl<T> IntoRect<T> for (T, T, T, T) {
    fn into_rect(self) -> Rect<T> {
        Rect {
            left: self.0,
            right: self.1,
            top: self.2,
            bottom: self.3,
        }
    }
}
