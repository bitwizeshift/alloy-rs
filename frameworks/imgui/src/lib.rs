//! This crate provides a wrapper around the underlying imgui implementation.
//!
//! The raw generated bindings can be found in the [`c`] module, or a more
//! hierarchical binding may be leveraged from the rest of this crate.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

/// This module provides all that raw definitions of the C Vulkan library.
pub mod c {
  #![allow(non_upper_case_globals)]
  #![allow(non_camel_case_types)]
  #![allow(non_snake_case)]
  #![allow(unused)]
  #![allow(missing_docs)]
  #![allow(unused_results)]
  #![allow(rust_2018_idioms)]
  #![allow(rustdoc::broken_intra_doc_links)]
  #![allow(clippy::all)]
  #[doc(inline)]
  pub use imgui_sys::*;
}

/// This module provides a wrapper around the [`c::ImGuiWindowFlags`] type.
#[derive(Default, Copy, Clone, Debug)]
pub struct WindowFlags {
  flags: crate::c::ImGuiWindowFlags,
  show_window_close: bool,
}

impl WindowFlags {
  /// Returns the raw window flags.
  pub fn flags(&self) -> crate::c::ImGuiWindowFlags {
    self.flags
  }

  /// Sets the window to be invisible.
  pub fn no_title_bar(self, enable: bool) -> Self {
    self.set_flag(
      crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoTitleBar,
      enable,
    )
  }

  /// Sets the window to be invisible.
  pub fn no_scrollbar(self, enable: bool) -> Self {
    self.set_flag(
      crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoScrollbar,
      enable,
    )
  }

  /// Sets the window to be invisible.
  pub fn no_move(self, enable: bool) -> Self {
    self.set_flag(crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoMove, enable)
  }

  /// Sets the window to be invisible.
  pub fn no_resize(self, enable: bool) -> Self {
    self.set_flag(
      crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoResize,
      enable,
    )
  }

  /// Sets the window to be invisible.
  pub fn no_collapse(self, enable: bool) -> Self {
    self.set_flag(
      crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoCollapse,
      enable,
    )
  }

  /// Sets the window to be invisible.
  pub fn no_nav(self, enable: bool) -> Self {
    self.set_flag(crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoNav, enable)
  }

  /// Sets the window to be invisible.
  pub fn no_background(self, enable: bool) -> Self {
    self.set_flag(
      crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoBackground,
      enable,
    )
  }

  /// Sets the window to be invisible.
  pub fn no_bring_to_front_on_focus(self, enable: bool) -> Self {
    self.set_flag(
      crate::c::ImGuiWindowFlags__ImGuiWindowFlags_NoBringToFrontOnFocus,
      enable,
    )
  }

  /// Whether to show the window close button (default: false)
  pub fn show_window_close(self, enable: bool) -> Self {
    Self {
      flags: self.flags,
      show_window_close: enable,
    }
  }

  const fn set_flag(self, flag: u32, enable: bool) -> Self {
    if enable {
      Self {
        flags: self.flags | flag as i32,
        show_window_close: self.show_window_close,
      }
    } else {
      Self {
        flags: self.flags & !flag as i32,
        show_window_close: self.show_window_close,
      }
    }
  }
}

///
pub trait Renderable {
  ///
  fn render(&self);
}

/// Creates a window with the given name and flags.
pub fn begin(name: &str, open: Option<&mut bool>, flags: WindowFlags) -> bool {
  unsafe {
    let name = std::ffi::CString::new(name).unwrap();
    let show_window_close = match open {
      Some(open) => open as *mut _,
      None => std::ptr::null_mut(),
    };
    c::igBegin(name.as_ptr(), show_window_close, flags.flags())
  }
}

///
#[inline(always)]
pub fn begin_default(name: &str) -> bool {
  begin(name, None, WindowFlags::default())
}

/// This module provides I/O functionality that wraps the [`c::ImGuiIO`] object.
///
/// [`c::ImGuiIO`]: crate::c::ImGuiIO
pub mod io {
  use std::ffi::CStr;

  /// The configuration flags for the ImGui I/O context.
  #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
  pub struct ConfigFlags(crate::c::ImGuiConfigFlags);

  impl ConfigFlags {
    /// Returns the configuration flags.
    pub fn flags(&self) -> crate::c::ImGuiConfigFlags {
      self.0
    }

    /// Enables or disables the keyboard navigation
    pub fn allow_keyboard_navigation(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableKeyboard,
        enable,
      )
    }

    /// Enables or disables the gamepad navigation
    pub fn allow_gamepad_navigation(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableGamepad,
        enable,
      )
    }

    /// Enables or disables the mouse navigation
    pub fn allow_mouse_navigation(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_NavEnableSetMousePos,
        enable,
      )
    }

    /// Enables or disables docking support.
    pub fn allow_docking(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_DockingEnable,
        enable,
      )
    }

    /// Enables or disables viewports.
    pub fn allow_viewports(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_ViewportsEnable,
        enable,
      )
    }

    /// Sets whether the mouse should be disabled.
    pub fn no_mouse(self, enable: bool) -> Self {
      self.set_flag(crate::c::ImGuiConfigFlags__ImGuiConfigFlags_NoMouse, enable)
    }

    /// Sets whether the mouse cursor should change.
    pub fn no_mouse_cursor_change(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_NoMouseCursorChange,
        enable,
      )
    }

    /// Sets whether viewports should be DPI-scaled.
    pub fn dpi_scale_viewports(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_DpiEnableScaleViewports,
        enable,
      )
    }

    /// Sets whether fonts should be DPI-scaled.
    pub fn dpi_scale_fonts(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_DpiEnableScaleFonts,
        enable,
      )
    }

    /// Sets whether the color profile is SRGB
    pub fn set_srgb(self, enable: bool) -> Self {
      self.set_flag(crate::c::ImGuiConfigFlags__ImGuiConfigFlags_IsSRGB, enable)
    }

    /// Sets whether the application is a touch screen.
    pub fn set_touchscreen(self, enable: bool) -> Self {
      self.set_flag(
        crate::c::ImGuiConfigFlags__ImGuiConfigFlags_IsTouchScreen,
        enable,
      )
    }
    const fn set_flag(self, flag: u32, enable: bool) -> Self {
      if enable {
        Self(self.0 | flag as i32)
      } else {
        Self(self.0 & !flag as i32)
      }
    }
  }

  /// Sets the configuration flags for the ImGui I/O context.
  pub fn set_io_config(config: ConfigFlags) {
    unsafe {
      let io = crate::c::igGetIO();
      (*io).ConfigFlags = config.flags();
    }
  }

  /// Sets the INI filename that ImGui will use.
  ///
  /// # Parameters
  ///
  /// * `filename` - The filename to set.
  pub fn set_ini_filename_cstr(filename: &'static CStr) {
    use crate::c;

    // SAFETY: This is safe because the pointer is valid for the lifetime of the
    // CStr, and because io is always a valid pointer.
    unsafe {
      let io = c::igGetIO();
      (*io).IniFilename = filename.as_ptr();
    }
  }

  /// Sets the INI filename that ImGui will use.
  ///
  /// # Safety
  ///
  /// The input string must be a valid static C string.
  ///
  /// # Parameters
  ///
  /// * `filename` - The filename to set.
  pub unsafe fn set_ini_filename(filename: &'static str) {
    let cstr = std::ffi::CStr::from_ptr(filename.as_ptr() as *const _);
    set_ini_filename_cstr(cstr);
  }
}

/// This module provides a wrapper around the underlying imgui implementation.
pub mod backend {
  use crate::c;
  use astd::ffi::cstr;

  /// A backend is a platform-specific implementation of the imgui renderer.
  pub trait Backend {
    /// Initializes the backend.
    fn init_for_window(&self, window: &mut glfw::Window);

    /// Creates a new frame.
    fn new_frame(&self);

    /// Renders the frame.
    fn render(&self);

    /// Shuts down the backend.
    fn shutdown(&self);
  }

  /// The OpenGL backend.
  #[cfg(feature = "opengl")]
  pub struct OpenGL;

  #[cfg(feature = "opengl")]
  impl Backend for OpenGL {
    fn init_for_window(&self, window: &mut glfw::Window) {
      unsafe {
        _ = c::backend::glfw::imgui_glfw_init_for_opengl(window.ptr_mut(), true);
        _ = c::backend::opengl::imgui_opengl3_init(cstr!("#version 410").as_ptr());
      }
    }

    fn new_frame(&self) {
      unsafe {
        _ = c::backend::opengl::imgui_opengl3_new_frame();
        _ = c::backend::glfw::imgui_glfw_new_frame();
      }
    }

    fn render(&self) {
      unsafe {
        _ = c::backend::opengl::imgui_opengl3_render_draw_data(c::igGetDrawData());
      }
    }

    fn shutdown(&self) {
      unsafe {
        _ = c::backend::opengl::imgui_opengl3_shutdown();
      }
    }
  }
}

/// The context is the default setup for Imgui.
pub struct Context(*mut c::ImGuiContext);

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}

impl Context {
  /// Constructs an ImGui context
  pub fn new() -> Self {
    unsafe { Self(c::igCreateContext(std::ptr::null_mut())) }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { c::igDestroyContext(self.0) }
  }
}

/// Begins a new frame.
pub fn new_frame() {
  unsafe {
    c::igNewFrame();
  }
}

/// Ends the current frame.
pub fn end_frame() {
  unsafe {
    c::igEndFrame();
  }
}

///
#[macro_export]
macro_rules! text {
  ($($arg:tt)*) => {
    unsafe {
      use std::ffi::CString;
      let s = format!($($arg)*);
      let cs = CString::new(s).unwrap();
      $crate::c::igText(cs.as_ptr());
    }
  };
}
