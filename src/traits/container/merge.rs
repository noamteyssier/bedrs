use super::Container;
use crate::{
    traits::{IntervalBounds, ValueBounds},
    types::MergeResults,
};

pub trait Merge<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    fn merge_unchecked(&self) -> MergeResults<T, I> {
        let mut base_interval = I::from(&self.records()[0]);

        let mut cluster_intervals = Vec::with_capacity(self.len());
        let mut cluster_ids = Vec::with_capacity(self.len());
        let mut current_id = 0;

        for interval in self.records().iter() {
            if base_interval.overlaps(interval) {
                let new_min = base_interval.start().min(interval.start());
                let new_max = base_interval.end().max(interval.end());
                base_interval.update_start(&new_min);
                base_interval.update_end(&new_max);
            } else {
                cluster_intervals.push(base_interval.to_owned());
                base_interval = I::from(interval);
                current_id += 1;
            }
            cluster_ids.push(current_id);
        }
        cluster_intervals.push(base_interval.to_owned());
        MergeResults::new(cluster_intervals, cluster_ids)
    }

    fn merge(&self) -> Option<MergeResults<T, I>> {
        if self.is_sorted() {
            Some(self.merge_unchecked())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod testing {
    use super::Merge;
    use crate::{types::{GenomicIntervalSet, IntervalSet}, traits::{Container, Coordinates}};

    #[test]
    fn test_merging_one_cluster() {
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.n_clusters(), 1);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        assert_eq!(merge_set.intervals()[0].start(), 10);
        assert_eq!(merge_set.intervals()[0].end(), 30);
    }

    #[test]
    fn test_merging_two_clusters() {
        let starts = vec![10, 15, 25, 35, 40];
        let ends = vec![30, 20, 30, 50, 45];
        let mut set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        set.sort();
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.n_clusters(), 2);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0, 1, 1]);
        assert_eq!(merge_set.intervals()[0].start(), 10);
        assert_eq!(merge_set.intervals()[0].end(), 30);
        assert_eq!(merge_set.intervals()[1].start(), 35);
        assert_eq!(merge_set.intervals()[1].end(), 50);
    }

    #[test]
    fn test_merging_one_cluster_unsort() {
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let merge_set = set.merge();
        assert!(merge_set.is_none());
    }

    #[test]
    fn test_merging_one_cluster_genomic() {
        let chrs = vec![1, 1, 1];
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let mut set = GenomicIntervalSet::from_endpoints_unchecked(&chrs, &starts, &ends);
        set.sort();
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.n_clusters(), 1);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        assert_eq!(merge_set.intervals()[0].start(), 10);
        assert_eq!(merge_set.intervals()[0].end(), 30);
    }

    #[test]
    fn test_merging_two_cluster_genomic() {
        let chrs = vec![1, 1, 2];
        let starts = vec![10, 15, 25];
        let ends = vec![30, 20, 30];
        let mut set = GenomicIntervalSet::from_endpoints_unchecked(&chrs, &starts, &ends);
        set.sort();
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.n_clusters(), 2);
        assert_eq!(merge_set.clusters(), &vec![0, 0, 1]);
        assert_eq!(merge_set.intervals()[0].start(), 10);
        assert_eq!(merge_set.intervals()[0].end(), 30);
    }
}
