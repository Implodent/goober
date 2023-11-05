pub use goober_runner::{launch, Error as LaunchError};
pub use goober_ui as ui;

pub use goober_runtime as runtime;

pub mod prelude {
    use super::*;
    pub use super::{launch, LaunchError};
    pub use ui::skia_safe::{Color, Font, FontStyle, Typeface};
    pub use ui::{
        alignment, arrangement, button::button, modifier::ApplyModifier, skia_safe as skia,
        stacking::stack_x, text::text, IntoDp, Modifier, View,
    };

    pub use runtime::signal_prelude::*;
}
