/// A trait for objects that can be tested for intersection.
pub trait Intersects<Rhs: ?Sized = Self> {
  /// Determines if this object intersects with another object.
  ///
  /// * `other` - The other object to check for intersection against.
  fn intersects(&self, other: &Rhs) -> bool;
}

/// A trait for objects that can be tested for enclosure.
pub trait Encloses<Rhs: ?Sized = Self> {
  /// Determines if this object encloses another object.
  ///
  /// * `other` - The other object to check for enclosure against.
  fn encloses(&self, other: &Rhs) -> bool;
}
