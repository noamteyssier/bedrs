use num_traits::{Bounded, FromPrimitive, NumOps, ToPrimitive, Zero};
use std::fmt::Debug;

pub mod errors;
pub mod interval;
pub use errors::SetError;
pub use interval::{
    Coordinates, Distance, Intersect, Overlap, Segment, StrandedOverlap, Subtract,
    UnstrandedOverlap,
};

/// Generic bounds for types to be used for [Coordinates] in the context
/// of Chromosome coordinates
pub trait ChromBounds
where
    Self: Clone + Default + Ord + Debug + Send + Sync,
{
}
impl<T> ChromBounds for T where T: Clone + Default + Ord + Debug + Send + Sync {}

/// Generic bounds for values to be used for [Coordinates] in the context
/// of numeric values
pub trait ValueBounds
where
    Self: Copy + ChromBounds + NumOps + ToPrimitive + FromPrimitive + Zero + Bounded + Send + Sync,
{
}
impl<T> ValueBounds for T where
    T: Copy + ChromBounds + NumOps + ToPrimitive + FromPrimitive + Zero + Bounded + Send + Sync
{
}

pub trait MetaBounds
where
    Self: Clone + Default + Debug + Send + Sync,
{
}
impl<T> MetaBounds for T where T: Clone + Default + Debug + Send + Sync {}

pub trait IntervalBounds<C>
where
    Self: Coordinates<C> + Clone + Overlap<C> + Send + Sync,
    C: ChromBounds,
{
}
impl<I, C> IntervalBounds<C> for I
where
    I: Coordinates<C> + Clone + Overlap<C> + Send + Sync,
    C: ChromBounds,
{
}
