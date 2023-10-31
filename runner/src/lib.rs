use goober_ui::{
    render::RenderContext,
    runtime::{create_render_effect, create_runtime, create_rw_signal, SignalUpdateUntracked},
    skia_safe, View,
};
use skia_safe::{
    gpu::{surfaces::render_target, Budgeted, DirectContext, SurfaceOrigin},
    ImageInfo,
};

pub fn gl<V: View + 'static>(make_root: impl FnOnce() -> V) {
    let runtime = create_runtime();
    let root = make_root();
    let mut context = DirectContext::new_gl(None, None).unwrap();
    let image_info = ImageInfo::new_n32_premul(root.size(), None);
    let surface = create_rw_signal(
        render_target(
            &mut context,
            Budgeted::Yes,
            &image_info,
            None,
            SurfaceOrigin::BottomLeft,
            None,
            false,
            None,
        )
        .unwrap(),
    );

    create_render_effect(move |_| {
        surface.update_untracked(|surface| {
            root.render(
                surface,
                &RenderContext {
                    position: (0, 0).into(),
                },
            )
        });
    });

    runtime.dispose();
}
