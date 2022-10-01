use std::fmt::Debug;

pub mod container;
pub mod interval;
pub use container::{Container, Find, Merge, Bound};
pub use interval::{Coordinates, Overlap};

pub trait ValueBounds
where
    Self: Copy + Default + Ord + Debug,
{
}
impl<T: Copy + Default + Ord + Debug> ValueBounds for T {}

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
