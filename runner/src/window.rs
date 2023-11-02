use crate::{skia_safe::*, EventMessage};
use gl::{types::*, *};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, GlProfile, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContext},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use goober_ui::render::*;
use gpu::{
    backend_render_targets,
    gl::{Format, FramebufferInfo, Interface},
    surfaces::wrap_backend_render_target,
    DirectContext, SurfaceOrigin,
};
use raw_window_handle::HasRawWindowHandle;
use std::{ffi::CString, num::NonZeroU32, sync::Arc};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::parse::Parse;

pub struct WindowEnv<T: Clone> {
    surface: Surface,
    gl_surface: glutin::surface::Surface<WindowSurface>,
    gr_context: DirectContext,
    gl_context: PossiblyCurrentContext,
    pub(crate) window: Window,
    fb: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
    pub(crate) cfg: WindowConfig<T>,
}
impl<T: Clone> WindowEnv<T> {
    /// Create a Window environment from a set of configuration
    pub fn from_config(cfg: WindowConfig<T>, event_loop: &EventLoop<EventMessage>) -> Self {
        let mut window_builder = WindowBuilder::new()
            .with_visible(false)
            .with_title(cfg.title)
            .with_decorations(cfg.decorations)
            .with_transparent(cfg.transparent)
            .with_inner_size(LogicalSize::<f64>::new(cfg.width, cfg.height));

        if let Some(min_size) = cfg.min_width.zip(cfg.min_height) {
            window_builder = window_builder.with_min_inner_size(LogicalSize::<f64>::from(min_size))
        }

        if let Some(max_size) = cfg.max_width.zip(cfg.max_height) {
            window_builder = window_builder.with_max_inner_size(LogicalSize::<f64>::from(max_size))
        }

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(cfg.transparent);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
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

        let fb = {
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

        let mut surface =
            create_surface(&mut window, fb, &mut gr_context, num_samples, stencil_size);

        let sf = window.scale_factor() as f32;
        surface.canvas().scale((sf, sf));

        WindowEnv {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            fb,
            num_samples,
            stencil_size,
            window,
            cfg,
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

/// Configuration for a Window.
#[derive(Clone)]
pub struct WindowConfig<T: Clone> {
    /// Width of the Window.
    pub width: f64,
    /// Height of the window.
    pub height: f64,
    /// Minimum width of the Window.
    pub min_width: Option<f64>,
    /// Minimum height of the window.
    pub min_height: Option<f64>,
    /// Maximum width of the Window.
    pub max_width: Option<f64>,
    /// Maximum height of the window.
    pub max_height: Option<f64>,
    /// Enable Window decorations.
    pub decorations: bool,
    /// Title for the Window.
    pub title: &'static str,
    /// Make the Window transparent or not.
    pub transparent: bool,
    /// A custom value to consume from your app.
    pub state: Option<T>,
    /// Background color of the Window.
    pub background: Color,
    /// Setup callback.
    pub on_setup: Option<WindowCallback>,
    /// Exit callback.
    pub on_exit: Option<WindowCallback>,
}

impl<T: Clone> Default for WindowConfig<T> {
    fn default() -> Self {
        LaunchConfigBuilder::default().build().window
    }
}

/// Launch configuration.
#[derive(Clone, Default)]
pub struct LaunchConfig<'a, T: Clone> {
    pub window: WindowConfig<T>,
    pub fonts: Vec<(&'a str, &'a [u8])>,
}

impl<'a, T: Clone> LaunchConfig<'a, T> {
    pub fn builder() -> LaunchConfigBuilder<'a, T> {
        LaunchConfigBuilder::default()
    }
}

pub type WindowCallback = Arc<Box<fn(&mut Window)>>;

/// Configuration Builder.
#[derive(Clone)]
pub struct LaunchConfigBuilder<'a, T> {
    pub width: f64,
    pub height: f64,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub decorations: bool,
    pub title: &'static str,
    pub transparent: bool,
    pub state: Option<T>,
    pub background: Color,
    pub fonts: Vec<(&'a str, &'a [u8])>,
    pub on_setup: Option<WindowCallback>,
    pub on_exit: Option<WindowCallback>,
}

impl<T> Default for LaunchConfigBuilder<'_, T> {
    fn default() -> Self {
        Self {
            width: 600.0,
            height: 600.0,
            min_width: None,
            min_height: None,
            max_height: None,
            max_width: None,
            decorations: true,
            title: "Freya app",
            transparent: false,
            state: None,
            background: Color::WHITE,
            fonts: Vec::default(),
            on_setup: None,
            on_exit: None,
        }
    }
}

impl<'a, T: Clone> LaunchConfigBuilder<'a, T> {
    /// Specify a Window width.
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Specify a Window height.
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Specify a minimum Window width.
    pub fn with_min_width(mut self, min_width: f64) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Specify a minimum Window height.
    pub fn with_min_height(mut self, min_height: f64) -> Self {
        self.min_height = Some(min_height);
        self
    }

    /// Specify a maximum Window width.
    pub fn with_max_width(mut self, max_width: f64) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Specify a maximum Window height.
    pub fn with_max_height(mut self, max_height: f64) -> Self {
        self.max_height = Some(max_height);
        self
    }

    /// Whether the Window will have decorations or not.
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Specify the Window title.
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// Make the Window transparent or not.
    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.transparent = transparency;
        self
    }

    /// Pass a custom value that your app will consume.
    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
        self
    }

    /// Specify the Window background color.
    pub fn with_background(mut self, background: &str) -> Self {
        self.background = Color::parse(background).unwrap_or(Color::WHITE);
        self
    }

    /// Register a font.
    pub fn with_font(mut self, font_name: &'a str, font: &'a [u8]) -> Self {
        self.fonts.push((font_name, font));
        self
    }

    /// Register a callback that will be executed when the window is created.
    pub fn on_setup(mut self, callback: fn(&mut Window)) -> Self {
        self.on_setup = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Register a callback that will be executed when the window is closed.
    pub fn on_exit(mut self, callback: fn(&mut Window)) -> Self {
        self.on_exit = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Build the configuration.
    pub fn build(self) -> LaunchConfig<'a, T> {
        LaunchConfig {
            window: WindowConfig {
                width: self.width,
                height: self.height,
                min_width: self.min_width,
                min_height: self.min_height,
                max_width: self.max_width,
                max_height: self.max_height,
                title: self.title,
                decorations: self.decorations,
                transparent: self.transparent,
                state: self.state,
                background: self.background,
                on_setup: self.on_setup,
                on_exit: self.on_exit,
            },
            fonts: self.fonts,
        }
    }
}
