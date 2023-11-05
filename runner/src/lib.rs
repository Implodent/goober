use std::rc::Rc;

use glutin::surface::GlSurface;
use goober_runtime::{
    as_child_of_current_owner, create_render_effect, create_runtime, create_trigger, store_value,
    with_owner, Owner,
};
use goober_ui::{
    skia_safe::{Color, IPoint, ISize, Point},
    *,
};

use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton as WinitMouseButton, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod renderer;
pub use winit::error::EventLoopError as Error;

pub fn launch<V: View + 'static>(make: impl Fn() -> V + 'static) -> Result<(), Error> {
    let _rt = create_runtime();
    let (root, _disposer) = as_child_of_current_owner(|()| Rc::new(make()))(());
    create_render_effect(|_| {});

    let event_loop = EventLoop::new()?;

    let ren = store_value(renderer::Render::new(
        WindowBuilder::new().with_visible(true).with_title("yes"),
        &event_loop,
    ));
    let render_trigger = create_trigger();

    create_render_effect({
        let root = root.clone();
        move |_| {
            render_trigger.track();
            ren.update_value(|ren| {
                let canvas = ren.surface.canvas();
                canvas.clear(Color::WHITE);

                root.render(canvas, &render_cx(&ren.window));

                ren.gr_context.flush_and_submit();
                ren.gl_surface.swap_buffers(&ren.gl_context).unwrap();
            })
        }
    });

    event_loop.run({
        let owner = Owner::current().expect("owner exploded");
        let mut last_mouse = Point::new(0.0, 0.0);
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
                    &ren.with_value(|ren| render_cx(&ren.window)),
                )
            }),
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                last_mouse = Point::new(position.x as f32, position.y as f32);
                with_owner(owner, || {
                    root.ev(
                        &goober_ui::Event::CursorMove(last_mouse),
                        &ren.with_value(|ren| render_cx(&ren.window)),
                    )
                })
            }
            _ => {}
        }
    })
}

fn constraints(window: &Window) -> Constraints {
    let size = window.inner_size();
    Constraints {
        min: ISize::new_empty(),
        max: ISize {
            height: size.height as i32,
            width: size.width as i32,
        },
    }
}

fn render_cx(window: &Window) -> RenderContext {
    let PhysicalSize { width, height } = window.inner_size();
    let (width, height): (i32, i32) = (width.try_into().unwrap(), height.try_into().unwrap());

    RenderContext {
        offset: IPoint::new(0, 0),
        constraints: constraints(window),
        space: ISize { width, height },
        density: Density(window.scale_factor() as f32),
    }
}
