use std::fmt::Debug;

pub mod container;
pub mod interval;
pub use container::{Bound, Container, Find, Merge, SetSubtract};
pub use interval::{Coordinates, Intersect, Overlap, Subtract};

/// Generic bounds for values to be used for [Coordinates]
pub trait ValueBounds
where
    Self: Copy + Default + Ord + Debug,
{
}
impl<T: Copy + Default + Ord + Debug> ValueBounds for T {}

/// Generic bounds for coordinates to be used within [Container]s
pub trait IntervalBounds<T>
where
    Self: Coordinates<T> + Clone + Overlap<T>,
    T: ValueBounds,
{
}
impl<I, T> IntervalBounds<T> for I
where
    I: Coordinates<T> + Clone + Overlap<T>,
    T: ValueBounds,
{
}
