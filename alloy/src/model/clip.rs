//! The `clip` module provides definitions for clip-space information.

use crate::math::vec::{Vec2, Vector2};

/// Representation of a depth range.
#[derive(Debug)]
pub struct Depth {
  near: f32,
  far: f32,
}

// Constructors

impl Depth {
  /// Creates a new depth range.
  ///
  /// # Panics
  ///
  /// Panics if `near` is less than or equal to zero, or if `far` is less than
  /// `near`.
  ///
  /// # Arguments
  ///
  /// * `near` - The near depth value.
  /// * `far` - The far depth value.
  pub const fn new(near: f32, far: f32) -> Self {
    // debug_assert!(near > 0.0);
    // debug_assert!(far > near);
    Self { near, far }
  }
}

impl Default for Depth {
  fn default() -> Self {
    Self::new(0.1, 100.0)
  }
}

impl From<Vector2> for Depth {
  fn from(vector: Vector2) -> Self {
    Self::new(vector.x(), vector.y())
  }
}

impl From<&Vec2> for Depth {
  fn from(vector: &Vec2) -> Self {
    Self::new(vector.x(), vector.y())
  }
}

// Properties

impl Depth {
  /// Retrieves the near depth value.
  pub const fn near(&self) -> f32 {
    self.near
  }

  /// Retrieves the far depth value.
  pub const fn far(&self) -> f32 {
    self.far
  }
}

/// Representation of a horizontal range.
#[derive(Debug)]
pub struct Horizontal {
  left: f32,
  right: f32,
}

impl Horizontal {
  /// Creates a new horizontal range.
  ///
  /// # Panics
  ///
  /// Panics if `left` is greater than or equal to `right`.
  ///
  /// # Arguments
  ///
  /// * `left` - The left horizontal value.
  /// * `right` - The right horizontal value.
  #[inline(always)]
  pub fn new(left: f32, right: f32) -> Self {
    debug_assert!(left < right);
    Self { left, right }
  }

  /// Creates a new horizontal range with a uniform value.
  ///
  /// # Panics
  ///
  /// Panics if `value` is less than or equal to zero.
  ///
  /// # Arguments
  ///
  /// * `value` - The uniform value.
  #[inline(always)]
  pub fn uniform(value: f32) -> Self {
    debug_assert!(value > 0.0);
    Self::new(-value, value)
  }
}

impl Default for Horizontal {
  fn default() -> Self {
    Self::uniform(1.0)
  }
}

impl From<Vector2> for Horizontal {
  fn from(vector: Vector2) -> Self {
    Self::new(vector.x(), vector.y())
  }
}

impl From<&Vec2> for Horizontal {
  fn from(vector: &Vec2) -> Self {
    Self::new(vector.x(), vector.y())
  }
}

impl Horizontal {
  /// Retrieves the left horizontal value.
  pub const fn left(&self) -> f32 {
    self.left
  }

  /// Retrieves the right horizontal value.
  pub const fn right(&self) -> f32 {
    self.right
  }
}

/// Representation of a vertical range.
#[derive(Debug)]
pub struct Vertical {
  bottom: f32,
  top: f32,
}

impl Vertical {
  /// Creates a new vertical range.
  ///
  /// # Panics
  ///
  /// Panics if `bottom` is greater than or equal to `top`.
  ///
  /// # Arguments
  ///
  /// * `bottom` - The bottom vertical value.
  /// * `top` - The top vertical value.
  #[inline(always)]
  pub fn new(bottom: f32, top: f32) -> Self {
    debug_assert!(bottom < top);
    Self { bottom, top }
  }

  /// Creates a new vertical range with a uniform value.
  ///
  /// # Panics
  ///
  /// Panics if `value` is less than or equal to zero.
  ///
  /// # Arguments
  ///
  /// * `value` - The uniform value.
  #[inline(always)]
  pub fn uniform(value: f32) -> Self {
    debug_assert!(value > 0.0);
    Self::new(-value, value)
  }
}

impl Default for Vertical {
  fn default() -> Self {
    Self::uniform(1.0)
  }
}

impl From<Vector2> for Vertical {
  fn from(vector: Vector2) -> Self {
    Self::new(vector.x(), vector.y())
  }
}

impl From<&Vec2> for Vertical {
  fn from(vector: &Vec2) -> Self {
    Self::new(vector.x(), vector.y())
  }
}

impl Vertical {
  /// Retrieves the bottom vertical value.
  pub const fn bottom(&self) -> f32 {
    self.bottom
  }

  /// Retrieves the top vertical value.
  pub const fn top(&self) -> f32 {
    self.top
  }
}

/// Representation of clip-space information.
#[derive(Default, Debug)]
pub struct ClipSpace {
  /// The depth range.
  pub depth: Depth,

  /// The horizontal range.
  pub horizontal: Horizontal,

  /// The vertical range.
  pub vertical: Vertical,
}

impl ClipSpace {
  /// Creates a new clip-space information.
  ///
  /// # Arguments
  ///
  /// * `depth` - The depth range.
  /// * `horizontal` - The horizontal range.
  /// * `vertical` - The vertical range.
  pub const fn new(depth: Depth, horizontal: Horizontal, vertical: Vertical) -> Self {
    Self {
      depth,
      horizontal,
      vertical,
    }
  }

  /// Creates a new clip-space information with symmetric horizontal and
  /// vertical
  ///
  /// # Panics
  ///
  /// Panics if `depth` is less than or equal to zero, or if `horizontal` or
  /// `vertical` are less than or equal to zero.
  ///
  /// # Arguments
  ///
  /// * `depth` - The depth range.
  /// * `horizontal` - The horizontal range.
  /// * `vertical` - The vertical range.
  #[inline(always)]
  pub fn uniform(depth: f32, horizontal: f32, vertical: f32) -> Self {
    Self::new(
      Depth::new(0.0, depth),
      Horizontal::uniform(horizontal),
      Vertical::uniform(vertical),
    )
  }
}
