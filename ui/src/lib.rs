use modifier::Modifier;
#[cfg(feature = "skia")]
#[doc(hidden)]
pub use skia_safe;
pub mod foundation;
pub mod modifier;
pub mod render;
pub mod unit;

use foundation::text::Font;
pub use goober_runtime as runtime;
use goober_runtime::Oco;
use render::*;
use unit::*;

pub trait View {
    fn modifiers(&self) -> Option<Box<dyn Iterator<Item = &'_ (dyn Modifier + '_)>>> {
        None
    }
    fn size(&self) -> unit::ISize;
    fn render(&self, renderer: &mut dyn Renderer, context: &RenderContext);

    fn boxed<'a>(self) -> Box<dyn View + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self) as Box<dyn View + 'a>
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
