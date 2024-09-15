#![allow(missing_docs)]

use std::borrow::{Borrow, BorrowMut};
use std::fmt;

use crate::math::vec::{Vec2, Vec3, Vec4, Vector4};
/// A color represented by four floating point values.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct Color(Vector4);

// Constructor

impl Color {
  /// Create a new color with the given red, green, blue, and alpha values.
  ///
  /// # Parameters
  ///
  /// * `red` - The red value.
  /// * `green` - The green value.
  /// * `blue` - The blue value.
  /// * `alpha` - The alpha value.
  #[must_use]
  #[inline(always)]
  pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
    Self(Vector4::new(red, green, blue, alpha))
  }

  /// Create a new color from a 32-bit RGBA value.
  ///
  /// # Parameters
  ///
  /// * `rgba` - The 32-bit RGBA value.
  pub fn from_rgba32(rgba: u32) -> Self {
    let r = ((rgba >> 24) & 0xFF) as f32 / 255.0;
    let g = ((rgba >> 16) & 0xFF) as f32 / 255.0;
    let b = ((rgba >> 8) & 0xFF) as f32 / 255.0;
    let a = (rgba & 0xFF) as f32 / 255.0;
    Self::new(r, g, b, a)
  }

  /// Create a new color from a 64-bit RGBA value.
  ///
  /// # Parameters
  ///
  /// * `rgba` - The 64-bit RGBA value.
  pub fn from_rgba64(rgba: u64) -> Self {
    let r = ((rgba >> 48) & 0xFFFF) as f32 / 65535.0;
    let g = ((rgba >> 32) & 0xFFFF) as f32 / 65535.0;
    let b = ((rgba >> 16) & 0xFFFF) as f32 / 65535.0;
    let a = (rgba & 0xFFFF) as f32 / 65535.0;
    Self::new(r, g, b, a)
  }

  /// Create a new color from a 32-bit ARGB value.
  ///
  /// # Parameters
  ///
  /// * `argb` - The 32-bit ARGB value.
  pub fn from_argb32(argb: u32) -> Self {
    let a = ((argb >> 24) & 0xFF) as f32 / 255.0;
    let r = ((argb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((argb >> 8) & 0xFF) as f32 / 255.0;
    let b = (argb & 0xFF) as f32 / 255.0;
    Self::new(r, g, b, a)
  }

  /// Create a new color from a 64-bit ARGB value.
  ///
  /// # Parameters
  ///
  /// * `argb` - The 64-bit ARGB value.
  pub fn from_argb64(argb: u64) -> Self {
    let a = ((argb >> 48) & 0xFFFF) as f32 / 65535.0;
    let r = ((argb >> 32) & 0xFFFF) as f32 / 65535.0;
    let g = ((argb >> 16) & 0xFFFF) as f32 / 65535.0;
    let b = (argb & 0xFFFF) as f32 / 65535.0;
    Self::new(r, g, b, a)
  }

  /// Create a new color with the given red, green, and blue values.
  ///
  /// The alpha value will be set to 1.0.
  ///
  /// # Parameters
  ///
  /// * `red` - The red value.
  /// * `green` - The green value.
  /// * `blue` - The blue value.
  #[must_use]
  #[inline(always)]
  pub const fn new_opaque(red: f32, green: f32, blue: f32) -> Self {
    Self(Vector4::new(red, green, blue, 1.0))
  }

  /// Create a new color from an existing 3-component vector.
  ///
  /// # Parameters
  ///
  /// * `vec3` - The vector to create the color from.
  #[must_use]
  #[inline(always)]
  pub const fn from_vec3(vec: &Vec3) -> Self {
    Self(Vector4::new(vec.x(), vec.y(), vec.z(), 1.0))
  }

  /// Create a new color from an existing 4-component vector.
  ///
  /// # Parameters
  ///
  /// * `vec4` - The vector to create the color from.
  #[must_use]
  #[inline(always)]
  pub const fn from_vec4(vec: &Vec4) -> Self {
    Self(Vector4::from_vec4(vec))
  }
}

impl From<&Vec3> for Color {
  fn from(vec: &Vec3) -> Self {
    Self::from_vec3(vec)
  }
}

impl From<&Vec4> for Color {
  fn from(vec: &Vec4) -> Self {
    Self::from_vec4(vec)
  }
}

impl From<(f32, f32, f32)> for Color {
  fn from((r, g, b): (f32, f32, f32)) -> Self {
    Self::new(r, g, b, 1.0)
  }
}

impl From<(f32, f32, f32, f32)> for Color {
  fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
    Self::new(r, g, b, a)
  }
}

// Constants

impl Color {
  /// A solid opaque black [`Color`] object.
  pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);

  /// A solid opaque while [`Color`] object.
  pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

  /// A solid opaque red [`Color`] object.
  pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);

  /// A solid opaque green [`Color`] object.
  pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);

  /// A solid opaque blue [`Color`] object.
  pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

  /// A solid opaque yellow [`Color`] object.
  pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);

  /// A solid opaque cyan [`Color`] object.
  pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);

  /// A solid opaque magenta [`Color`] object.
  pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

  /// A solid transparent/invisible [`Color`] object.
  pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

// Properties

impl Color {
  /// Checks whether this color is opaque.
  ///
  /// This only returns true if the value is fully solid (e.g. alpha is 1.0).
  #[must_use]
  #[inline(always)]
  pub fn is_opaque(&self) -> bool {
    self.a() == 1.0
  }

  /// Checks whether this color is transparent.
  ///
  /// This only returns true if the value is fully transparent (e.g. alpha is 0.0).
  #[must_use]
  #[inline(always)]
  pub fn is_transparent(&self) -> bool {
    self.a() == 0.0
  }

  /// Get the red value of the color.
  #[must_use]
  #[inline(always)]
  pub const fn r(&self) -> f32 {
    self.0.as_vec4().x()
  }

  /// Get the green value of the color.
  #[must_use]
  #[inline(always)]
  pub const fn g(&self) -> f32 {
    self.0.as_vec4().y()
  }

  /// Get the blue value of the color.
  #[must_use]
  #[inline(always)]
  pub const fn b(&self) -> f32 {
    self.0.as_vec4().z()
  }

  /// Get the alpha value of the color.
  #[must_use]
  #[inline(always)]
  pub const fn a(&self) -> f32 {
    self.0.as_vec4().w()
  }

  // Get a mutable reference to the red value of the color.
  #[must_use]
  #[inline(always)]
  pub fn r_mut(&mut self) -> &mut f32 {
    self.0.as_mut_vec4().x_mut()
  }

  // Get a mutable reference to the green value of the color.
  #[must_use]
  #[inline(always)]
  pub fn g_mut(&mut self) -> &mut f32 {
    self.0.as_mut_vec4().y_mut()
  }

  // Get a mutable reference to the blue value of the color.
  #[must_use]
  #[inline(always)]
  pub fn b_mut(&mut self) -> &mut f32 {
    self.0.as_mut_vec4().z_mut()
  }

  // Get a mutable reference to the alpha value of the color.
  #[must_use]
  #[inline(always)]
  pub fn a_mut(&mut self) -> &mut f32 {
    self.0.as_mut_vec4().w_mut()
  }

  /// Gets the red and green components of this color as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn rg(&self) -> &Vec2 {
    self.0.as_vec4().xy()
  }

  /// Gets the green and blue components of this color as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn gb(&self) -> &Vec2 {
    self.0.as_vec4().yz()
  }

  /// Gets the blue and alpha components of this color as a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub const fn ba(&self) -> &Vec2 {
    self.0.as_vec4().zw()
  }

  /// Gets the red, green, and blue components of this color as a [`Vec3`].
  #[must_use]
  #[inline(always)]
  pub const fn rgb(&self) -> &Vec3 {
    self.0.as_vec4().xyz()
  }

  /// Gets the green, blue, and alpha components of this color as a [`Vec3`].
  #[must_use]
  #[inline(always)]
  pub const fn gba(&self) -> &Vec3 {
    self.0.as_vec4().yzw()
  }

  /// Gets a mutable reference to the red and green components of this color as
  /// a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub fn rg_mut(&mut self) -> &mut Vec2 {
    self.0.as_mut_vec4().xy_mut()
  }

  /// Gets a mutable reference to the green and blue components of this color as
  /// a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub fn gb_mut(&mut self) -> &mut Vec2 {
    self.0.as_mut_vec4().yz_mut()
  }

  /// Gets a mutable reference to the blue and alpha components of this color as
  /// a [`Vec2`].
  #[must_use]
  #[inline(always)]
  pub fn ba_mut(&mut self) -> &mut Vec2 {
    self.0.as_mut_vec4().zw_mut()
  }

  /// Gets a mutable reference to the red, green, and blue components of this
  /// color as a [`Vec3`].
  #[must_use]
  #[inline(always)]
  pub fn rgb_mut(&mut self) -> &mut Vec3 {
    self.0.as_mut_vec4().xyz_mut()
  }

  /// Gets a mutable reference to the green, blue, and alpha components of this
  /// color as a [`Vec3`].
  #[must_use]
  #[inline(always)]
  pub fn gba_mut(&mut self) -> &mut Vec3 {
    self.0.as_mut_vec4().yzw_mut()
  }

  /// Blends this color with another color, returning the blended result.
  ///
  /// Blending makes heavy use of the alpha channel to determine how two colors
  /// should be combined. The alpha channel of the other color is used to
  /// determine how much of the other color should be blended with this color.
  ///
  /// If the alpha cannel of the other color is 1.0, then the other color will
  /// be fully blended with this color. If the alpha channel of the other color
  /// is 0.0, then the other color will not be blended with this color at all.
  ///
  /// # Parameters
  ///
  /// * `other` - The other color to blend with.
  pub fn blend(&self, other: &Self) -> Self {
    let diff = 1.0 - other.a();

    let alpha_delta = self.a() * diff;
    let alpha = alpha_delta + other.a();
    if alpha == 0.0 {
      Color::default()
    } else {
      Color::new(
        ((self.r() * alpha_delta) + (other.r() * other.a())) / alpha,
        ((self.g() * alpha_delta) + (other.g() * other.a())) / alpha,
        ((self.b() * alpha_delta) + (other.b() * other.a())) / alpha,
        alpha,
      )
    }
  }
}

// Modifiers

impl Color {
  /// Set the red value of the color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new red value.
  #[inline(always)]
  pub fn set_r(&mut self, value: f32) {
    self.0.as_mut_vec4().set_x(value);
  }

  /// Set the green value of the color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new green value.
  #[inline(always)]
  pub fn set_g(&mut self, value: f32) {
    self.0.as_mut_vec4().set_y(value);
  }

  /// Set the blue value of the color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new blue value.
  #[inline(always)]
  pub fn set_b(&mut self, value: f32) {
    self.0.as_mut_vec4().set_z(value);
  }

  /// Set the alpha value of the color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new alpha value.
  #[inline(always)]
  pub fn set_a(&mut self, value: f32) {
    self.0.as_mut_vec4().set_w(value);
  }

  /// Sets the red and green components of this color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new red and green values.
  #[inline(always)]
  pub fn set_rg(&mut self, value: &Vec2) {
    self.0.as_mut_vec4().set_xy(value);
  }

  /// Sets the green and blue components of this color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new green and blue values.
  #[inline(always)]
  pub fn set_gb(&mut self, value: &Vec2) {
    self.0.as_mut_vec4().set_yz(value);
  }

  /// Sets the blue and alpha components of this color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new blue and alpha values.
  #[inline(always)]
  pub fn set_ba(&mut self, value: &Vec2) {
    self.0.as_mut_vec4().set_zw(value);
  }

  /// Sets the red, green, and blue components of this color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new red, green, and blue values.
  #[inline(always)]
  pub fn set_rgb(&mut self, value: &Vec3) {
    self.0.as_mut_vec4().set_xyz(value);
  }

  /// Sets the green, blue, and alpha components of this color.
  ///
  /// # Parameters
  ///
  /// * `value` - The new green, blue, and alpha values.
  #[inline(always)]
  pub fn set_gba(&mut self, value: &Vec3) {
    self.0.as_mut_vec4().set_yzw(value);
  }
}

// Conversions

impl Color {
  /// Convert this color to a [`Vec4`].
  pub const fn as_vec4(&self) -> &Vec4 {
    self.0.as_vec4()
  }

  /// Convert this color to a mutable [`Vec4`].
  pub fn as_mut_vec4(&mut self) -> &mut Vec4 {
    self.0.as_mut_vec4()
  }

  /// Convert this color to a pointer to [`f32`]
  pub const fn as_ptr(&self) -> *const f32 {
    self.0.as_vec4().as_ptr()
  }

  /// Convert this color to a mutable pointer to [`f32`].
  pub fn as_mut_ptr(&mut self) -> *mut f32 {
    self.0.as_mut_vec4().as_mut_ptr()
  }

  /// Convert this color to a 32-bit RGB value.
  pub fn to_rgba32(&self) -> u32 {
    let r = (self.r() * 255.0) as u32;
    let g = (self.g() * 255.0) as u32;
    let b = (self.b() * 255.0) as u32;
    let a = (self.a() * 255.0) as u32;
    (r << 24) | (g << 16) | (b << 8) | a
  }

  /// Convert this color to a 64-bit RGBA value.
  pub fn to_rgba64(&self) -> u64 {
    let r = (self.r() * 65535.0) as u64;
    let g = (self.g() * 65535.0) as u64;
    let b = (self.b() * 65535.0) as u64;
    let a = (self.a() * 65535.0) as u64;
    (r << 48) | (g << 32) | (b << 16) | a
  }

  /// Convert this color to a 32-bit ARGB value.
  pub fn to_argb32(&self) -> u32 {
    let a = (self.a() * 255.0) as u32;
    let r = (self.r() * 255.0) as u32;
    let g = (self.g() * 255.0) as u32;
    let b = (self.b() * 255.0) as u32;
    (a << 24) | (r << 16) | (g << 8) | b
  }

  /// Convert this color to a 64-bit ARGB value.
  pub fn to_argb64(&self) -> u64 {
    let a = (self.a() * 65535.0) as u64;
    let r = (self.r() * 65535.0) as u64;
    let g = (self.g() * 65535.0) as u64;
    let b = (self.b() * 65535.0) as u64;
    (a << 48) | (r << 32) | (g << 16) | b
  }
}

impl Borrow<Vec4> for Color {
  fn borrow(&self) -> &Vec4 {
    &self.0
  }
}

impl BorrowMut<Vec4> for Color {
  fn borrow_mut(&mut self) -> &mut Vec4 {
    &mut self.0
  }
}

impl AsRef<Vec4> for Color {
  fn as_ref(&self) -> &Vec4 {
    &self.0
  }
}

impl AsMut<Vec4> for Color {
  fn as_mut(&mut self) -> &mut Vec4 {
    &mut self.0
  }
}

// Formatting

impl fmt::Debug for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Color")
      .field("r", &self.r())
      .field("g", &self.g())
      .field("b", &self.b())
      .field("a", &self.a())
      .finish()
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Color({:.3}, {:.3}, {:.3}, {:.3})",
      self.r(),
      self.g(),
      self.b(),
      self.a()
    )
  }
}

impl fmt::LowerHex for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "#{:02x}{:02x}{:02x}{:02x}",
      (self.r() * 255.0) as u8,
      (self.g() * 255.0) as u8,
      (self.b() * 255.0) as u8,
      (self.a() * 255.0) as u8,
    )
  }
}

#[cfg(test)]
#[path = "color.test.rs"]
mod test;
