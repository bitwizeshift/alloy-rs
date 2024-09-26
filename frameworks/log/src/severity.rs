/// The severity of a given log message.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Severity {
  /// Debug, for
  Debug,
  /// Info, for
  Info,
  /// Warning, for
  Warning,
  /// Error, for
  Error,
  /// Critical, for
  Critical,
}

impl std::fmt::Display for Severity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Severity::Debug => "debug",
      Severity::Info => "info",
      Severity::Warning => "warning",
      Severity::Error => "error",
      Severity::Critical => "critical",
    }
    .fmt(f)
  }
}

/// A filter for log [`Severity`] levels.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct SeverityFilter(u8);

impl SeverityFilter {
  const DEBUG_BIT: u8 = (1 << 0);
  const INFO_BIT: u8 = (1 << 1);
  const WARNING_BIT: u8 = (1 << 2);
  const ERROR_BIT: u8 = (1 << 3);
  const CRITICAL_BIT: u8 = (1 << 4);
  const MASK: u8 = (1 << 5) - 1;

  /// Creates a filter that allows all [`Severity`] values.
  #[inline(always)]
  pub const fn all() -> Self {
    Self::mask(!0)
  }

  /// Creates a filter that allows no [`Severity`] values.
  #[inline(always)]
  pub const fn none() -> Self {
    Self(0)
  }

  /// Creates a filter that only allows the specified [`Severity`] `s`
  ///
  /// # Parameters
  ///
  /// * `s` - the severity.
  #[inline(always)]
  pub const fn only(s: Severity) -> Self {
    Self(Self::bit(s))
  }

  /// Constructs a filter that only allows [`Severity`] values at or above
  /// the specified severity `s`
  ///
  /// # Parameters
  ///
  /// * `s` - the severity.
  #[inline(always)]
  pub const fn above_or_eq(s: Severity) -> Self {
    Self::mask(!(Self::bit(s) - 1))
  }

  /// Constructs a filter tht allows only [`Severity`] values above the specified
  /// severity `s`
  ///
  /// # Parameters
  ///
  /// * `s` - the severity.
  #[inline(always)]
  pub const fn above(s: Severity) -> Self {
    Self::mask(!((Self::bit(s) << 1) - 1))
  }

  /// Constructs a filter that allows only [`Severity`] values at or below the
  /// specified severity `s`.
  ///
  /// # Parameters
  ///
  /// * `s` - the severity.
  #[inline(always)]
  pub const fn below_or_eq(s: Severity) -> Self {
    Self((Self::bit(s) << 1) - 1)
  }

  /// Constructs a filter that allows only [`Severity`] values below the specified
  /// severity `s`.
  ///
  /// # Parameters
  ///
  /// * `s` - the severity.
  #[inline(always)]
  pub const fn below(s: Severity) -> Self {
    Self(Self::bit(s) - 1)
  }

  /// Returns a filter that enables [`Severity::Debug`] messages.
  #[inline(always)]
  pub const fn debug(mut self) -> Self {
    self.0 |= Self::DEBUG_BIT;
    self
  }

  /// Returns a filter that enables [`Severity::Info`] messages.
  #[inline(always)]
  pub const fn info(mut self) -> Self {
    self.0 |= Self::INFO_BIT;
    self
  }

  /// Returns a filter that enables [`Severity::Warning`] messages.
  #[inline(always)]
  pub const fn warning(mut self) -> Self {
    self.0 |= Self::WARNING_BIT;
    self
  }

  /// Returns a filter that enables [`Severity::Error`] messages.
  #[inline(always)]
  pub const fn error(mut self) -> Self {
    self.0 |= Self::ERROR_BIT;
    self
  }

  /// Returns a filter that enables [`Severity::Critical`] messages.
  #[inline(always)]
  pub const fn critical(mut self) -> Self {
    self.0 |= Self::CRITICAL_BIT;
    self
  }

  /// Checks if this filter allows the specified [`Severity`] `s`.
  ///
  /// # Parameters
  ///
  /// * `s` - the severity.
  #[inline(always)]
  pub const fn allows(&self, s: Severity) -> bool {
    self.0 & Self::bit(s) != 0
  }

  /// Constructs a filter from the specified bits.
  ///
  /// # Parameters
  ///
  /// * `bits` - the bits to use.
  #[inline(always)]
  const fn mask(bits: u8) -> Self {
    Self(Self::MASK & bits)
  }

  const fn bit(s: Severity) -> u8 {
    match s {
      Severity::Critical => Self::CRITICAL_BIT,
      Severity::Error => Self::ERROR_BIT,
      Severity::Warning => Self::WARNING_BIT,
      Severity::Info => Self::INFO_BIT,
      Severity::Debug => Self::DEBUG_BIT,
    }
  }
}
