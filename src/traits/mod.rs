use num_traits::{FromPrimitive, NumOps, ToPrimitive, Zero};
use std::fmt::Debug;

pub mod container;
pub mod errors;
pub mod interval;
pub use container::{
    Bound, Closest, Complement, Container, Find, Internal, Merge, Sample, SetSubtract,
};
pub use errors::SetError;
pub use interval::{Coordinates, Distance, Intersect, Overlap, Subtract};

/// Generic bounds for types to be used for [Coordinates] in the context
/// of Chromosome coordinates
pub trait ChromBounds
where
    Self: Clone + Default + Ord + Debug,
{
}
impl<T> ChromBounds for T where T: Clone + Default + Ord + Debug {}

/// Generic bounds for values to be used for [Coordinates] in the context
/// of numeric values
pub trait ValueBounds
where
    Self: Copy + ChromBounds + NumOps + ToPrimitive + FromPrimitive + Zero,
{
}
impl<T> ValueBounds for T where T: Copy + ChromBounds + NumOps + ToPrimitive + FromPrimitive + Zero {}

/// Generic bounds for coordinates to be used within [Container]s
pub trait IntervalBounds<C, T>
where
    Self: Coordinates<C, T> + Clone + Overlap<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}
impl<I, C, T> IntervalBounds<C, T> for I
where
    I: Coordinates<C, T> + Clone + Overlap<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
}
