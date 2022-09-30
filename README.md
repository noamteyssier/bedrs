# bedrs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/bedrs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/bedrs/branch/main/graph/badge.svg?token=CZANC7RKWP)](https://codecov.io/gh/noamteyssier/bedrs)
![Crates.io](https://img.shields.io/crates/v/bedrs)

bedtools-like functionality for interval sets in rust

## Summary

I wanted some bedtools-like functionality in rust and I made this tool to both
learn how to implement genomic arithmetic as well as get more comfortable with
generics and traits in rust.

This library will eventually be focused towards genome-specific arithmetic
and focuses around a base `Coordinates` trait which includes functions to
retrieve `<chr, start, stop>`.

This is a work in progress and is subject to heavy changes.

If you want a more robust interval library I recommend the following:

- [rust_lapper](https://crates.io/crates/rust-lapper)
- [COITrees](https://crates.io/crates/coitrees)
- [rampart](https://crates.io/crates/rampart)

This library is heavily inspired from those above.

## Usage

The main benefit of this library is that it is trait-based.
So you can define your own types - but if they implement the
`Coordinates` trait they can use the other functions within the
library.

### `Coordinates` Trait

The library centers around the `Coordinates` trait.

```rust
pub trait Coordinates<T>
where
    T: Copy + Default,
{
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn chr(&self) -> T;
    fn update_start(&mut self, val: &T);
    fn update_end(&mut self, val: &T);
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
    fn chr(&self) -> usize {
        0
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
