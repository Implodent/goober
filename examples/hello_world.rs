use goober::prelude::*;

fn main() -> Result<(), LaunchError> {
    launch(app)
}

fn app() -> impl View {
    // A simple Hello world.
    text("Hello world!").font_size(50.0)
}
