//! The [`debug`] module provides utilities for working with the Vulkan
//! debugging layer extensions
//!
//! [`debug`]: crate::debug

use foundation::cstr;
use vulkan_sys as c;

/// The [`raw`] module provides utility APIs using the raw underlying C API.
///
/// It is not strictly as safe to use these APIs, but may be required from time
/// to time to work around Rust's lifetime system.
pub mod raw {
  use super::*;

  /// Creates a debug messenger utility
  ///
  /// # Safety
  ///
  /// This function requires that the caller pass valid information for each of
  /// the arguments, and uses the same `allocator` argument when destroying the
  /// messenger instance.
  pub unsafe fn create_debug_utils_messenger_ext(
    instance: c::VkInstance,
    create_info: *const c::VkDebugUtilsMessengerCreateInfoEXT,
    allocator: *const c::VkAllocationCallbacks,
    debug_messenger: *mut c::VkDebugUtilsMessengerEXT,
  ) -> c::VkResult {
    let func = c::vkGetInstanceProcAddr(instance, cstr!("vkCreateDebugUtilsMessengerEXT").as_ptr());
    let func: c::PFN_vkCreateDebugUtilsMessengerEXT = std::mem::transmute(func);
    if let Some(callback) = func {
      callback(instance, create_info, allocator, debug_messenger)
    } else {
      c::VkResult_VK_ERROR_EXTENSION_NOT_PRESENT
    }
  }

  /// Destroys a debug messenger utility
  ///
  /// # Safety
  ///
  /// This function requires that the caller pass valid information for each of
  /// the arguments, and uses the same `allocator` argument when destroying the
  /// messenger instance.
  pub unsafe fn destroy_debug_utils_messenger_ext(
    instance: c::VkInstance,
    debug_messenger: c::VkDebugUtilsMessengerEXT,
    allocator: *const c::VkAllocationCallbacks,
  ) -> c::VkResult {
    let func =
      c::vkGetInstanceProcAddr(instance, cstr!("vkDestroyDebugUtilsMessengerEXT").as_ptr());
    let func: c::PFN_vkDestroyDebugUtilsMessengerEXT = std::mem::transmute(func);
    if let Some(callback) = func {
      callback(instance, debug_messenger, allocator);
      c::VkResult_VK_SUCCESS
    } else {
      c::VkResult_VK_ERROR_EXTENSION_NOT_PRESENT
    }
  }
}
