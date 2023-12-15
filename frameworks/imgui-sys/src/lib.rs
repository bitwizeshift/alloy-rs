//! This crate provides a statically-linked definition of the C imgui library
//! from source.
//!
//! The generated bindings are embedded in this module.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(missing_docs)]
#![allow(unused_results)]
#![allow(rust_2018_idioms)]
#![allow(rustdoc::broken_intra_doc_links)]
include!(concat!(env!("OUT_DIR"), "/imgui.rs"));

pub mod backend {

  pub mod glfw {
    use core::ffi::{c_int, c_uint, c_void};

    extern "C" {
      pub fn imgui_glfw_init_for_opengl(
        window: *mut glfw_sys::GLFWwindow,
        install_callbacks: bool,
      ) -> bool;
      pub fn imgui_glfw_init_for_vulkan(
        window: *mut glfw_sys::GLFWwindow,
        install_callbacks: bool,
      ) -> bool;
      pub fn imgui_glfw_init_for_other(
        window: *mut glfw_sys::GLFWwindow,
        install_callbacks: bool,
      ) -> bool;
      pub fn imgui_glfw_shutdown() -> c_void;
      pub fn imgui_glfw_new_frame() -> c_void;

      pub fn imgui_glfw_set_callbacks_chain_for_all_windows(chain_for_all_windows: bool) -> c_void;
      pub fn imgui_glfw_window_focus_callback(
        window: *mut glfw_sys::GLFWwindow,
        focused: c_int,
      ) -> c_void;
      pub fn imgui_glfw_cursor_enter_callback(
        window: *mut glfw_sys::GLFWwindow,
        entered: c_int,
      ) -> c_void;
      pub fn imgui_glfw_cursor_pos_callback(
        window: *mut glfw_sys::GLFWwindow,
        x: f64,
        y: f64,
      ) -> c_void;
      pub fn imgui_glfw_mouse_button_callback(
        window: *mut glfw_sys::GLFWwindow,
        button: c_int,
        action: c_int,
        mods: c_int,
      ) -> c_void;
      pub fn imgui_glfw_scroll_callback(
        window: *mut glfw_sys::GLFWwindow,
        xoffset: f64,
        yoffset: f64,
      ) -> c_void;
      pub fn imgui_glfw_key_callback(
        window: *mut glfw_sys::GLFWwindow,
        key: c_int,
        scancode: c_uint,
        action: c_int,
        mods: c_int,
      ) -> c_void;
      pub fn imgui_glfw_char_callback(window: *mut glfw_sys::GLFWwindow, c: c_uint) -> c_void;
      pub fn imgui_glfw_monitor_callback(window: *mut glfw_sys::GLFWwindow, event: c_int)
        -> c_void;
    }
  } // extern "C"

  #[cfg(feature = "vulkan")]
  pub mod vulkan {}

  #[cfg(feature = "opengl")]
  pub mod opengl {}
}
