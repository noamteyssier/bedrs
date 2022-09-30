use crate::traits::{Container, Coordinates};
use std::marker::PhantomData;

pub struct MergeResults<T, I>
where
    I: Coordinates<T>,
    T: Copy + Default,
{
    intervals: Vec<I>,
    clusters: Vec<usize>,
    n_clusters: usize,
    phantom: PhantomData<T>,
}

impl<T, I> Container<T, I> for MergeResults<T, I>
where
    I: Coordinates<T> + Ord,
    T: Copy + Default,
{
    fn records(&self) -> &Vec<I> {
        &self.intervals
    }
    fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.intervals
    }
}

impl<T, I> MergeResults<T, I>
where
    I: Coordinates<T>,
    T: Copy + Default,
{
    #[must_use] pub fn new(intervals: Vec<I>, clusters: Vec<usize>) -> Self {
        let n_clusters = clusters.iter().max().unwrap_or(&0) + 1;
        Self::from_raw_parts(intervals, clusters, n_clusters)
    }
    #[must_use] pub fn from_raw_parts(intervals: Vec<I>, clusters: Vec<usize>, n_clusters: usize) -> Self {
        Self {
            intervals,
            clusters,
            n_clusters,
            phantom: PhantomData,
        }
    }
    #[must_use] pub fn intervals(&self) -> &Vec<I> {
        &self.intervals
    }
    #[must_use] pub fn clusters(&self) -> &Vec<usize> {
        &self.clusters
    }
    #[must_use] pub fn n_clusters(&self) -> usize {
        self.n_clusters
    }
}
