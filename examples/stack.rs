use goober::prelude::*;
use goober_ui::{skia_safe::IRect, IntoPaint};

fn main() -> Result<(), LaunchError> {
    launch(app)
}

fn single_stack() -> impl View {
    stack_y((text("hello"), text("world")))
        .arrange(arrangement::BuiltinVertical::SpacedBy(10.dp()))
        .align(alignment::Vertical::Top)
}

fn multi_stack() -> impl View {
    stack_x((single_stack(), single_stack(), single_stack()))
        .arrange(arrangement::BuiltinHorizontal::SpacedBy(10.dp()))
        .align(alignment::Horizontal::Start)
}

fn rect(color: Color) -> impl View {
    let rect = IRect::from_ltrb(100, 100, 100, 100);

    with_canvas(
        move |canvas| {
            canvas.draw_irect(rect, &color.into_paint());
        },
        move |_| rect,
    )
}

fn app() -> impl View {
    stack_x((
        stack_z((
            multi_stack(),
            multi_stack().offset((10, 10)),
            multi_stack().offset((20, 20)),
        )),
        stack_z((rect(Color::RED), rect(Color::GREEN).offset((10, 10)))),
    ))
    .arrange(arrangement::BuiltinHorizontal::SpacedBy(30.dp()))
}
