use crate::c::GLenum;

/// Specify clear values for the color buffers
///
/// **NOTE:** Calls the [`glClearColor`]` function.
///
/// # Arguments
///
/// * `red`   - the red color (clamped between 0.0 and 1.0)
/// * `green` - the green color (clamped between 0.0 and 1.0)
/// * `blue`  - the blue color (clamped between 0.0 and 1.0)
/// * `alpha` -  the alpha value (clamped between 0.0 and 1.0)
///
/// [`glClearColor`]: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glClearColor.xhtml
pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
  unsafe { crate::c::glClearColor(red, green, blue, alpha) }
}

/// Specify the clear value for the depth buffer
///
/// **Note:** Calls the [`glClearDepth`] function
///
/// # Arguments
///
/// * `depth` - the depth value (clamped between 0.0 and 1.0)
///
/// [`glClearDepth`]: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glClearDepth.xhtml
pub fn clear_depth(depth: f64) {
  unsafe { crate::c::glClearDepth(depth) }
}

/// Specify the clear value for the stencil buffer
///
/// **Note:** Calls the [`glClearStencil`] function
///
/// # Arguments
///
/// * `s` - the stencil value
///
/// [`glClearStencil`]: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glClearStencil.xhtml
pub fn clear_stencil(s: i32) {
  unsafe { crate::c::glClearStencil(s) }
}

/// Specify the clear value for the color, depth, and stencil buffers
/// used in [`clear`].
#[derive(Copy, Clone)]
pub struct ClearBits(GLenum);

impl ClearBits {
  /// Clear the color buffer
  pub const COLOR: Self = Self(crate::c::GL_COLOR_BUFFER_BIT);

  /// Clear the depth buffer
  pub const DEPTH: Self = Self(crate::c::GL_DEPTH_BUFFER_BIT);

  /// Clear the stencil buffer
  pub const STENCIL: Self = Self(crate::c::GL_STENCIL_BUFFER_BIT);

  /// Constructs a new `ClearBits` from the given bits.
  ///
  /// # Arguments
  ///
  /// * `bits` - the bits to construct the `ClearBits` from
  #[inline]
  pub const fn from_bits(bits: GLenum) -> Self {
    Self(bits)
  }

  /// Returns the bits of the `ClearBits`.
  #[inline]
  pub const fn bits(&self) -> GLenum {
    self.0
  }

  /// Returns whether the `ClearBits` contains the given bits.
  ///
  /// # Arguments
  ///
  /// * `other` - the bits to check for
  #[inline]
  pub const fn contains(&self, other: Self) -> bool {
    self.0 & other.0 == other.0
  }

  /// Returns whether the `ClearBits` is empty.
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.0 == 0
  }

  /// Returns the union of the `ClearBits` and the given bits.
  #[inline]
  pub const fn all() -> Self {
    Self(
      crate::c::GL_COLOR_BUFFER_BIT
        | crate::c::GL_DEPTH_BUFFER_BIT
        | crate::c::GL_STENCIL_BUFFER_BIT,
    )
  }

  /// Returns an empty `ClearBits`.
  #[inline]
  pub const fn none() -> Self {
    Self(0)
  }

  /// Returns the union of the `ClearBits` and the given bits.
  #[inline]
  pub fn color(mut self) -> Self {
    self.0 |= Self::COLOR.0;
    self
  }

  /// Returns the union of the `ClearBits` and the given bits.
  #[inline]
  pub fn depth(mut self) -> Self {
    self.0 |= Self::DEPTH.0;
    self
  }

  /// Returns the union of the `ClearBits` and the given bits.
  #[inline]
  pub fn stencil(mut self) -> Self {
    self.0 |= Self::STENCIL.0;
    self
  }

  /// Returns whether the `ClearBits` contains the color bits.
  #[inline]
  pub const fn has_color(&self) -> bool {
    self.contains(Self::COLOR)
  }

  /// Returns whether the `ClearBits` contains the depth bits.
  #[inline]
  pub const fn has_depth(&self) -> bool {
    self.contains(Self::DEPTH)
  }

  /// Returns whether the `ClearBits` contains the stencil bits.
  #[inline]
  pub const fn has_stencil(&self) -> bool {
    self.contains(Self::STENCIL)
  }

  /// Clears the buffers to preset values.
  #[inline]
  pub fn clear(self) {
    clear(self)
  }
}

/// Clear buffers to preset values
///
/// **Note:** Calls the [`glClear`] function
///
/// # Arguments
///
/// * `bits` - the buffers to clear
///
/// [`glClear`]: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glClear.xhtml
#[inline]
pub fn clear(bits: ClearBits) {
  unsafe { crate::c::glClear(bits.0) }
}
