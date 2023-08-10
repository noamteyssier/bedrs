use crate::traits::{Container, IntervalBounds, ValueBounds};

pub struct MergeResults<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    intervals: Vec<I>,
    clusters: Vec<usize>,
    n_clusters: usize,
    max_len: Option<T>,
    is_sorted: bool,
}

impl<T, I> Container<T, I> for MergeResults<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    fn new(records: Vec<I>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            intervals: records,
            clusters: Vec::new(),
            n_clusters: 0,
            max_len,
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

impl<T, I> MergeResults<T, I>
where
    I: IntervalBounds<T>,
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
