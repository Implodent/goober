use goober_ui::{render::RenderContext, skia_safe, View};
use skia_safe::{
    gpu::{surfaces::render_target, Budgeted, DirectContext, SurfaceOrigin},
    ImageInfo,
};

pub fn gl(root: impl View) {
    let mut context = DirectContext::new_gl(None, None).unwrap();
    let image_info = ImageInfo::new_n32_premul(root.precalc_size(), None);
    let mut surface = render_target(
        &mut context,
        Budgeted::Yes,
        &image_info,
        None,
        SurfaceOrigin::BottomLeft,
        None,
        false,
        None,
    )
    .unwrap();

    root.render(
        &surface.canvas(),
        &RenderContext {
            position: (0, 0).into(),
        },
    )
}
