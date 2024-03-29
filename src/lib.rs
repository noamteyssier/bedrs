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
//! ### Coordinates
//!
//! The [Coordinates] trait is the base trait for all interval types.
//! It is the trait that defines the `<chr, start, stop>` coordinates.
//! It is also the trait that defines the methods for interval arithmetic.
//! This trait is generic over the type of the coordinates.
//!
//! You can explore the full functionality of this trait by looking at the
//! [crate::traits::interval] module.
//!
//! Some examples of the functionality are:
//! - [Distance](crate::traits::interval::Distance)
//! - [Intersect](crate::traits::interval::Intersect)
//! - [Overlap](crate::traits::interval::Overlap)
//! - [Subtract](crate::traits::interval::Subtract)
//!
//! ### Container
//!
//! The [Container] trait is the base trait for all interval containers.
//! It is the trait that defines the methods for set operations.
//! This trait is generic over the type of the coordinates.
//! It is also generic over the type of the interval.
//! This means that you can have a container of any interval type.
//!
//! You can explore the full functionality of this trait by looking at the
//! [crate::traits::container] module.
//!
//! Some examples of the functionality are:
//! - [Bound](crate::traits::container::Bound)
//! - [Closest](crate::traits::container::Closest)
//! - [Complement](crate::traits::container::Complement)
//! - [Internal](crate::traits::container::Internal)
//! - [Find](crate::traits::container::Find)
//! - [Merge](crate::traits::container::Merge)
//! - [Sample](crate::traits::container::Sample)
//! - [SetSubtract](crate::traits::container::SetSubtract)
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
//!
//! ## Interval Operations
//!
//! The following operations with be shown with the base [Interval], but
//! the same operations can be done with a [GenomicInterval] or any other
//! custom type which implements the [Coordinates] trait.
//!
//! ### Overlap
//! ```
//! use bedrs::{Overlap, Interval};
//!
//! let a = Interval::new(10, 20);
//! let b = Interval::new(15, 25);
//! assert!(a.overlaps(&b));
//! ```
//!
//! ### Containment
//!
//! Whether an interval is contained by another.
//!
//! ```
//! use bedrs::{Overlap, Interval};
//!
//! let a = Interval::new(10, 30);
//! let b = Interval::new(15, 25);
//! assert!(a.contains(&b));
//! assert!(b.contained_by(&a));
//! ```
//!
//! ### Borders
//!
//! Whether an interval is bordered by another.
//!
//! ```
//! use bedrs::{Overlap, Interval};
//!
//! let a = Interval::new(10, 30);
//! let b = Interval::new(30, 50);
//! assert!(a.borders(&b));
//! ```
//!
//! ## Interval Functions
//!
//! The following are active functions to generate more intervals using
//! some query intervals.
//!
//! ### Intersect
//!
//! Here is an example of a positive intersect
//!
//! ```
//! use bedrs::{Interval, Coordinates, Intersect};
//!
//! let a = Interval::new(10, 30);
//! let b = Interval::new(20, 40);
//! let ix = a.intersect(&b).unwrap();
//! assert_eq!(ix.start(), 20);
//! assert_eq!(ix.end(), 30);
//! ```
//!
//! Here is an example of a negative intersect
//!
//! ```
//! use bedrs::{Interval, Coordinates, Intersect};
//!
//! let a = Interval::new(10, 30);
//! let b = Interval::new(30, 40);
//! let ix = a.intersect(&b);
//! assert!(ix.is_none());
//! ```
//!
//! ### Subtract
//!
//! The following method subtracts an interval from another.
//! This returns a vector of intervals, as there could be
//! either zero, one, or two possible interval returned.
//!
//! #### Left-Hand Subtraction
//!
//! ```
//! use bedrs::{Interval, Coordinates, Subtract};
//!
//! let a = Interval::new(10, 30);
//! let b = Interval::new(20, 40);
//! let sub = a.subtract(&b).unwrap();
//! assert_eq!(sub.len(), 1);
//! assert_eq!(sub[0].start(), 10);
//! assert_eq!(sub[0].end(), 20);
//! ```
//!
//! #### Right-Hand Subtraction
//!
//! ```
//! use bedrs::{Interval, Coordinates, Subtract};
//!
//! let a = Interval::new(20, 40);
//! let b = Interval::new(10, 30);
//! let sub = a.subtract(&b).unwrap();
//! assert_eq!(sub.len(), 1);
//! assert_eq!(sub[0].start(), 30);
//! assert_eq!(sub[0].end(), 40);
//! ```
//!
//! #### Contained Subtraction
//!
//! ```
//! use bedrs::{Interval, Coordinates, Subtract};
//!
//! let a = Interval::new(10, 40);
//! let b = Interval::new(20, 30);
//! let sub = a.subtract(&b).unwrap();
//! assert_eq!(sub.len(), 2);
//! assert_eq!(sub[0].start(), 10);
//! assert_eq!(sub[0].end(), 20);
//! assert_eq!(sub[1].start(), 30);
//! assert_eq!(sub[1].end(), 40);
//! ```
//!
//! #### Contained-By Subtraction
//!
//! ```
//! use bedrs::{Interval, Coordinates, Subtract};
//!
//! let a = Interval::new(20, 30);
//! let b = Interval::new(10, 40);
//! let sub = a.subtract(&b);
//! assert!(sub.is_none());
//! ```
//!
//! #### No Overlap Subtraction
//!
//! ```
//! use bedrs::{Interval, Coordinates, Subtract};
//!
//! let a = Interval::new(10, 20);
//! let b = Interval::new(20, 30);
//! let sub = a.subtract(&b).unwrap();
//! assert_eq!(sub.len(), 1);
//! assert_eq!(sub[0].start(), 10);
//! assert_eq!(sub[0].end(), 20);
//! ```

/// Traits used within the library
pub mod traits;

/// Types used within the library
pub mod types;

pub use traits::{
    Bound, Closest, Complement, Container, Coordinates, Distance, Find, Internal, Intersect, Merge,
    Overlap, Sample, SetSubtract, Subtract,
};
pub use types::{
    GenomicInterval, GenomicIntervalSet, IntersectIter, Interval, IntervalIterOwned,
    IntervalIterRef, IntervalSet, MergeIter, NamedInterval, Strand, StrandedGenomicInterval,
};
