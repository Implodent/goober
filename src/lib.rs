#[cfg(feature = "terminal")]
pub use goober_runner::launch_terminal;
pub use goober_runner::{launch, launch_terminal_or_winit, Error as LaunchError};
pub use goober_ui as ui;

pub use goober_runtime as runtime;

pub mod prelude {
    use super::*;
    #[cfg(feature = "terminal")]
    pub use goober_runner::launch_terminal;
    pub use goober_runner::{launch, launch_terminal_or_winit, Error as LaunchError};
    pub use ui::skia_safe::{Color, Font, FontStyle, IRect, Typeface};
    pub use ui::{
        alignment::{self, Alignment},
        arrangement,
        button::button,
        canvas::{rectangle, with_canvas},
        modifier::ApplyModifier,
        skia_safe as skia,
        stacking::{stack_x, stack_y},
        text::text,
        IntoDp, Modifier, MouseButton, View,
        LengthPercentage
    };

    pub use runtime::{oco::Oco, signal_prelude::*};
}
