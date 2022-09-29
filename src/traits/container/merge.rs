use crate::{traits::{Coordinates, Overlap}, types::Interval};
use super::Container;

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
    pub fn from_raw_parts(intervals: Vec<Interval<T>>, clusters: Vec<usize>, n_clusters: usize) -> Self {
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

pub trait Merge<T, I>: Container<T, I>
where
    T: Copy + PartialOrd + Ord,
    I: Coordinates<T>
{
    fn merge(&self) -> MergeResults<T> {
        let mut base_interval = Interval::from(&self.records()[0]);

        let mut cluster_intervals = Vec::with_capacity(self.len());
        let mut cluster_ids = Vec::with_capacity(self.len());
        let mut current_id = 0;

        for interval in self.records().iter() {
            if base_interval.overlaps(interval) {
                let new_min = *base_interval.start().min(interval.start());
                let new_max = *base_interval.end().max(interval.end());
                base_interval.update_start(&new_min);
                base_interval.update_end(&new_max);
            } else {
                cluster_intervals.push(base_interval.to_owned());
                base_interval.update_start(interval.start());
                base_interval.update_end(interval.end());
                current_id += 1;
            }
            cluster_ids.push(current_id);
        }
        cluster_intervals.push(base_interval.to_owned());
        MergeResults::new(cluster_intervals, cluster_ids)
    }
}

#[cfg(test)]
mod testing {
    use crate::{types::IntervalSet, traits::Coordinates};
    use super::Merge;

    #[test]
    fn test_merging_one_cluster() {
        let starts = vec![
            10,
            15,
            25,
        ];
        let ends = vec![
            30,
            20,
            30,
        ];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let merge_set = set.merge();
        assert_eq!(merge_set.n_clusters(), 1);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        assert_eq!(merge_set.intervals()[0].start(), &10);
        assert_eq!(merge_set.intervals()[0].end(), &30);
    }

    #[test]
    fn test_merging_two_clusters() {
        let starts = vec![
            10,
            15,
            25,
            35,
            40,
        ];
        let ends = vec![
            30,
            20,
            30,
            50,
            45,
        ];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let merge_set = set.merge();
        assert_eq!(merge_set.n_clusters(), 2);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0, 1, 1]);
        assert_eq!(merge_set.intervals()[0].start(), &10);
        assert_eq!(merge_set.intervals()[0].end(), &30);
        assert_eq!(merge_set.intervals()[1].start(), &35);
        assert_eq!(merge_set.intervals()[1].end(), &50);
    }
}
