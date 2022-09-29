use std::fmt::Debug;

use super::Container;
use crate::{
    traits::{Coordinates, GenomicOverlap, Overlap, GenomicCoordinates},
    types::{Interval, MergeResults, GenomicInterval},
};

pub trait Merge<T, I>: Container<T, I>
where
    T: Copy + PartialOrd + Ord + Debug,
    I: Coordinates<T> + Ord,
{
    fn merge(&self) -> MergeResults<T, Interval<T>> {
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
                base_interval = Interval::from(interval);
                current_id += 1;
            }
            cluster_ids.push(current_id);
        }
        cluster_intervals.push(base_interval.to_owned());
        MergeResults::new(cluster_intervals, cluster_ids)
    }
}

pub trait GenomicMerge<T, I>: Container<T, I>
where
    T: Copy + PartialOrd + Ord + Debug,
    I: GenomicCoordinates<T> + Ord,
{
    fn merge(&self) -> MergeResults<T, GenomicInterval<T>> {
        let mut base_interval = GenomicInterval::from(&self.records()[0]);

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
                base_interval = GenomicInterval::from(interval);
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
    use super::{Merge, GenomicMerge};
    use crate::{traits::Coordinates, types::{IntervalSet, GenomicIntervalSet}};

    #[test]
    fn test_merging_one_cluster() {
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let merge_set = set.merge();
        assert_eq!(merge_set.n_clusters(), 1);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        assert_eq!(merge_set.intervals()[0].start(), &10);
        assert_eq!(merge_set.intervals()[0].end(), &30);
    }

    #[test]
    fn test_merging_two_clusters() {
        let starts = vec![10, 15, 25, 35, 40];
        let ends = vec![30, 20, 30, 50, 45];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let merge_set = set.merge();
        assert_eq!(merge_set.n_clusters(), 2);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0, 1, 1]);
        assert_eq!(merge_set.intervals()[0].start(), &10);
        assert_eq!(merge_set.intervals()[0].end(), &30);
        assert_eq!(merge_set.intervals()[1].start(), &35);
        assert_eq!(merge_set.intervals()[1].end(), &50);
    }

    #[test]
    fn test_merging_one_cluster_genomic() {
        let chrs = vec![1, 1, 1];
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let set = GenomicIntervalSet::from_endpoints_unchecked(&chrs, &starts, &ends);
        let merge_set = set.merge();
        assert_eq!(merge_set.n_clusters(), 1);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        assert_eq!(merge_set.intervals()[0].start(), &10);
        assert_eq!(merge_set.intervals()[0].end(), &30);
    }

    #[test]
    fn test_merging_two_cluster_genomic() {
        let chrs = vec![1, 1, 2];
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let set = GenomicIntervalSet::from_endpoints_unchecked(&chrs, &starts, &ends);
        let merge_set = set.merge();
        assert_eq!(merge_set.n_clusters(), 2);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 1]);
        assert_eq!(merge_set.intervals()[0].start(), &10);
        assert_eq!(merge_set.intervals()[0].end(), &30);
    }
}
