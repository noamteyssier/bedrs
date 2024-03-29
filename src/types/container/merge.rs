use std::marker::PhantomData;

use crate::traits::{ChromBounds, Container, IntervalBounds, ValueBounds};

pub struct MergeResults<C, T, I>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    intervals: Vec<I>,
    clusters: Vec<usize>,
    n_clusters: usize,
    max_len: Option<T>,
    is_sorted: bool,
    phantom_c: PhantomData<C>,
}

impl<C, T, I> Container<C, T, I> for MergeResults<C, T, I>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn new(_records: Vec<I>) -> Self {
        unimplemented!("MergeResults overwrites the new() method")
    }
    fn records(&self) -> &Vec<I> {
        &self.intervals
    }
    fn records_mut(&mut self) -> &mut Vec<I> {
        &mut self.intervals
    }
    fn records_owned(self) -> Vec<I> {
        self.intervals
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn sorted_mut(&mut self) -> &mut bool {
        &mut self.is_sorted
    }
    fn max_len(&self) -> Option<T> {
        self.max_len
    }
    fn max_len_mut(&mut self) -> &mut Option<T> {
        &mut self.max_len
    }
}

impl<C, T, I> MergeResults<C, T, I>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    #[must_use]
    pub fn new(intervals: Vec<I>, clusters: Vec<usize>) -> Self {
        let max_len = intervals.iter().map(|iv| iv.len()).max();
        let n_clusters = clusters.iter().max().unwrap_or(&0) + 1;
        Self::from_raw_parts(intervals, clusters, n_clusters, max_len)
    }
    #[must_use]
    pub fn from_raw_parts(
        intervals: Vec<I>,
        clusters: Vec<usize>,
        n_clusters: usize,
        max_len: Option<T>,
    ) -> Self {
        Self {
            intervals,
            clusters,
            n_clusters,
            max_len,
            is_sorted: true,
            phantom_c: PhantomData,
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
