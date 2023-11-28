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
pub use winit::error::EventLoopError as Error;

pub fn launch<V: View + 'static>(make: impl Fn() -> V + 'static) -> Result<(), Error> {
    let _rt = create_runtime();
    let (root, _disposer) = as_child_of_current_owner(|()| Rc::new(make()))(());
    create_render_effect(|_| {});

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
                        },
                    );
                });

                ren.gr_context.flush_and_submit();
                ren.gl_surface.swap_buffers(&ren.gl_context).unwrap();
            })
        }
    });

    event_loop.run({
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
                            },
                        )
                    })
                })
            }
            _ => {}
        }
    })
}
