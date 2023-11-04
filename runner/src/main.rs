pub mod renderer;

use goober_runner::launch;
use goober_ui::{
    modifier::*,
    runtime::*,
    skia_safe::{Color, Font, FontStyle, Typeface},
    text::*,
    View,
};

fn main() -> Result<(), winit::error::EventLoopError> {
    launch(app)
}

fn app() -> impl View {
    let (read, write) = create_signal(0);
    let (hovering, set_hover) = create_signal(false);

    text(move || format!("Counter: {} (hovering: {})", read.get(), hovering.get()))
        .font(Font::from_typeface(
            Typeface::new("JetBrainsMono Nerd Font Mono", FontStyle::normal())
                .expect("font unavailable"),
            40.0,
        ))
        .background(Color::BLUE)
        .padding(20)
        .on_click(move |button| {
            write.update(|x| {
                *x = match button {
                    goober_ui::MouseButton::Left => *x + 1,
                    goober_ui::MouseButton::Right => *x - 1,
                    _ => return,
                }
            })
        })
        .hovering(set_hover)
}
