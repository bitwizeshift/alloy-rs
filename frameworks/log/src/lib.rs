//! This crate provides core definitions that need to be shared between the
//! various wrapper utilities.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(rust_2018_idioms)]

use std::time::SystemTime;

use astd::SourceLocation;

mod severity;

#[doc(inline)]
pub use severity::*;

/// Logs a message of formatted `args` at the specified severity level
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use log::{Logger, Severity};
/// # let mut logger: Logger = Logger::new("log");
/// log::log!(logger, Severity::Debug, "Hello {}!", "world");
/// ```
///
/// Use with metadata:
///
/// ```rust
/// # use log::{Logger, Severity};
/// # let mut logger: Logger = Logger::new("log");
/// log::log!(logger, Severity::Debug, "Hello {}!", "world");
/// ```
#[macro_export]
macro_rules! log {
  ($log:expr, $severity:expr, $($arg:tt)*) => {
    $log.log($severity, std::format_args!($($arg)*), &$crate::metadata!())
  };
  ($log:expr => $md:expr, $severity:expr, $($arg:tt)*) => {
    $log.log($severity, std::format_args!($($arg)*), &$crate::metadata!($md))
  };
}

/// Logs a message of formatted `args` at debug severity
///
/// This is a convenience function that just wraps `Logger::log` with a
/// specified [`Debug`] log severity
///
/// [`Debug`]: Severity::Debug
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::debug!(logger, "Hello {}!", "world");
/// ```
///
/// Use with metadata:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::debug!(logger => log::NoMeta, "Hello {}!", "world");
/// ```
#[macro_export]
macro_rules! debug {
  ($log:expr, $($arg:tt)*) => {
    $log.debug(std::format_args!($($arg)*), &$crate::metadata!())
  };
  ($log:expr => $md:expr, $($arg:tt)*) => {
    $log.debug(std::format_args!($($arg)*), &$crate::metadata!($md))
  };
}

/// Logs a message of formatted `args` at info severity
///
/// This is a convenience function that just wraps `Logger::log` with a
/// specified [`Info`] log severity
///
/// [`Info`]: Severity::Info
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::info!(logger, "Hello {}!", "world");
/// ```
///
/// Use with metadata:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::info!(logger => log::NoMeta, "Hello {}!", "world");
/// ```
#[macro_export]
macro_rules! info {
  ($log:expr, $($arg:tt)*) => {
    $log.info(std::format_args!($($arg)*), &$crate::metadata!())
  };
  ($log:expr => $md:expr, $($arg:tt)*) => {
    $log.info(std::format_args!($($arg)*), &$crate::metadata!($md))
  };
}

/// Logs a message of formatted `args` at warning severity
///
/// This is a convenience function that just wraps `Logger::log` with a
/// specified [`Warning`] log severity
///
/// [`Warning`]: Severity::Warning
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::warning!(logger, "Hello {}!", "world");
/// ```
///
/// Use with metadata:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::warning!(logger => log::NoMeta, "Hello {}!", "world");
/// ```
#[macro_export]
macro_rules! warning {
  ($log:expr, $($arg:tt)*) => {
    $log.warning(std::format_args!($($arg)*), &$crate::metadata!())
  };
  ($log:expr => $md:expr, $($arg:tt)*) => {
    $log.warning(std::format_args!($($arg)*), &$crate::metadata!($md))
  };
}

/// Logs a message of formatted `args` at error severity
///
/// This is a convenience function that just wraps `Logger::log` with a
/// specified [`Error`] log severity
///
/// [`Error`]: Severity::Error
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::error!(logger, "Hello {}!", "world");
/// ```
///
/// Use with metadata:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::error!(logger => log::NoMeta, "Hello {}!", "world");
/// ```
#[macro_export]
macro_rules! error {
  ($log:expr, $($arg:tt)*) => {
    $log.error(std::format_args!($($arg)*), &$crate::metadata!())
  };
  ($log:expr => $md:expr, $($arg:tt)*) => {
    $log.error(std::format_args!($($arg)*), &$crate::metadata!($md))
  };
}

/// Logs a message of formatted `args` at critical severity
///
/// This is a convenience function that just wraps `Logger::log` with a
/// specified [`Critical`] log severity
///
/// [`Critical`]: Severity::Critical
///
/// # Examples
///
/// Basic use:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::critical!(logger, "Hello {}!", "world");
/// ```
///
/// Use with metadata:
///
/// ```rust
/// # use log::Logger;
/// # let mut logger: Logger = Logger::new("log");
/// log::critical!(logger => log::NoMeta, "Hello {}!", "world");
/// ```
#[macro_export]
macro_rules! critical {
  ($log:expr, $($arg:tt)*) => {
    $log.critical(std::format_args!($($arg)*), &$crate::metadata!())
  };
  ($log:expr => $md:expr, $($arg:tt)*) => {
    $log.critical(std::format_args!($($arg)*), &$crate::metadata!($md))
  };
}

/// Empty meta data.
///
/// This is the default for constructed [`Logger`] objects.
pub struct NoMeta;

impl std::fmt::Display for NoMeta {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Ok(())
  }
}

/// [`Meta`] data that can be associated with log sources.
///
///
pub struct Meta<MD = NoMeta> {
  source_location: Option<SourceLocation>,
  time: SystemTime,
  meta: Option<MD>,
}

impl<MD> Meta<MD> {
  ///
  pub fn new(source_location: Option<SourceLocation>, meta: Option<MD>) -> Self {
    Self {
      source_location,
      time: SystemTime::now(),
      meta,
    }
  }
  /// Retrieves a reference to the underlying meta data.
  pub fn data(&self) -> Option<&MD> {
    self.meta.as_ref()
  }

  /// Retrieves a reference to the source location, if this crate was
  /// built with `feature = "log-source"`.
  pub fn source(&self) -> Option<&SourceLocation> {
    self.source_location.as_ref()
  }

  /// Retrieves the time that the meta object was created.
  pub fn time(&self) -> SystemTime {
    self.time
  }
}

/// Constructs a [`Meta`] object that contains the optionally specified
/// data for the metadata.
#[macro_export]
#[doc(hidden)]
macro_rules! metadata {
  () => {
    $crate::Meta::new($crate::source_location!(), None)
  };
  ($e:expr) => {
    $crate::Meta::new($crate::source_location!(), $e.into())
  };
}

/// Constructs an [`Option`] containing a [`SourceLocation`] object if
/// `feature = "log-source"` is enabled, otherwise [`None`]
#[macro_export]
#[cfg(feature = "log-source")]
macro_rules! source_location {
  () => {
    Some(::astd::current_location!())
  };
}

/// Constructs an [`Option`] containing a [`SourceLocation`] object if
/// `feature = "log-source"` is enabled, otherwise [`None`]
#[macro_export]
#[cfg(not(feature = "log-source"))]
macro_rules! source_location {
  () => {
    None
  };
}

struct SinkEntry<MD> {
  sink: Box<dyn Sink<MD>>,
  filter: SeverityFilter,
}

impl<MD> SinkEntry<MD> {
  pub fn new<S: Sink<MD> + 'static>(sink: S) -> Self {
    Self::new_with_filter(sink, SeverityFilter::all())
  }

  pub fn new_with_filter<S: Sink<MD> + 'static>(sink: S, filter: SeverityFilter) -> Self {
    Self {
      sink: Box::new(sink),
      filter,
    }
  }
}

/// An instance of a Logger in this crate.
///
/// [`Logger`] objects may optionally emit meta data, which is free-form data
/// that will be passed down to the underlying [`Sink`] instances. It is up for
/// users of the logger to implement [`Sink`] with a custom meta-data if there
/// is an intent to handle this in some way.
///
/// Actual meta data is always optional for the user to provide. The idiomatic
/// way to provide this metadata is through one of the logging macros:
///
/// * [`debug`]
/// * [`info`]
/// * [`warning`]
/// * [`error`]
/// * [`critical`]
pub struct Logger<MD = NoMeta> {
  // TODO: Add a purpose to the scope.
  _scope: String,
  sinks: Vec<SinkEntry<MD>>,
}

impl<MD> Logger<MD> {
  /// Creates a new instance of a [`Logger`].
  pub fn new(name: &str) -> Self {
    Self {
      _scope: name.to_owned(),
      sinks: Vec::default(),
    }
  }

  /// Adds a [`Sink`] to this [`Logger`].
  ///
  /// Messages that get logged to the [`Logger`] will delegate logging down to
  /// the specified [`Sink`] object(s).
  ///
  /// # Arguments
  ///
  /// * `sink` - the [`Sink`] to write to.
  pub fn sink<S: Sink<MD> + 'static>(&mut self, sink: S) {
    self.sinks.push(SinkEntry::new(sink))
  }

  /// Adds a [`Sink`] to this [`Logger`] that will only be written to if the
  /// `filter` allows it.
  ///
  /// # Arguments
  ///
  /// * `sink` - the [`Sink`] to write to.
  /// * `filter` - the [`SeverityFilter`] to enable writing.
  pub fn sink_with_filter<S: Sink<MD> + 'static>(&mut self, sink: S, filter: SeverityFilter) {
    self.sinks.push(SinkEntry::new_with_filter(sink, filter))
  }
}

impl<MD> Logger<MD> {
  /// Logs a message of formatted `args` at the given `severity`.
  ///
  /// # Arguments
  ///
  /// * `severity` - the log severity level
  /// * `args` - the formatting arguments
  /// * `meta` - additional meta data to optionally provide to the logger.
  pub fn log(&mut self, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    for sink in self.sinks.iter_mut() {
      if sink.filter.allows(severity) {
        // Well this looks terrible
        sink.sink.sink(severity, args, meta);
      }
    }
  }

  /// Logs a message of formatted `args` at debug severity
  ///
  /// This is a convenience function that just wraps `Logger::log` with a
  /// specified [`Debug`] log severity
  ///
  /// [`Debug`]: Severity::Debug
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// log::debug!(logger, "Hello {}!", "world");
  /// ```
  ///
  /// Direct use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// // This is a less-standard example; but it allows for this to be used
  /// // from within other macro definitions.
  /// logger.debug(std::format_args!("hello {}", "world"), &log::metadata!())
  /// ```
  ///
  /// # Arguments
  ///
  /// * `args` - the formatting arguments
  /// * `meta` - additional meta data to optionally provide to the logger.
  #[inline]
  pub fn debug(&mut self, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    self.log(Severity::Debug, args, meta)
  }

  /// Logs a message of formatted `args` at info severity
  ///
  /// This is a convenience function that just wraps `Logger::log` with a
  /// specified [`Info`] log severity
  ///
  /// [`Info`]: Severity::Info
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// log::info!(logger, "Hello {}!", "world");
  /// ```
  ///
  /// Direct use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// // This is a less-standard example; but it allows for this to be used
  /// // from within other macro definitions.
  /// logger.info(std::format_args!("hello {}", "world"), &log::metadata!())
  /// ```
  ///
  /// # Arguments
  ///
  /// * `args` - the formatting arguments
  /// * `meta` - additional meta data to optionally provide to the logger.
  #[inline]
  pub fn info(&mut self, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    self.log(Severity::Info, args, meta)
  }

  /// Logs a message of formatted `args` at warning severity
  ///
  /// This is a convenience function that just wraps `Logger::log` with a
  /// specified [`Warning`] log severity
  ///
  /// [`Warning`]: Severity::Warning
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// log::warning!(logger, "Hello {}!", "world");
  /// ```
  ///
  /// Direct use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// // This is a less-standard example; but it allows for this to be used
  /// // from within other macro definitions.
  /// logger.warning(std::format_args!("hello {}", "world"), &log::metadata!())
  /// ```
  ///
  /// # Arguments
  ///
  /// * `args` - the formatting arguments
  /// * `meta` - additional meta data to optionally provide to the logger.
  #[inline]
  pub fn warning(&mut self, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    self.log(Severity::Warning, args, meta)
  }

  /// Logs a message of formatted `args` at error severity
  ///
  /// This is a convenience function that just wraps `Logger::log` with a
  /// specified [`Error`] log severity
  ///
  /// [`Error`]: Severity::Error
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// log::error!(logger, "Hello {}!", "world");
  /// ```
  ///
  /// Direct use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// // This is a less-standard example; but it allows for this to be used
  /// // from within other macro definitions.
  /// logger.error(std::format_args!("hello {}", "world"), &log::metadata!())
  /// ```
  ///
  /// # Arguments
  ///
  /// * `args` - the formatting arguments
  /// * `meta` - additional meta data to optionally provide to the logger.
  #[inline]
  pub fn error(&mut self, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    self.log(Severity::Error, args, meta)
  }

  /// Logs a message of formatted `args` at critical severity
  ///
  /// This is a convenience function that just wraps `Logger::log` with a
  /// specified [`Critical`] log severity
  ///
  /// [`Critical`]: Severity::Critical
  ///
  /// # Examples
  ///
  /// Basic use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// log::critical!(logger, "Hello {}!", "world");
  /// ```
  ///
  /// Direct use:
  ///
  /// ```rust
  /// # use log::Logger;
  /// # let mut logger: Logger = Logger::new("log");
  /// // This is a less-standard example; but it allows for this to be used
  /// // from within other macro definitions.
  /// logger.critical(std::format_args!("hello {}", "world"), &log::metadata!())
  /// ```
  ///
  /// # Arguments
  ///
  /// * `args` - the formatting arguments
  /// * `meta` - additional meta data to optionally provide to the logger.
  #[inline]
  pub fn critical(&mut self, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    self.log(Severity::Critical, args, meta)
  }
}

/// An output system to log messages to from a [`Logger`].
pub trait Sink<MD = NoMeta> {
  /// Sinks a log message
  ///
  /// # Arguments
  ///
  /// * `saverity` - the log severity
  /// * `args` - the formatted arguments
  /// * `meta` - the metadata to pass to the sink
  fn sink(&mut self, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>);
}

impl<MD> Sink<MD> for std::io::Stdout
where
  MD: std::fmt::Display,
{
  fn sink(&mut self, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    write_log(self, severity, args, meta)
  }
}

impl<'a, MD> Sink<MD> for std::io::StdoutLock<'a>
where
  MD: std::fmt::Display,
{
  fn sink(&mut self, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    write_log(self, severity, args, meta)
  }
}

impl<MD> Sink<MD> for std::io::Stderr
where
  MD: std::fmt::Display,
{
  fn sink(&mut self, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    write_log(self, severity, args, meta)
  }
}

impl<'a, MD> Sink<MD> for std::io::StderrLock<'a>
where
  MD: std::fmt::Display,
{
  fn sink(&mut self, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>) {
    write_log(self, severity, args, meta)
  }
}

const ERROR_FMT: ansi::Format<'_> = ansi::format![ansi::fmt::fg::RED];
const DEBUG_FMT: ansi::Format<'_> = ansi::format![ansi::fmt::fg::GREEN];
const CRITICAL_FMT: ansi::Format<'_> = ansi::format![
  ansi::fmt::UNDERLINE,
  ansi::fmt::SLOW_BLINK,
  ansi::fmt::fg::RED
];
const INFO_FMT: ansi::Format<'_> = ansi::format![ansi::fmt::fg::CYAN];
const WARNING_FMT: ansi::Format<'_> = ansi::format![ansi::fmt::fg::YELLOW];
const SRC_FMT: ansi::Format<'_> = ansi::format![ansi::fmt::fg::GRAY, ansi::fmt::UNDERLINE];

fn write_log<MD, W>(w: &mut W, severity: Severity, args: std::fmt::Arguments<'_>, meta: &Meta<MD>)
where
  W: std::io::Write + std::io::IsTerminal,
  MD: std::fmt::Display,
{
  let severity_fmt = match severity {
    Severity::Debug => DEBUG_FMT,
    Severity::Info => INFO_FMT,
    Severity::Warning => WARNING_FMT,
    Severity::Error => ERROR_FMT,
    Severity::Critical => CRITICAL_FMT,
  }
  .optimize_for(w);
  let source_fmt = SRC_FMT.optimize_for(w);
  let severity = severity_fmt.wrap(&severity);
  let _ = if let Some(source) = meta.source() {
    let source = source_fmt.wrap(&source);
    if let Some(meta) = meta.data() {
      writeln!(w, "{:8} {}: [{}] {}", severity, source, meta, args)
    } else {
      writeln!(w, "{:8} {}: {}", severity, source, args)
    }
  } else if let Some(meta) = meta.data() {
    writeln!(w, "{:8} [{}] {}", severity, meta, args)
  } else {
    writeln!(w, "{:8} {}", severity, args)
  };
}
