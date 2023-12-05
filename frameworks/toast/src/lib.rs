//! This crate provides a simplified mechanism for producing native modal
//! dialog toast messages to the underlying system.
//!
//! Dialogs are formed by creating basic builders which will ultimately create
//! and show the modal dialog until the user responds to the message.
//!
//! This has been achieved through the [`boxer_sys`] Crate, which wraps the
//! original C++ Boxer library in C bindings.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

use boxer_sys as c;

/// The button that was selected by the user
pub enum Selection {
  /// The user selected OK from an [`Buttons::OK`] or [`Buttons::OKCancel`]
  OK,
  /// The user selected Cancel from an [`Buttons::OKCancel`]
  Cancel,
  /// The user selected "Yes" from a [`Buttons::YesNo`]
  Yes,
  /// The user selected "No" from a [`Buttons::YesNo`]
  No,
  /// The user selected "Quit" from a [`Buttons::Quit`]
  Quit,
  /// The user did not select a button
  None,
  /// An internal error occurred.
  Error,
}

impl Selection {
  fn from_c(value: c::BoxerSelection) -> Self {
    match value {
      c::BoxerSelection_OKSelection => Self::OK,
      c::BoxerSelection_CancelSelection => Self::Cancel,
      c::BoxerSelection_YesSelection => Self::Yes,
      c::BoxerSelection_NoSelection => Self::No,
      c::BoxerSelection_QuitSelection => Self::Quit,
      c::BoxerSelection_NoneSelection => Self::None,
      c::BoxerSelection_ErrorSelection => Self::Error,
      _ => Self::Error,
    }
  }
}

/// The buttons to present to the user.
pub enum Buttons {
  /// A typical dialog with just an "OK" button.
  OK,
  /// A typical dialog with "OK" or "Cancel" buttons.
  OKCancel,
  /// A question dialog box with "Yes" or "No" buttons.
  YesNo,
  /// A dialog that only offers a "quit" button
  Quit,
}

impl Buttons {
  fn to_c(self) -> c::BoxerButtons {
    match self {
      Self::OK => c::BoxerButtons_OKButtons,
      Self::OKCancel => c::BoxerButtons_OKCancelButtons,
      Self::YesNo => c::BoxerButtons_YesNoButtons,
      Self::Quit => c::BoxerButtons_QuitButtons,
    }
  }
}

/// The style that the window should be created in.
enum Style {
  Info,
  Warning,
  Error,
  Question,
}

impl Style {
  fn to_c(self) -> c::BoxerStyle {
    match self {
      Self::Info => c::BoxerStyle_InfoStyle,
      Self::Warning => c::BoxerStyle_WarningStyle,
      Self::Error => c::BoxerStyle_ErrorStyle,
      Self::Question => c::BoxerStyle_QuestionStyle,
    }
  }
}

use std::ffi::CStr;

/// The main builder class for creating a dialog window.
pub struct ToastBuilder<'a, 'b> {
  style: Style,
  title: &'a CStr,
  message: &'b CStr,
  buttons: Buttons,
}

impl<'a, 'b> ToastBuilder<'a, 'b> {
  /// Create an info toast builder
  pub fn info() -> Self {
    Self::from_style(Style::Info)
  }

  /// Creates a warning toast builder
  pub fn warning() -> Self {
    Self::from_style(Style::Warning)
  }

  /// Creates an error toast builder
  pub fn error() -> Self {
    Self::from_style(Style::Error)
  }

  /// Creates a question toast builder
  pub fn question() -> Self {
    Self::from_style(Style::Question)
  }

  /// Sets the message string to display to the user.
  ///
  /// # Arguments
  ///
  /// * `message` - the message [`CStr`]` to display.
  pub fn message_cstr(mut self, message: &'b CStr) -> Self {
    self.message = message;
    self
  }

  /// Sets the title string to display to the user.
  ///
  /// # Arguments
  ///
  /// * `title` - the title [`CStr`]` to use.
  pub fn title_cstr(mut self, title: &'a CStr) -> Self {
    self.title = title;
    self
  }

  /// Sets the button style to display to the user.
  ///
  /// # Arguments
  ///
  /// * `buttons` - the buttons to display to the user.
  pub fn buttons(mut self, buttons: Buttons) -> Self {
    self.buttons = buttons;
    self
  }

  /// Show the window
  pub fn show(self) -> Selection {
    Selection::from_c(unsafe {
      Self::show_cstr(self.message, self.title, self.style, self.buttons)
    })
  }

  /// Creates a builder with sensible defaults in the given style.
  ///
  /// # Arguments
  ///
  /// * `s` - the style to create a builder for
  fn from_style(s: Style) -> Self {
    Self {
      style: s,
      title: foundation::cstr!("untitled"),
      message: foundation::cstr!(""),
      buttons: Buttons::OK,
    }
  }

  /// Shows the underlying message to the user.
  unsafe fn show_cstr(message: &CStr, title: &CStr, style: Style, buttons: Buttons) -> u32 {
    c::boxer_show(
      message.as_ptr(),
      title.as_ptr(),
      style.to_c(),
      buttons.to_c(),
    )
  }
}
