use crate::alloc::{Allocator, Scope};
use std::alloc::Layout;
use std::ffi::c_void;
use vulkan_sys as c;

/// A collection of allocation callbacks that can be used within Vulkan.
#[derive(Clone)]
#[repr(transparent)]
pub struct AllocationCallbacks(c::VkAllocationCallbacks);

foundation::define_transparent! {
  AllocationCallbacks.0 => c::VkAllocationCallbacks
}

impl AllocationCallbacks {
  /// Constructs a new set of [`AllocationCallbacks`] from an underlying
  /// `allocator` object.
  ///
  /// # Arguments
  ///
  /// * `allocator` - the allocator to use for the allocation callbacks
  pub fn new<T: Allocator>(allocator: &mut T) -> Self {
    Self(c::VkAllocationCallbacks {
      pUserData: allocator as *mut _ as *mut c_void,
      pfnAllocation: Some(vulkan_alloc_fn::<T>),
      pfnReallocation: Some(vulkan_realloc_fn::<T>),
      pfnFree: Some(vulkan_dealloc_fn::<T>),
      pfnInternalAllocation: None,
      pfnInternalFree: None,
    })
  }

  /// Constructs the [`AllocationCallbacks`] object from the raw underlying C
  /// API.
  ///
  /// # Arguments
  ///
  /// * `callbacks` - the callbacks to user internally.
  pub const fn from_c(callbacks: c::VkAllocationCallbacks) -> Self {
    Self(callbacks)
  }

  /// Returns a pointer to the internal [`VkAllocationCallbacks`]
  ///
  /// [`VkAllocationCallbacks`]: c::VkAllocationCallbacks
  pub fn as_ptr(&self) -> *const c::VkAllocationCallbacks {
    &self.0 as *const c::VkAllocationCallbacks
  }

  /// Returns a mutable pointer to the internal [`VkAllocationCallbacks`]
  ///
  /// [`VkAllocationCallbacks`]: c::VkAllocationCallbacks
  pub fn as_ptr_mut(&mut self) -> *mut c::VkAllocationCallbacks {
    &mut self.0 as *mut c::VkAllocationCallbacks
  }
}

unsafe extern "C" fn vulkan_alloc_fn<T: Allocator>(
  data: *mut c_void,
  size: usize,
  alignment: usize,
  scope: c::VkSystemAllocationScope,
) -> *mut c_void {
  let allocator: &mut T = unsafe { &mut *(data as *mut T) };
  let maybe_layout = Layout::from_size_align(size, alignment);

  let layout = match maybe_layout {
    Ok(layout) => layout,
    Err(_) => return std::ptr::null_mut(),
  };

  match allocator.allocate(layout, unsafe { Scope::from_c_unchecked(scope) }) {
    Ok(ptr) => ptr.as_ptr() as *mut c_void,
    Err(_) => std::ptr::null_mut(),
  }
}

unsafe extern "C" fn vulkan_dealloc_fn<T: Allocator>(data: *mut c_void, memory: *mut c_void) {
  let allocator: &mut T = unsafe { &mut *(data as *mut T) };

  allocator.deallocate(memory as *mut u8);
}

unsafe extern "C" fn vulkan_realloc_fn<T: Allocator>(
  data: *mut c_void,
  ptr: *mut c_void,
  size: usize,
  alignment: usize,
  scope: c::VkSystemAllocationScope,
) -> *mut c_void {
  let allocator: &mut T = unsafe { &mut *(data as *mut T) };
  let layout = Layout::from_size_align(size, alignment).expect("Invalid alignment or size");

  let value = unsafe { Scope::from_c_unchecked(scope) };
  match allocator.reallocate(ptr as *mut u8, layout, value) {
    Ok(ptr) => ptr.as_ptr() as *mut c_void,
    Err(_) => std::ptr::null_mut(),
  }
}
