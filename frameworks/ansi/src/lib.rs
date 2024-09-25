//! This crate provides a wrapper around the underlying imgui implementation.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};

lazy_static! {
  static ref ENABLED: bool = {
    // The double-negative is not ideal, but this is the "common" pattern with
    // CLI tooling; usually NOCOLOR or NO_COLOR can disable formatting.
    // This checks if this is set to a positive value, otherwise defaults to
    // assuming that this is enabled.
    if let Ok(value) = std::env::var("NOCOLOR") {
      !is_true_str(value.as_str())
    } else if let Ok(value) = std::env::var("NO_COLOR") {
      !is_true_str(value.as_str())
    } else {
      // Assume default enabled
      true
    }
  };
}

#[inline]
fn is_true_str(s: &str) -> bool {
  matches!(s, "true" | "TRUE" | "1")
}

/// The ANSI Escape character, which is octal 033.
const ESCAPE: char = 0o33 as char;

/// Represents an ANSI Attribute value. This is always a 1-byte op-code.
pub struct Attribute(pub(crate) u8);

impl Display for Attribute {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}[{}m", ESCAPE, self.0)
  }
}

/// A crate for containing various ANSI formatting elements.
///
/// This is organized into two sub-modules: [`bg`] for background colors, and
/// [`fg`] for foreground colors. Root level formatting operations like bolding,
/// font-style, etc are all in this [`fmt`] root module.
pub mod fmt {
  use super::*;

  /// All attributes become turned off.
  pub const RESET: Attribute = Attribute(0);

  /// Bold or increased intensity.
  pub const BOLD: Attribute = Attribute(1);

  /// Faint, decreased intensity, or dim.
  pub const DIM: Attribute = Attribute(2);

  /// Makes text italicized.
  ///
  /// # Platform Support
  ///
  /// Not widely supported. Sometimes treated as inverse or blink.
  pub const ITALIC: Attribute = Attribute(3);

  /// Underlines the text.
  ///
  /// # Platform Support
  ///
  /// Style extensions exist for Kitty, VTE, mintty, iTerm2 and Konsole.
  pub const UNDERLINE: Attribute = Attribute(4);

  /// Sets blinking to less than 150 times per minute.
  ///
  /// # Platform Support
  ///
  /// Widely supported
  pub const SLOW_BLINK: Attribute = Attribute(5);

  /// Sets blinking to more than 150 times per minute.
  ///
  /// # Platform Support
  ///
  /// Not widely supported.
  pub const RAPID_BLINK: Attribute = Attribute(6);

  /// Conceals or hides the text.
  ///
  /// # Platform Support
  ///
  /// Not widely supported.
  pub const CONCEAL: Attribute = Attribute(8);

  /// Reveals previously concealed or hidden text.
  ///
  /// # Platform Support
  ///
  /// Not widely supported.
  pub const REVEAL: Attribute = Attribute(28);

  /// Crosses out/strikes through text.
  ///
  /// Characters legible but marked as if for deletion.
  ///
  /// # Platform Support
  ///
  /// Not supported in `Terminal.app`.
  pub const STRIKETHROUGH: Attribute = Attribute(9);

  /// The [`fg`] module provides specific format attributes that apply to
  /// foreground elements.
  pub mod fg {
    use super::*;

    /// A pure black foreground color.
    pub const BLACK: Attribute = Attribute(30);

    /// A dark red foreground color.
    pub const RED: Attribute = Attribute(31);

    /// A dark green foreground color.
    pub const GREEN: Attribute = Attribute(32);

    /// A dark yellow foreground color.
    pub const YELLOW: Attribute = Attribute(33);

    /// A dark blue foreground color.
    pub const BLUE: Attribute = Attribute(34);

    /// A dark magenta foreground color.
    pub const MAGENTA: Attribute = Attribute(35);

    /// A dark cyan foreground color.
    pub const CYAN: Attribute = Attribute(36);

    /// A white foreground color.
    ///
    /// This is not a "pure" white; for that, see [`BRIGHT_WHITE`].
    pub const WHITE: Attribute = Attribute(37);

    /// A gray foreground color.
    ///
    /// This is also known as `BRIGHT_BLACK` in some systems.
    pub const GRAY: Attribute = Attribute(90);

    /// A bright red foreground color.
    pub const BRIGHT_RED: Attribute = Attribute(91);

    /// A bright green foreground color.
    pub const BRIGHT_GREEN: Attribute = Attribute(92);

    /// A bright yellow foreground color.
    pub const BRIGHT_YELLOW: Attribute = Attribute(93);

    /// A bright blue foreground color.
    pub const BRIGHT_BLUE: Attribute = Attribute(94);

    /// A bright magenta foreground color.
    pub const BRIGHT_MAGENTA: Attribute = Attribute(95);

    /// A bright cyan foreground color.
    pub const BRIGHT_CYAN: Attribute = Attribute(96);

    /// A bright white foreground color.
    pub const BRIGHT_WHITE: Attribute = Attribute(97);

    /// Resets the foreground color.
    pub const RESET: Attribute = Attribute(39);
  }

  /// The [`bg`] module provides specific format attributes that apply to
  /// background elements.
  pub mod bg {
    use super::*;

    /// A pure black background color.
    pub const BLACK: Attribute = Attribute(40);

    /// A dark red background color.
    pub const RED: Attribute = Attribute(41);

    /// A dark green background color.
    pub const GREEN: Attribute = Attribute(42);

    /// A dark yellow background color.
    pub const YELLOW: Attribute = Attribute(43);

    /// A dark blue background color.
    pub const BLUE: Attribute = Attribute(44);

    /// A dark magenta background color.
    pub const MAGENTA: Attribute = Attribute(45);

    /// A dark cyan background color.
    pub const CYAN: Attribute = Attribute(46);

    /// A white background color.
    ///
    /// This is not a "Pure" white; for that
    pub const WHITE: Attribute = Attribute(47);

    /// A gray background color.
    ///
    /// This is not a "pure" white; for that, see [`BRIGHT_WHITE`].
    pub const GRAY: Attribute = Attribute(100);

    /// A bright red background color.
    pub const BRIGHT_RED: Attribute = Attribute(101);

    /// A bright green background color.
    pub const BRIGHT_GREEN: Attribute = Attribute(102);

    /// A bright yellow background color.
    pub const BRIGHT_YELLOW: Attribute = Attribute(103);

    /// A bright blue background color.
    pub const BRIGHT_BLUE: Attribute = Attribute(104);

    /// A bright magenta background color.
    pub const BRIGHT_MAGENTA: Attribute = Attribute(105);

    /// A bright cyan background color.
    pub const BRIGHT_CYAN: Attribute = Attribute(106);

    /// A bright white background color.
    pub const BRIGHT_WHITE: Attribute = Attribute(107);

    /// Resets the background color.
    pub const RESET: Attribute = Attribute(49);
  }
}

/// A collection of ANSI [`Attribute`]s that can be applied to formatting or
/// textual elements.
///
/// This will disable itself if the environment variable `NOCOLOR` or `NO_COLOR`
/// is defined and set to some truthy value.
pub struct Format<'a>(&'a [Attribute]);

impl Default for Format<'_> {
  fn default() -> Self {
    NO_FORMAT
  }
}

/// A default [`Format`] object that does not have any formatting values.
pub const NO_FORMAT: Format<'static> = Format::from_slice(&[]);

impl<'a> Format<'a> {
  /// Constructs a [`Format`] object from a slice containing [`Attribute`]
  /// instances.
  pub const fn from_slice(attributes: &'a [Attribute]) -> Self {
    Self(attributes)
  }

  /// Wraps a formatted [`Display`] element into a [`FormatDisplay`] that
  /// will automatically toggle the ANSI formatting before displaying the
  /// element, and then follow it up with a [`fmt::RESET`].
  ///
  /// # Parameters
  ///
  /// * `display` - the element to display
  pub const fn wrap<'b>(&'a self, display: &'b dyn Display) -> FormatDisplay<'a, 'b> {
    FormatDisplay {
      format: self,
      display,
    }
  }

  /// Computes and returns the optimal format for the specified `writer`
  ///
  /// This will disable formatting if the `writer` does not support ANSI
  /// formatting (determined by checking [`IsTerminal`])
  ///
  /// # Parameters
  ///
  /// * `writer` - the writer to write to
  ///
  /// [`IsTerminal`]: std::io::IsTerminal
  pub fn optimize_for<W>(self, writer: &W) -> Self
  where
    W: std::io::Write + std::io::IsTerminal,
  {
    if self.can_format(writer) {
      self
    } else {
      NO_FORMAT
    }
  }

  /// Checks whether formatting is available for the given `writer`
  ///
  /// # Parameters
  ///
  /// * `writer` - the writer to check formatting availability.
  fn can_format<W>(&self, writer: &W) -> bool
  where
    W: std::io::Write + std::io::IsTerminal,
  {
    writer.is_terminal()
  }
}

impl<'a> Display for Format<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if *ENABLED {
      if self.0.is_empty() {
        return Ok(());
      }

      write!(f, "{}[", ESCAPE)?;
      for attribute in self.0.iter() {
        write!(f, "{};", attribute.0)?;
      }
      write!(f, "m")
    } else {
      Ok(())
    }
  }
}

/// A [`Display`] element that will automatically use [`Format`] and reset the
/// format after.
pub struct FormatDisplay<'a, 'b> {
  format: &'a Format<'a>,
  display: &'b dyn Display,
}

/// A wrapper around [`fmt::RESET`], used to leverage the test for whether
/// ANSI formatting is enabled.
const RESET_FORMAT: Format<'_> = format![fmt::RESET];

impl<'a, 'b> Display for FormatDisplay<'a, 'b> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.format.fmt(f)?;
    self.display.fmt(f)?;
    RESET_FORMAT.fmt(f)
  }
}

/// A convenience macro for forming a [`Format`] object, analogous to the
/// [`vec`] macro in the standard library.
///
/// # Example
///
/// Basic Use:
///
/// ```rust
/// const ERROR_FORMAT: ansi::Format<'_> = ansi::format![
///   ansi::fmt::fg::RED, ansi::fmt::UNDERLINE, ansi::fmt::BOLD,
/// ];
/// println!("{}", ERROR_FORMAT.wrap(&"Hello"));
/// ```
#[macro_export]
macro_rules! format {
  ($($attribute:expr$(,)?)+) => {
    $crate::Format::from_slice(&[$($attribute,)+])
  };
}
