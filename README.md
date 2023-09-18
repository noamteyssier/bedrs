# bedrs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/bedrs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/bedrs/branch/main/graph/badge.svg?token=CZANC7RKWP)](https://codecov.io/gh/noamteyssier/bedrs)
[![Crates.io](https://img.shields.io/crates/v/bedrs)](https://crates.io/crates/bedrs)
[![docs.rs](https://img.shields.io/docsrs/bedrs/latest)](https://docs.rs/bedrs/latest/bedrs/)

`bedtools`-like functionality for interval sets in rust

## Summary

This is an interval library written in rust that takes advantage of the trait
system, generics, and monomorphization.

It focuses around two main traits: `Coordinates` and `Container` which when
implemented on an arbitrary type allow for a wide range of genomic interval
arithmetic.

Interval arithmetic can be thought of as set theoretic operations (like intersection,
union, difference, complement, etc.) on intervals with associated chromosomes, strands,
and other genomic markers.

This library facilitates the development of these types of operations on arbitrary types
and lets the user tailor their structures to minimize overhead.

## Usage

The main benefit of this library is that it is trait-based.
So you can define your own types - but if they implement the
`Coordinates` trait they can use the other functions within the
library.

For detailed usage and examples please review the [documentation](https://docs.rs/bedrs/latest/bedrs/).

### `Coordinates` Trait

The library centers around the `Coordinates` trait.

The `ChromBounds` and `ValueBounds` are the minimal trait requirements
for all the types that can be used as the chromosome and interval values.

```rust
pub trait Coordinates<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn chr(&self) -> &C;
    fn update_start(&mut self, val: &T);
    fn update_end(&mut self, val: &T);
    fn update_chr(&mut self, val: &C);
    fn from(other: &Self) -> Self;
}
```

This is so that if you would like to implement your own interval type
you will only need to implement the `Coordinates` trait for your type
and you can use all the functionality of the library.

```rust
// define a custom interval struct for testing
struct CustomInterval {
    left: usize,
    right: usize,
}
impl Coordinates<usize> for CustomInterval {
    fn start(&self) -> usize {
        self.left
    }
    fn end(&self) -> usize {
        self.right
    }
    fn chr(&self) -> &usize {
        &0
    }
    fn update_start(&mut self, val: &usize) {
        self.left = *val;
    }
    fn update_end(&mut self, val: &usize) {
        self.right = *val;
    }
    fn from(other: &Self) -> Self {
        Self {
            left: other.start(),
            right: other.end(),
        }
    }
}
```

### Interval Types

There are some base interval types provided however, which you can use
for reference or directly for your use case.

#### Base Interval

This is a straightforward singular interval type.
It still implements the `chr()` method, but will return the
default of its generic type.

```rust
use bedrs::{Overlap, Interval};

let a = Interval::new(10, 20);
let b = Interval::new(15, 25);
assert!(a.overlaps(&b));
```

#### Genomic Interval

This is the bread and butter of genomic arithmetic.
It is a 3-attribute struct of `[chr, start, stop]`.

```rust
use bedrs::{Overlap, GenomicInterval};

// Initializing two intervals on the same Chr
let a = GenomicInterval::new(1, 10, 20);
let b = GenomicInterval::new(1, 15, 25);
assert!(a.overlaps(&b));

// Initializing two intervals on different Chr
let a = GenomicInterval::new(1, 10, 20);
let b = GenomicInterval::new(2, 15, 25);
assert!(!a.overlaps(&b));
```

#### Stranded Genomic Interval

This is another version of the genomic interval which includes strand information.
It is a 4-attribute struct of `[chr, start, stop, strand]`

```rust
use bedrs::{Overlap, Strand, StrandedGenomicInterval};

// Initializing three intervals on the same Chr with strands
let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);

// All intervals overlap
assert!(a.overlaps(&b));
assert!(a.overlaps(&c));

// Only `a` and `b` overlap on the same strand
assert!(a.stranded_overlaps(&b));
assert!(!a.stranded_overlaps(&c));
```

## Other Work

This library is heavily inspired by other interval libraries in rust
which are listed below:

- [rampart](https://crates.io/crates/rampart)
- [rust_lapper](https://crates.io/crates/rust-lapper)
- [COITrees](https://crates.io/crates/coitrees)
