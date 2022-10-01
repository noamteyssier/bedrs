#![warn(clippy::pedantic, clippy::perf)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc
)]

//! # BEDrs
//! bedtools-like functionality for interval sets in rust
//!
//! ## Summary
//!
//! I wanted some bedtools-like functionality in rust and I made this tool to both
//! learn how to implement genomic arithmetic as well as get more comfortable with
//! generics and traits in rust.
//!
//! This library will eventually be focused towards genome-specific arithmetic
//! and focuses around a base [Coordinates] trait which includes methods to
//! retrieve `<chr, start, stop>` and a base [Container] trait which includes
//! methods to perform set operations.
//!
//! This is a work in progress and is subject to heavy changes.
//!
//! If you want a more robust interval library I recommend the following:
//!
//! - [rust_lapper](https://crates.io/crates/rust-lapper)
//! - [COITrees](https://crates.io/crates/coitrees)
//! - [rampart](https://crates.io/crates/rampart)
//!
//! This library is heavily inspired from those above.
//!
//! ## Traits
//!
//! The advantage of this library is that all of the functionality is implemented
//! via traits.
//! As a result, if you have a custom data type - if you implement the traits then
//! you get all the functionality for free. There are two main traits in this library.
//!
//! 1. [Coordinates] :: which applies to individual interval records.
//! 2. [Container] :: which applies to sets of interval records.
//!
//! ## Types
//!
//! This library has batteries included and has a few types you can use immediately or
//! as references for designing your own.
//!
//! ### Base Interval
//!
//! Here is an example of the base [Interval](types::Interval).
//!
//! This interval only has two coordinates: `start` and `end`.
//! This is the classic interval type.
//!
//! ```
//! use bedrs::{Overlap, Interval};
//!
//! let a = Interval::new(10, 20);
//! let b = Interval::new(15, 25);
//! let c = Interval::new(20, 30);
//!
//! assert!(a.overlaps(&b));
//! assert!(!a.overlaps(&c));
//! assert!(b.overlaps(&c));
//! ```
//!
//! ### Genomic Interval
//!
//! Here is an example of a [GenomicInterval](types::GenomicInterval).
//!
//! This is the bread and butter of genomic arithmetic and has three
//! coordinates: `chr`, `start`, and `end`.
//!
//! ```
//! use bedrs::{Overlap, GenomicInterval};
//!
//! let a = GenomicInterval::new(1, 10, 20);
//! let b = GenomicInterval::new(1, 15, 25);
//! let c = GenomicInterval::new(2, 15, 25);
//!
//! assert!(a.overlaps(&b));
//! assert!(!a.overlaps(&c));
//! assert!(!b.overlaps(&c));
//! ```

/// Traits used within the library
pub mod traits;

/// Types used within the library
pub mod types;

pub use traits::{Bound, Container, Coordinates, Find, Intersect, Merge, Overlap, Subtract};
pub use types::{GenomicInterval, GenomicIntervalSet, Interval, IntervalSet};
