//! The Alloy crate provides the base Alloy Game Engine.
//!
//! Alloy is a flexible game-engine and development kit under active
//! development written in Rust. It is the successor to the C++ project of the
//! [same name](https://github.com/bitwizeshift/alloy), and exists largely as
//! an experiment to gain more deep knowledge of working with Rust and rendering
//! libraries.
//!
//! It is characterized by upholding a strict set of design-goals which center
//! around maintainability and readability through proper software engineering
//! practices. The overall goal for this project is to be a well-rounded and
//! easy-to-use, efficient game-engine for 2.5D graphics projects.
#![deny(missing_docs)]
#![deny(unused_results)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(unused_lifetimes)]
#![deny(rust_2018_idioms)]

// These dependencies will be used, but are not yet implemented.
#[cfg(feature = "opengl")]
use gl as _;
use glfw as _;
#[cfg(feature = "openal")]
use openal as _;

// Core / base modules
pub mod cmp;
pub mod core;
pub mod meta;
pub mod ops;

// Business logic modules
pub mod geometry;
pub mod math;
pub mod model;
