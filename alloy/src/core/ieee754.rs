//! The ieee754 module provides utilities for working with IEEE 754
//! floating-point numbers at compile-time.

/// Module for working with 32-bit floating point numbers.
pub mod f32 {

  /// The mask for the sign bit of a 32-bit floating point number.
  #[cfg(target_endian = "little")]
  pub const SIGN_BIT_MASK_F32: u32 = 0x7FFFFFFF;

  /// The mask for the sign bit of a 32-bit floating point number.
  #[cfg(target_endian = "big")]
  pub const SIGN_BIT_MASK_F32: u32 = 0xFFFFFF7F;

  /// Computes the absolute value of a 32-bit floating point number at compile-time.
  ///
  /// # Safety
  ///
  /// This function should largely be to safe to use, but requires that the user
  /// ensure that numbers are not NaN or subnormal numbers. This ultimately should
  /// only be used in cases where constant-time operations are desired (e.g. during
  /// construction).
  ///
  /// # Arguments
  ///
  /// * `x` - The floating point number.
  pub const unsafe fn abs(x: f32) -> f32 {
    let bits: u32 = std::mem::transmute(x);
    // NOTE: This ignores NaN and subnormal numbers, which is largely fine for
    // the cases of where this is intended to be used.
    std::mem::transmute(bits & SIGN_BIT_MASK_F32)
  }

  /// Computes the negation of a 32-bit floating point number at compile-time.
  ///
  /// # Safety
  ///
  /// This function should largely be to safe to use, but requires that the user
  /// ensure that numbers are not NaN or subnormal numbers. This ultimately should
  /// only be used in cases where constant-time operations are desired (e.g. during
  /// construction).
  ///
  /// # Arguments
  ///
  /// * `x` - The floating point number.
  pub const unsafe fn neg(x: f32) -> f32 {
    let bits: u32 = std::mem::transmute(x);
    // NOTE: This ignores NaN and subnormal numbers, which is largely fine for
    // the cases of where this is intended to be used.
    std::mem::transmute(bits ^ !SIGN_BIT_MASK_F32)
  }
}

#[cfg(test)]
#[path = "ieee754.test.rs"]
mod test;
