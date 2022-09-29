use crate::{traits::Container, types::Interval};

pub struct MergeResults<T> {
    intervals: Vec<Interval<T>>,
    clusters: Vec<usize>,
    n_clusters: usize,
}
impl<T> MergeResults<T> {
    pub fn new(intervals: Vec<Interval<T>>, clusters: Vec<usize>) -> Self {
        let n_clusters = clusters.iter().max().unwrap_or(&0) + 1;
        Self::from_raw_parts(intervals, clusters, n_clusters)
    }
    pub fn from_raw_parts(
        intervals: Vec<Interval<T>>,
        clusters: Vec<usize>,
        n_clusters: usize,
    ) -> Self {
        Self {
            intervals,
            clusters,
            n_clusters,
        }
    }
    pub fn intervals(&self) -> &Vec<Interval<T>> {
        &self.intervals
    }
    pub fn clusters(&self) -> &Vec<usize> {
        &self.clusters
    }
    pub fn n_clusters(&self) -> usize {
        self.n_clusters
    }
}
impl<T> Container<T, Interval<T>> for MergeResults<T> {
    fn records(&self) -> &Vec<Interval<T>> {
        &self.intervals
    }
}
