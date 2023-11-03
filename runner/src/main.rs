pub mod renderer;

use goober_runner::launch;
use goober_ui::{
    runtime::{create_signal, Oco, SignalGet},
    skia_safe::{Color, Color4f, Font, Paint},
    Text, View,
};

fn main() -> Result<(), winit::error::EventLoopError> {
    launch(app)
}

fn app() -> impl View {
    let (read, _write) = create_signal(0);
    Text {
        text: move || Oco::Owned(format!("yey {}", read.get())),
        font: Font::default(),
        paint: Paint::new(Color4f::from(Color::RED), None),
    }
}
