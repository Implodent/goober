mod parse;
mod window;

use goober_ui::{
    render::RenderContext,
    runtime::{create_render_effect, create_runtime, create_rw_signal, SignalUpdateUntracked},
    skia_safe,
    unit::Density,
    View,
};
use window::{LaunchConfig, WindowEnv};
use winit::{event_loop::EventLoopBuilder, window::CursorIcon};

#[derive(Debug)]
pub enum EventMessage {
    RequestRerender,
    SetCursorIcon(CursorIcon),
}

pub fn launch<V: View + 'static>(make_root: impl FnOnce() -> V) {
    let runtime = create_runtime();
    let root = make_root();
    let config = LaunchConfig::<()>::builder().build();

    let evl = EventLoopBuilder::<EventMessage>::with_user_event()
        .build()
        .unwrap();
    let proxy = evl.create_proxy();

    let env = WindowEnv::from_config(config.window.clone(), &evl);

    runtime.dispose();
}
