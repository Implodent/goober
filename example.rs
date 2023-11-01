pub fn component() -> impl View {
    stack_x((text("AAAAAA"), text("HHHHHH")))
}

pub fn counter() -> impl View {
    let (read, write) = create_signal(0);

    stack_x((
        text(format!("Count: {}", read.get())),
        button(|_| write.update(|x| x += 1), || text("+1")),
    ))
}
