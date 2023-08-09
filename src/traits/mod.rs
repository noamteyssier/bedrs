use num_traits::Zero;
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

pub mod container;
pub mod errors;
pub mod interval;
pub use container::{Bound, Container, Find, Internal, Merge, Sample, SetSubtract};
pub use errors::SetError;
pub use interval::{Coordinates, Intersect, Overlap, Subtract};

/// Generic bounds for values to be used for [Coordinates]
pub trait ValueBounds
where
    Self: Copy
        + Default
        + Ord
        + Debug
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Zero,
{
}
impl<T> ValueBounds for T where
    T: Copy
        + Default
        + Ord
        + Debug
        + Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Zero
{
}

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
