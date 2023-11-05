use goober::prelude::*;

fn main() -> Result<(), LaunchError> {
    launch(app)
}

fn app() -> impl View {
    stack_x((text("hello"), text("world")))
        .arrange(arrangement::BuiltinHorizontal::SpacedBy(10.dp()))
        .align(alignment::Horizontal::End)
}
