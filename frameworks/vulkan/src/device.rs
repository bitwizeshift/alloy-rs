//! This module provides wrappers for both logical and physical Vulkan devices
//! (e.g. [`VkPhysicalDevice`])
//!
//! [`VkPhysicalDevice`]: crate::c::VkPhysicalDevice

mod physical_device;
mod physical_device_features;
mod physical_device_limits;
mod physical_device_properties;
mod physical_device_sparse_properties;
mod physical_device_type;
mod sample_count_flags;

#[doc(inline)]
pub use physical_device::*;
#[doc(inline)]
pub use physical_device_features::*;
#[doc(inline)]
pub use physical_device_limits::*;
#[doc(inline)]
pub use physical_device_properties::*;
#[doc(inline)]
pub use physical_device_sparse_properties::*;
#[doc(inline)]
pub use physical_device_type::*;
#[doc(inline)]
pub use sample_count_flags::*;
