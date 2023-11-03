use goober_ui::{
    runtime::{as_child_of_current_owner, create_render_effect, create_runtime, run_as_child},
    skia_safe::IPoint,
    *,
};
use winit::{
    error::EventLoopError,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod renderer;

pub fn launch<V: View>(make: impl Fn() -> V + 'static) -> Result<(), EventLoopError> {
    let _rt = create_runtime();
    let (root, disposer) = as_child_of_current_owner(|()| make())(());

    let event_loop = EventLoop::new()?;

    let mut ren = renderer::Render::new(
        WindowBuilder::new().with_visible(true).with_title("yes"),
        &event_loop,
    );

    event_loop.run(move |event, explode| {
        println!("{event:?}");

        match event {
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
            }
            | Event::UserEvent(()) => root.render(
                &ren.surface.canvas(),
                &RenderContext {
                    offset: IPoint::new(0, 0),
                },
            ),
            _ => {}
        }
    })
}
