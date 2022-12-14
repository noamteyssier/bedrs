use crate::traits::{Container, IntervalBounds, ValueBounds};
use std::marker::PhantomData;

pub struct MergeResults<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    intervals: Vec<I>,
    clusters: Vec<usize>,
    n_clusters: usize,
    phantom: PhantomData<T>,
    is_sorted: bool,
}

impl<T, I> Container<T, I> for MergeResults<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<I>) -> Self {
        Self {
            intervals: records,
            clusters: Vec::new(),
            n_clusters: 0,
            phantom: PhantomData,
            is_sorted: true,
        }
    }
    fn records(&self) -> &Vec<I> {
        &self.intervals
    }
    fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.intervals
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn set_sorted(&mut self) {
        self.is_sorted = true;
    }
}

impl<T, I> MergeResults<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    #[must_use]
    pub fn new(intervals: Vec<I>, clusters: Vec<usize>) -> Self {
        let n_clusters = clusters.iter().max().unwrap_or(&0) + 1;
        Self::from_raw_parts(intervals, clusters, n_clusters)
    }
    #[must_use]
    pub fn from_raw_parts(intervals: Vec<I>, clusters: Vec<usize>, n_clusters: usize) -> Self {
        Self {
            intervals,
            clusters,
            n_clusters,
            phantom: PhantomData,
            is_sorted: true,
        }
    }
    #[must_use]
    pub fn intervals(&self) -> &Vec<I> {
        &self.intervals
    }
    #[must_use]
    pub fn clusters(&self) -> &Vec<usize> {
        &self.clusters
    }
    #[must_use]
    pub fn n_clusters(&self) -> usize {
        self.n_clusters
    }
}
