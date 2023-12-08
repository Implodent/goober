use goober::prelude::*;

fn main() -> Result<(), LaunchError> {
    launch_terminal_or_winit(app)
}

fn app() -> impl View {
    let (counter, counter_set) = create_signal(0);

    text(move || Oco::Owned(format!("Counter: {}", counter.get())))
        // set the font's size to be a little bigger
        .font_size(50.0)
        // bad attempt at making it look like a button ;)
        .background(Color::new(0xffaaaaaa))
        .on_click(move |button| {
            counter_set.update(|x| {
                *x = match button {
                    goober_ui::MouseButton::Left => *x + 1,
                    goober_ui::MouseButton::Right => *x - 1,
                    _ => return,
                }
            })
        }).padding((LengthPercentage::Points(1.0), LengthPercentage::Points(1.0)))
}
