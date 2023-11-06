use goober::prelude::*;
use goober_runtime::create_effect;

fn main() -> Result<(), LaunchError> {
    launch(bingo_app)
}

fn card(bg: Color, name: &'static str) -> impl View {
    let (active, set_active) = create_signal(false);

    create_effect(move |_| {
        println!("{name}: active={}", active.get());
    });

    text(name)
        .font_size(50.0)
        .on_click(move |_| set_active.update(|x| *x = !*x))
        .padding(30)
        .background::<Color>(create_memo(
            move |_| if active.get() { Color::GRAY } else { bg },
        ))
        .align(Alignment::CENTER)
}

fn bingo_app() -> impl View {
    stack_y((
        stack_x((card(Color::RED, "j"), card(Color::GREEN, "z"))),
        stack_x((card(Color::BLUE, "x"), card(Color::MAGENTA, "n"))),
    ))
}
