use core::{
  cmp::{Ordering, PartialOrd},
  fmt::{self, Debug},
  hash::{Hash, Hasher},
};

/// A zero-sized type that may be embedded in other types to guarantee an
/// alignment of `N` bytes. Alignment is provided as a const generic parameter,
/// allowing for better configurability.
///
/// ```
/// # use alloy::core::Align;
/// # use std::mem::align_of;
///
/// assert_eq!(align_of::<Align<1>>(), 1);
/// assert_eq!(align_of::<Align<2>>(), 2);
/// assert_eq!(align_of::<Align<4>>(), 4);
/// // etc..
/// ```
///
/// This can be embedded in types to guarantee the alignment. Ordering will
/// generally matter, as the alignment type must be the first field in the
/// struct.
///
/// ```
/// # use alloy::core::Align;
/// # use std::mem::align_of_val;
///
/// struct AlignedArray<T, const N: usize>(Align<16>, [T; N]);
///
/// let aligned = AlignedArray::<f32, 4>(Align::new(), [0.0, 0.0, 0.0, 0.0]);
///
/// assert_eq!(align_of::<Align<1>>(), 1);
/// assert_eq!(align_of::<Align<2>>(), 2);
/// assert_eq!(align_of::<Align<4>>(), 4);
/// ```
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct Align<const N: usize>([<Self as sealed::Alignment>::Type; 0])
where
  Self: Alignment;

/// A trait implemented for the alignment type, allowing for the type to be
/// embedded in other types to guarantee the alignment.
pub trait Alignment: sealed::Alignment {}

impl<const N: usize> Align<N>
where
  Self: Alignment,
{
  /// Create a new instance of the alignment type.
  #[inline(always)]
  pub const fn new() -> Self {
    Self([])
  }
}

mod sealed {
  /// This trait is used internally to map an `Align<N>` to a unit
  /// struct of alignment N.
  pub trait Alignment {
    /// A zero-sized type of particular alignment.
    type Type: Copy + Eq + PartialEq + Send + Sync + Unpin;
  }

  impl super::Alignment for super::Align<1> {}
  impl Alignment for super::Align<1> {
    type Type = Align1;
  }

  impl super::Alignment for super::Align<2> {}
  impl Alignment for super::Align<2> {
    type Type = Align2;
  }

  impl super::Alignment for super::Align<4> {}
  impl Alignment for super::Align<4> {
    type Type = Align4;
  }

  impl super::Alignment for super::Align<8> {}
  impl Alignment for super::Align<8> {
    type Type = Align8;
  }

  impl super::Alignment for super::Align<16> {}
  impl Alignment for super::Align<16> {
    type Type = Align16;
  }

  impl super::Alignment for super::Align<32> {}
  impl Alignment for super::Align<32> {
    type Type = Align32;
  }

  impl super::Alignment for super::Align<64> {}
  impl Alignment for super::Align<64> {
    type Type = Align64;
  }

  impl super::Alignment for super::Align<128> {}
  impl Alignment for super::Align<128> {
    type Type = Align128;
  }

  impl super::Alignment for super::Align<256> {}
  impl Alignment for super::Align<256> {
    type Type = Align256;
  }

  impl super::Alignment for super::Align<512> {}
  impl Alignment for super::Align<512> {
    type Type = Align512;
  }

  impl super::Alignment for super::Align<1024> {}
  impl Alignment for super::Align<1024> {
    type Type = Align1024;
  }

  impl super::Alignment for super::Align<2048> {}
  impl Alignment for super::Align<2048> {
    type Type = Align2048;
  }

  impl super::Alignment for super::Align<4096> {}
  impl Alignment for super::Align<4096> {
    type Type = Align4096;
  }

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(1))]
  pub struct Align1;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(2))]
  pub struct Align2;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(4))]
  pub struct Align4;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(8))]
  pub struct Align8;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(16))]
  pub struct Align16;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(32))]
  pub struct Align32;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(64))]
  pub struct Align64;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(128))]
  pub struct Align128;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(256))]
  pub struct Align256;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(512))]
  pub struct Align512;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(1024))]
  pub struct Align1024;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(2048))]
  pub struct Align2048;

  #[derive(Copy, Clone, Eq, PartialEq)]
  #[repr(align(4096))]
  pub struct Align4096;
}

impl<const N: usize> Copy for Align<N> where Self: Alignment {}

impl<const N: usize> Clone for Align<N>
where
  Self: Alignment,
{
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}

impl<const N: usize> Debug for Align<N>
where
  Self: Alignment,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Align<{}>", N)
  }
}

impl<const N: usize> Default for Align<N>
where
  Self: Alignment,
{
  #[inline(always)]
  fn default() -> Self {
    Self::new()
  }
}

impl<const N: usize> Hash for Align<N>
where
  Self: Alignment,
{
  #[inline(always)]
  fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl<const N: usize> Ord for Align<N>
where
  Self: Alignment,
{
  #[inline(always)]
  fn cmp(&self, _: &Self) -> Ordering {
    Ordering::Equal
  }
}

impl<const N: usize> PartialOrd for Align<N>
where
  Self: Alignment,
{
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
