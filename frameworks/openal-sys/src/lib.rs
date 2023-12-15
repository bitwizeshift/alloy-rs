//! This crate provides a wrapper around the underlying OpenAL implementation.
//!
//! The raw generated bindings can be found in the [`c`] module, or a more
//! hierarchical binding may be leveraged from the rest of this crate.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(missing_docs)]
#![allow(unused_results)]
#![allow(rust_2018_idioms)]
#![allow(rustdoc::broken_intra_doc_links)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
