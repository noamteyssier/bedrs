use num_traits::{Bounded, FromPrimitive, NumOps, ToPrimitive, Zero};
use std::{fmt::Debug, hash::Hash};

pub mod errors;
pub mod interval;

pub use errors::SetError;
pub use interval::*;

/// Generic bounds for types to be used for [Coordinates] in the context
/// of Chromosome coordinates
pub trait ChromBounds
where
    Self: Clone + Default + Ord + Debug + Send + Sync + Hash,
{
}
impl<T> ChromBounds for T where T: Clone + Default + Ord + Debug + Send + Sync + Hash {}

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

pub trait IntervalBounds<C, T>
where
    Self: GenericIntervalExt<C, T> + Clone + Overlap<C, T> + Send + Sync,
    C: ChromBounds,
    T: Clone,
{
}
impl<I, C, T> IntervalBounds<C, T> for I
where
    I: GenericIntervalExt<C, T> + Clone + Overlap<C, T> + Send + Sync,
    C: ChromBounds,
    T: Clone,
{
}
