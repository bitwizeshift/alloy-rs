//! Raw OpenGL bindings for rust.
//!
//! This crate provides raw OpenGL bindings for the Rust programming language.
//! The underlying wrangling of the OpenGL bindings may be implemented through
//! other packages like the [`glew-sys`] crate -- and so it's required to
//! initialize the underlying OpenGL bindings before using this crate.
//!
//! [`glew-sys`]: ::glew_sys
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod glew;

#[doc(inline)]
pub use glew::*;
