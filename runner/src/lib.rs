use std::rc::Rc;
use taffy::prelude::*;

use glutin::surface::GlSurface;
use goober_runtime::{
    as_child_of_current_owner, create_effect, create_render_effect, create_runtime, create_trigger,
    store_value, with_owner, Owner,
};
use goober_ui::{skia_safe::Color, *};

use winit::{
    event::{ElementState, Event, MouseButton as WinitMouseButton, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "skia")]
    EventLoop(winit::error::EventLoopError),
    Io(std::io::Error),
}

#[cfg(feature = "skia")]
impl From<winit::error::EventLoopError> for Error {
    fn from(value: winit::error::EventLoopError) -> Self {
        Self::EventLoop(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn launch<V: View + 'static>(make: impl Fn() -> V + 'static) -> Result<(), Error> {
    let _rt = create_runtime();
    let (root, _disposer) = as_child_of_current_owner(|()| Rc::new(make()))(());

    let event_loop = EventLoop::new()?;

    let render_trigger = create_trigger();

    let ren = store_value(renderer::Render::new(
        WindowBuilder::new().with_visible(true).with_title("yes"),
        &event_loop,
    ));
    let density = Density(ren.with_value(|x| x.window.scale_factor() as f32));

    let (node, taffy) = {
        let mut tf = Taffy::new();
        let node = root.measure(None, &mut tf);
        ren.with_value(|ren| {
            let size = ren.window.inner_size();
            tf.compute_layout(
                node,
                Size {
                    width: AvailableSpace::Definite(size.width as f32),
                    height: AvailableSpace::Definite(size.height as f32),
                },
            )
            .unwrap();
        });
        (node, store_value(tf))
    };

    create_effect({
        let root = root.clone();
        move |_| {
            taffy.update_value(|tf| {
                root.measure(Some(node), tf);
                if tf.dirty(node).unwrap() {
                    ren.with_value(|ren| {
                        let size = ren.window.inner_size();
                        tf.compute_layout(
                            node,
                            Size {
                                width: AvailableSpace::Definite(size.width as f32),
                                height: AvailableSpace::Definite(size.height as f32),
                            },
                        )
                        .unwrap();
                        render_trigger.notify();
                    });
                }
            });
        }
    });

    create_render_effect({
        let root = root.clone();
        move |_| {
            render_trigger.track();
            ren.update_value(|ren| {
                let canvas = ren.surface.canvas();
                canvas.clear(Color::WHITE);

                taffy.with_value(|taffy| {
                    let layout = *taffy.layout(node).unwrap();
                    root.render(
                        canvas,
                        &RenderContext {
                            taffy,
                            layout,
                            this_node: node,
                            density,
                            #[cfg(feature = "terminal")]
                            is_terminal: false,
                        },
                    );
                });

                ren.gr_context.flush_and_submit();
                ren.gl_surface.swap_buffers(&ren.gl_context).unwrap();
            })
        }
    });

    event_loop
        .run({
            let owner = Owner::current().expect("owner exploded");
            let mut last_mouse = Point::ZERO;
            move |event, explode| match event {
                winit::event::Event::NewEvents(StartCause::Init) => {
                    explode.set_control_flow(ControlFlow::Wait);
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => explode.exit(),
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => with_owner(owner, || render_trigger.notify()),
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => ren.update_value(|ren| ren.resize(size)),
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            button,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => with_owner(owner, || {
                    taffy.with_value(|taffy| {
                        let layout = *taffy.layout(node).unwrap();
                        root.ev(
                            &goober_ui::Event::Click(
                                last_mouse,
                                match button {
                                    WinitMouseButton::Left => MouseButton::Left,
                                    WinitMouseButton::Right => MouseButton::Right,
                                    WinitMouseButton::Back => MouseButton::Back,
                                    WinitMouseButton::Forward => MouseButton::Forward,
                                    WinitMouseButton::Middle => MouseButton::Middle,
                                    WinitMouseButton::Other(other) => MouseButton::Other(other),
                                },
                            ),
                            &RenderContext {
                                taffy,
                                layout,
                                this_node: node,
                                density,
                                #[cfg(feature = "terminal")]
                                is_terminal: false,
                            },
                        )
                    })
                }),
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    last_mouse = Point {
                        x: position.x as f32,
                        y: position.y as f32,
                    };
                    with_owner(owner, || {
                        taffy.with_value(|taffy| {
                            let layout = *taffy.layout(node).unwrap();
                            root.ev(
                                &goober_ui::Event::CursorMove(last_mouse),
                                &RenderContext {
                                    taffy,
                                    layout,
                                    this_node: node,
                                    density,
                                    #[cfg(feature = "terminal")]
                                    is_terminal: false,
                                },
                            )
                        })
                    })
                }
                _ => {}
            }
        })
        .map_err(Into::into)
}

pub fn launch_terminal_or_winit<V: View + 'static>(
    make: impl Fn() -> V + 'static,
) -> Result<(), Error> {
    #[cfg(feature = "terminal")]
    if std::env::var("GOOBER_LAUNCH_ENV").is_ok_and(|x| x == "terminal") {
        launch_terminal(make)
    } else {
        launch(make)
    }

    #[cfg(not(feature = "terminal"))]
    launch(make)
}

#[cfg(feature = "terminal")]
pub fn launch_terminal<V: View + 'static>(make: impl Fn() -> V + 'static) -> Result<(), Error> {
    let _tokio = tokio::runtime::Builder::new_current_thread()
        .build()?
        .enter();

    launch_term(make)
}
#[cfg(feature = "terminal")]
fn launch_term<V: View + 'static>(make: impl Fn() -> V + 'static) -> Result<(), Error> {
    use std::io::stdout;

    let _rt = create_runtime();
    let owner = Owner::current().expect("owner exploded");
    let (root, _disposer) = as_child_of_current_owner(|()| Rc::new(make()))(());

    let render_trigger = create_trigger();
    let ren = store_value(Terminal::new(stdout()));
    ren.update_value(|ren| ren.execute(EnableMouseCapture).unwrap());

    let density = Density(1.0);

    let (node, taffy) = {
        let mut tf = Taffy::new();
        let node = with_owner(owner, || root.measure_terminal(None, &mut tf));
        let (width, height) = crossterm::terminal::size()?;
        tf.compute_layout(
            node,
            Size {
                width: AvailableSpace::Definite(width as f32),
                height: AvailableSpace::Definite(height as f32),
            },
        )
        .unwrap();
        (node, store_value(tf))
    };

    create_effect({
        let root = root.clone();
        move |_| {
            taffy.update_value(|tf| {
                with_owner(owner, || root.measure_terminal(None, tf));
                if tf.dirty(node).unwrap() {
                    let (width, height) = crossterm::terminal::size().unwrap();
                    tf.compute_layout(
                        node,
                        Size {
                            width: AvailableSpace::Definite(width as f32),
                            height: AvailableSpace::Definite(height as f32),
                        },
                    )
                    .unwrap();
                    render_trigger.notify();
                }
            });
        }
    });

    create_render_effect({
        let root = root.clone();
        move |_| {
            render_trigger.track();
            ren.update_value(|ren| {
                taffy.with_value(|taffy| {
                    ren.execute(Clear(ClearType::All)).unwrap();
                    let layout = *taffy.layout(node).unwrap();
                    root.render_terminal(
                        ren,
                        &RenderContext {
                            taffy,
                            layout,
                            this_node: node,
                            density,
                            is_terminal: true,
                        },
                    )
                    .unwrap();
                    ren.flush().unwrap();
                });
            })
        }
    });

    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{Clear, ClearType},
    };
    use goober_runtime::{create_signal_from_stream, SignalWith};
    let stream = create_signal_from_stream(crossterm::event::EventStream::new());

    create_effect(move |_| {
        let event = stream.with(|x| x.as_ref().map(|x| x.as_ref().unwrap().clone()))?;
        println!("{event:?}");
        taffy.update_value(|taffy| {
            with_owner(owner, || {
                root.ev(
                    &goober_ui::Event::Terminal(event.clone()),
                    &RenderContext {
                        taffy,
                        layout: *taffy.layout(node).unwrap(),
                        this_node: node,
                        density,
                        is_terminal: true,
                    },
                );
            })
        });
        Some(())
    });

    ren.update_value(|ren| ren.execute(DisableMouseCapture).unwrap());

    Ok(())
}
