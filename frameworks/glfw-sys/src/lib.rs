//! This crate provides a statically-linked definition of the GLFW library
//! from source.
//!
//! The generated bindings are

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(missing_docs)]
#![allow(unused_results)]
#![allow(rust_2018_idioms)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
