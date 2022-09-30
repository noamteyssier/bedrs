use std::fmt::Debug;

pub mod container;
pub mod interval;
pub use container::{Container, Find, Merge};
pub use interval::{Coordinates, Overlap};

pub trait ValueBounds: 
where
    Self: Copy + Default + Ord + Debug 
{}
impl <T: Copy + Default + Ord + Debug> ValueBounds for T {}

pub trait IntervalBounds<T>
where
    Self: Ord + Coordinates<T> + Clone + Overlap<T> + Copy,
    T: ValueBounds
{}
impl <I, T> IntervalBounds<T> for I
where
    I: Ord + Coordinates<T> + Clone + Overlap<T> + Copy,
    T: ValueBounds,
{}
