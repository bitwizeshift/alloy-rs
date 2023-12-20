use crate::alloc::Scope;
use std::alloc::Layout;
use std::ptr::NonNull;

/// The primary trait to define the allocation mechanisms in the Vulkan API.
///
/// Types implementing this can be converted easily into [`VkAllocationCallbacks`]
///
/// # Safety
///
/// This trait requires the semantics of an allocator, in that an allocation
/// must always be freeable by a call to deallocate.
///
/// [`VkAllocationCallbacks`] c::VkAllocationCallbacks
pub unsafe trait Allocator {
  /// The error type that may be returned
  type Error;

  /// Requests Layout memory from the underlying system to be used with the
  /// given allocation scope
  ///
  /// # Arguments
  ///
  /// * `layout` - the layout to allocate
  /// * `scope` - the amount requested to allocate
  fn allocate(&mut self, layout: Layout, scope: Scope) -> Result<NonNull<u8>, Self::Error>;

  /// Requests to deallocate the bytes at the given address
  ///
  /// # Arguments
  ///
  /// * `data` - the bytes to deallocate
  ///
  /// # Safety
  ///
  /// * `data` must denote a block of memory currently allocated via this allocator
  unsafe fn deallocate(&mut self, data: *mut u8);

  /// Requests that a given pointer be reallocated to align to the new layout
  /// and within the given scope.
  ///
  /// # Arguments
  ///
  /// * `ptr` - the address to reallocate
  /// * `new_layout` - the new layout to allocate at this pointer
  /// * `scope` - the new scope this allocation should be allocated to
  ///
  /// # Safety
  ///
  /// This function is unsafe because undefined behavior can result if the
  /// caller does not ensure all of the following:
  ///
  /// * `ptr` must be currently allocated via this allocator,
  /// * `new_layout` must be greater than zero
  unsafe fn reallocate(
    &mut self,
    ptr: *mut u8,
    new_layout: Layout,
    new_scope: Scope,
  ) -> Result<NonNull<u8>, Self::Error>;
}
