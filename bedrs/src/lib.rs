#![warn(clippy::pedantic, clippy::perf)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc
)]
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
//! ![actions status](https://github.com/noamteyssier/bedrs/workflows/CI/badge.svg)
//! [![codecov](https://codecov.io/gh/noamteyssier/bedrs/branch/main/graph/badge.svg?token=CZANC7RKWP)](https://codecov.io/gh/noamteyssier/bedrs)
//! [![Crates.io](https://img.shields.io/crates/v/bedrs)](https://crates.io/crates/bedrs)
//! [![docs.rs](https://img.shields.io/docsrs/bedrs/latest)](https://docs.rs/bedrs/latest/bedrs/)
//!
//! # bedrs
//!
//! `bedtools`-like functionality for interval sets in rust
//!
//! ## Summary
//!
//! This is an interval library written in rust that takes advantage of the trait
//! system, generics, monomorphization, and procedural macros, for high efficiency
//! interval operations with nice quality of life features for developers.
//!
//! It focuses around the [`Coordinates`] trait, which once implemented on
//! and arbitrary interval type allows for a wide range of genomic interval arithmetic.
//!
//! It also introduces a new collection type, [`IntervalContainer`], which acts as a collection
//! of [`Coordinates`] and has many set operations implemented.
//!
//! Interval arithmetic can be thought of as set theoretic operations (like intersection,
//! union, difference, complement, etc.) on intervals with associated chromosomes, strands,
//! and other genomic markers.
//!
//! This library facilitates the development of these types of operations on arbitrary types
//! and lets the user tailor their structures to minimize computational overhead, but also
//! remains a flexible library for general interval operations.
//!
//! ## Usage
//!
//! The main benefit of this library is that it is trait-based.
//! So you can define your own types - but if they implement the
//! [`Coordinates`] trait they can use the other functions within the
//! library.
//!
//! For detailed usage and examples please review the [documentation](https://docs.rs/bedrs/latest/bedrs/).
//!
//! ### [`Coordinates`] Trait
//!
//! The library centers around the [`Coordinates`] trait.
//!
//! This trait defines some minimal functions that are required for all set operations.
//! This includes things like getting the chromosome ID of an interval, or the start and
//! endpoints of that interval, or the strand.
//!
//! This can be implemented by hand, or if you follow common naming conventions used in the
//! library (`chr`, `start`, `end`, `strand`) then you can `[derive(Coordinates)]` on your
//! custom interval type.
//!
//! ```rust
//! use bedrs::prelude::*;
//!
//! // define a custom interval struct for testing
//! #[derive(Default, Coordinates)]
//! struct MyInterval {
//!     chr: usize,
//!     start: usize,
//!     end: usize,
//! }
//! ```
//!
//! ### Interval Types
//!
//! While you can create your own interval types, there are plenty of 'batteries-included'
//! types you can use in your own libraries already.
//!
//! These include:
//! - [`Bed3`]
//! - [`Bed4`]
//! - [`Bed6`]
//! - [`Bed12`]
//! - [`BedGraph`]
//! - [`Gtf`]
//! - [`MetaInterval`]
//! - [`StrandedBed3`]
//!
//! These are pre-built interval types and can be used in many usecases:
//!
//! ``` rust
//! use bedrs::prelude::*;
//!
//! // An interval on chromosome 1 and spanning base 20 <-> 40
//! let a = Bed3::new(1, 20, 40);
//!
//! // An interval on chromosome 1 and spanning base 30 <-> 50
//! let b = Bed3::new(1, 30, 50);
//!
//! // Find the intersecting interval of the two
//! // This returns an Option<Bed3> because they may not intersect.
//! let c = a.intersect(&b).unwrap();
//!
//! assert_eq!(c.chr(), &1);
//! assert_eq!(c.start(), 30);
//! assert_eq!(c.end(), 40);
//! ```
//!
//! ## Interval Operations
//!
//! - [`Overlap`]
//! - [`Distance`]
//! - [`Intersect`]
//! - [`Segment`]
//! - [`Subtract`]
//!
//! ## Interval Set Operations
//!
//! Set operations are performed using the methods of the [`IntervalContainer`].
//!
//! We can build an [`IntervalContainer`] easily on any collection of intervals:
//!
//! ``` rust
//! use bedrs::prelude::*;
//!
//! let set = IntervalContainer::new(vec![
//!     Bed3::new(1, 20, 30),
//!     Bed3::new(1, 30, 40),
//!     Bed3::new(1, 40, 50),
//! ]);
//!
//! assert_eq!(set.len(), 3);
//! ```
//!
//! For more details on each of these and more please explore the [`IntervalContainer`] for all
//! associated methods.
//!
//! - Bound
//! - Closest
//! - Complement
//! - Find
//! - Internal
//! - Merge
//! - Sample
//! - Intersect
//! - Segment
//! - Subtract
//!
//! ## Other Work
//!
//! This library is heavily inspired by other interval libraries in rust
//! which are listed below:
//!
//! - [rampart](https://crates.io/crates/rampart)
//! - [rust_lapper](https://crates.io/crates/rust-lapper)
//! - [COITrees](https://crates.io/crates/coitrees)
//!
//! It also was motivated by the following interval toolkits in C++ and C respectively:
//! - [bedtools](https://github.com/arq5x/bedtools2)
//! - [bedops](https://github.com/bedops/bedops)
/// Traits used within the library
pub mod traits;

/// Types used within the library
pub mod types;

/// Prelude for the library
pub mod prelude;

pub use traits::{
    Coordinates, Distance, Intersect, Overlap, Segment, StrandedOverlap, Subtract,
    UnstrandedOverlap,
};
pub use types::{
    BaseInterval, Bed12, Bed3, Bed4, Bed6, BedGraph, Frame, Gtf, IntersectIter, IntervalContainer,
    IntervalIterOwned, IntervalIterRef, MergeIter, MetaInterval, Score, Strand, StrandedBed3,
};
