//! The ieee754 module provides utilities for working with IEEE 754
//! floating-point numbers at compile-time.

/// Module for working with 32-bit floating point numbers at compile-time.
pub mod f32 {

  /// Determines the sign of a 32-bit floating point number.
  #[inline(always)]
  pub const fn sign(x: f32) -> bool {
    let bits: u32 = unsafe { std::mem::transmute(x) };
    sign_impl(bits)
  }

  /// The mask for the exponent bits of a 32-bit floating point number.
  #[inline(always)]
  pub const fn exponent(x: f32) -> u32 {
    let bits: u32 = unsafe { std::mem::transmute(x) };
    exponent_impl(bits)
  }

  /// The mask for the mantissa bits of a 32-bit floating point number.
  #[inline(always)]
  pub const fn mantissa(x: f32) -> u32 {
    let bits: u32 = unsafe { std::mem::transmute(x) };
    mantissa_impl(bits)
  }

  /// Determines if a 32-bit floating point number is a NaN.
  #[inline(always)]
  pub const fn is_nan(x: f32) -> bool {
    let bits: u32 = unsafe { std::mem::transmute(x) };
    is_nan_impl(bits)
  }

  /// Computes the parts of a 32-bit floating point number.
  ///
  /// This returns the sign of the float, the exponent, and the mantissa.
  ///
  /// # Parameters
  ///
  /// * `x` - The floating point number.
  #[inline(always)]
  pub const fn parts(x: f32) -> (bool, u32, u32) {
    let bits: u32 = unsafe { std::mem::transmute(x) };
    (sign_impl(bits), exponent_impl(bits), mantissa_impl(bits))
  }

  /// Returns the merged exponent and mantissa of a 32-bit floating point number.
  #[inline(always)]
  pub const fn exponent_mantissa(x: f32) -> u32 {
    let bits: u32 = unsafe { std::mem::transmute(x) };
    exponent_mantissa_impl(bits)
  }

  /// Computes the absolute value of a 32-bit floating point number at compile-time.
  ///
  /// NOTE: Unlike the standard [`f32::abs`] function, this function will
  /// return the absolute value of a floating point number, even if it is a NaN.
  /// This means that the representation of the NaN may change by calling this,
  /// even though the value is still considered a NaN.
  ///
  /// # Parameters
  ///
  /// * `x` - The floating point number.
  pub const fn abs(x: f32) -> f32 {
    // SAFETY: Transmuting float to u32 is safe, it's just generally inadvisable.
    let bits: u32 = unsafe { std::mem::transmute(x) };

    // SAFETY: This is safe because the bits are transmuted back to a float.
    unsafe { std::mem::transmute(abs_impl(bits)) }
  }

  /// Computes the negation of a 32-bit floating point number at compile-time.
  ///
  /// NOTE: Unlike the standard [`Neg`] trait for [`f32`], this function will
  /// return the negation of a floating point number, even if it is a NaN. This
  /// means that the representation of the NaN may change by calling this, even
  /// though the value is still a NaN.
  ///
  /// # Parameters
  ///
  /// * `x` - The floating point number.
  ///
  /// [`Neg`]: std::ops::Neg
  pub const fn neg(x: f32) -> f32 {
    // SAFETY: Transmuting float to u32 is safe, it's just generally inadvisable.
    let bits: u32 = unsafe { std::mem::transmute(x) };

    // SAFETY: This is safe because the bits are transmuted back to a float.
    unsafe { std::mem::transmute(neg_impl(bits)) }
  }

  /// Determines if a 32-bit floating point number is less than another at
  /// compile-time.
  ///
  /// This function is _not_ recommended in general code, since its performance
  /// characteristics will not be equivalent to the standard library's
  /// [`PartialOrd`] implementation. However it is useful for `const` contexts
  /// so that it can be used in [`assert`]-like situations.
  ///
  /// # Parameters
  ///
  /// * `lhs` - The left-hand side of the comparison.
  /// * `rhs` - The right-hand side of the comparison.
  pub const fn lt(lhs: f32, rhs: f32) -> bool {
    // SAFETY: Transmuting float to u32 is "safe", it's just generally inadvisable.
    // They both have the same size and alignment, and this is how to_bits is
    // implemented in the standard library -- however to_bits does handle a few
    // extra cases on tier-2 platforms that might change the f32 representation
    // at runtime. These don't apply to Alloy, so this is safe for our uses.
    //
    // This will be updated to use to_bits when able to.
    let lhs_bits: u32 = unsafe { std::mem::transmute(lhs) };
    let rhs_bits: u32 = unsafe { std::mem::transmute(rhs) };

    if is_nan(lhs) || is_nan(rhs) {
      return false;
    }
    if sign_impl(lhs_bits) && !sign_impl(rhs_bits) {
      return true;
    }
    if exponent_impl(lhs_bits) < exponent_impl(rhs_bits) {
      return true;
    }
    if mantissa_impl(lhs_bits) < mantissa_impl(rhs_bits) {
      return true;
    }
    false
  }

  /// Determines if a 32-bit floating point number is equal to another at
  /// compile-time.
  ///
  /// This function is _not_ recommended in general code, since its performance
  /// characteristics will not be equivalent to the standard library's
  /// [`PartialEq`] implementation. However it is useful for `const` contexts
  /// so that it can be used in [`assert`]-like situations.
  ///
  /// # Parameters
  ///
  /// * `lhs` - The left-hand side of the comparison.
  /// * `rhs` - The right-hand side of the comparison.
  pub const fn eq(lhs: f32, rhs: f32) -> bool {
    // SAFETY: Transmuting float to u32 is safe, it's just generally inadvisable.
    // They both have the same size and alignment, and this is how to_bits is
    // implemented in the standard library -- it's just not const.
    let lhs_bits: u32 = unsafe { std::mem::transmute(lhs) };
    let rhs_bits: u32 = unsafe { std::mem::transmute(rhs) };

    if is_nan(lhs) || is_nan(rhs) {
      return false;
    }

    let lhs_content = exponent_mantissa_impl(lhs_bits);
    let rhs_content = exponent_mantissa_impl(rhs_bits);
    if sign_impl(lhs_bits) != sign_impl(rhs_bits) {
      return lhs_content == 0 && rhs_content == 0;
    }
    lhs_content == rhs_content
  }

  // Private impl

  #[inline(always)]
  const fn abs_le(bits: u32) -> u32 {
    bits & !0x80000000
  }

  #[cfg(target_endian = "little")]
  #[inline(always)]
  const fn abs_impl(bits: u32) -> u32 {
    abs_le(bits)
  }

  #[cfg(target_endian = "big")]
  #[inline(always)]
  const fn abs_impl(bits: u32) -> u32 {
    abs_le(bits.swap_bytes())
  }

  #[inline(always)]
  const fn neg_le(bits: u32) -> u32 {
    bits ^ 0x80000000
  }

  #[cfg(target_endian = "little")]
  #[inline(always)]
  const fn neg_impl(bits: u32) -> u32 {
    neg_le(bits)
  }

  #[cfg(target_endian = "big")]
  #[inline(always)]
  const fn neg_impl(bits: u32) -> u32 {
    neg_le(bits.swap_bytes())
  }

  #[cfg(target_endian = "little")]
  #[inline(always)]
  const fn sign_impl(bits: u32) -> bool {
    bits & 0x80000000 != 0
  }

  #[cfg(target_endian = "big")]
  #[inline(always)]
  const fn sign_impl(bits: u32) -> bool {
    bits & 0x00000080 != 0
  }

  #[inline(always)]
  const fn exponent_le(bits: u32) -> u32 {
    (bits & 0x7F800000) >> 23
  }

  #[cfg(target_endian = "little")]
  #[inline(always)]
  const fn exponent_impl(bits: u32) -> u32 {
    exponent_le(bits)
  }

  #[cfg(target_endian = "big")]
  #[inline(always)]
  const fn exponent_impl(bits: u32) -> u32 {
    exponent_le(bits.swap_bytes())
  }

  #[inline(always)]
  const fn mantissa_le(bits: u32) -> u32 {
    bits & 0x007FFFFF
  }

  #[cfg(target_endian = "little")]
  #[inline(always)]
  const fn mantissa_impl(bits: u32) -> u32 {
    mantissa_le(bits)
  }

  #[cfg(target_endian = "big")]
  #[inline(always)]
  const fn mantissa_impl(bits: u32) -> u32 {
    mantissa_le(bits.swap_bytes())
  }

  #[inline(always)]
  const fn is_nan_impl(bits: u32) -> bool {
    exponent_impl(bits) == 0xFF && mantissa_impl(bits) != 0
  }

  #[inline(always)]
  const fn exponent_mantissa_le(bits: u32) -> u32 {
    bits & 0x7FFFFFFF
  }

  #[cfg(target_endian = "little")]
  #[inline(always)]
  const fn exponent_mantissa_impl(bits: u32) -> u32 {
    exponent_mantissa_le(bits)
  }

  #[cfg(target_endian = "big")]
  #[inline(always)]
  const fn exponent_mantissa_impl(bits: u32) -> u32 {
    exponent_mantissa_le(bits.swap_bytes())
  }
}

#[cfg(test)]
#[path = "ieee754.test.rs"]
mod test;
