use std::{ffi::CString, num::NonZeroU32};

use gl::{types::*, *};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, GlProfile, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContext},
    surface::{Surface as GluSfc, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use goober_ui::skia_safe::{
    gpu::{
        backend_render_targets,
        gl::{Format, FramebufferInfo, Interface},
        surfaces::wrap_backend_render_target,
        DirectContext, SurfaceOrigin,
    },
    ColorType, Surface,
};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::Event,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Render {
    pub surface: Surface,
    gl_surface: GluSfc<WindowSurface>,
    gr_context: DirectContext,
    gl_context: PossiblyCurrentContext,
    pub window: Window,
    fb_info: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
}

impl Render {
    pub fn new(wb: WindowBuilder, ev: &EventLoop<()>) -> Self {
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(wb));

        let (window, gl_config) = display_builder
            .build(ev, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let mut window = window.expect("Could not create window with OpenGL context");

        let raw_window_handle = window.raw_window_handle();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(Some(raw_window_handle));

        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        let (width, height): (u32, u32) = window.inner_size().into();

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

        load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let mut gr_context =
            DirectContext::new_gl(Some(interface), None).expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { GetIntegerv(FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let mut surface = create_surface(
            &mut window,
            fb_info,
            &mut gr_context,
            num_samples,
            stencil_size,
        );

        let sf = window.scale_factor() as f32;
        surface.canvas().scale((sf, sf));

        Self {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            fb_info,
            num_samples,
            stencil_size,
            window,
        }
    }
}

/// Create the surface for Skia to render in
fn create_surface(
    window: &mut Window,
    fb_info: FramebufferInfo,
    gr_context: &mut DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);
    wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}
