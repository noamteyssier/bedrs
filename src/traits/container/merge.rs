use super::Container;
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    types::MergeResults,
};

/// A trait to merge overlapping interval regions within a container
pub trait Merge<C, T, I>: Container<C, T, I>
where
    C: ChromBounds,
    T: ValueBounds,
    I: IntervalBounds<C, T>,
{
    fn merge_unchecked(&self) -> MergeResults<C, T, I> {
        let mut base_interval = I::from(&self.records()[0]);

        let mut cluster_intervals = Vec::with_capacity(self.len());
        let mut cluster_ids = Vec::with_capacity(self.len());
        let mut current_id = 0;

        for interval in self.records().iter() {
            if base_interval.overlaps(interval) || base_interval.borders(interval) {
                let new_min = base_interval.start().min(interval.start());
                let new_max = base_interval.end().max(interval.end());
                base_interval.update_endpoints(&new_min, &new_max);
            } else {
                cluster_intervals.push(base_interval.to_owned());
                base_interval.update_all_from(interval);
                current_id += 1;
            }
            cluster_ids.push(current_id);
        }
        cluster_intervals.push(base_interval.to_owned());
        MergeResults::new(cluster_intervals, cluster_ids)
    }

    /// Merges overlapping intervals within a container
    ///
    /// ```text
    /// (a)    i----j
    /// (b)      k----l
    /// (c)        m----n
    /// (d)                  o----p
    /// (e)                    q----r
    /// ===============================
    /// (1)    i--------n
    /// (2)                  o------r
    /// ```
    fn merge(&self) -> Result<MergeResults<C, T, I>, SetError> {
        if self.is_sorted() {
            Ok(self.merge_unchecked())
        } else {
            Err(SetError::UnsortedSet)
        }
    }
}

#[cfg(test)]
mod testing {
    use super::Merge;
    use crate::{
        traits::{Container, Coordinates},
        types::{GenomicIntervalSet, IntervalSet, MergeResults},
        Interval,
    };

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
        assert!(merge_set.is_err());
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

    #[test]
    fn merging_base_borders() {
        let records = vec![
            Interval::new(10, 20),
            Interval::new(20, 30),
            Interval::new(30, 40),
        ];
        let set = IntervalSet::from_sorted_unchecked(records);
        let merge_set = set.merge_unchecked();
        assert_eq!(merge_set.n_clusters(), 1);
        assert_eq!(merge_set.intervals()[0].start(), 10);
        assert_eq!(merge_set.intervals()[0].end(), 40);
    }

    #[test]
    fn merge_container_methods() {
        let records = vec![
            Interval::new(10, 20),
            Interval::new(20, 30),
            Interval::new(30, 40),
        ];
        let set = IntervalSet::from_sorted_unchecked(records);
        let mut merge_set = set.merge_unchecked();
        let mut_records = merge_set.records_mut();
        assert_eq!(mut_records.len(), 1);
        assert_eq!(mut_records[0].start(), 10);
        assert_eq!(mut_records[0].end(), 40);
        assert!(merge_set.is_sorted());
        *merge_set.sorted_mut() = false;
        assert!(!merge_set.is_sorted());
        assert_eq!(merge_set.max_len(), Some(30));
        assert_eq!(merge_set.max_len_mut().unwrap(), 30);
    }

    #[test]
    #[should_panic]
    fn merge_container_new() {
        let records = vec![
            Interval::new(10, 20),
            Interval::new(20, 30),
            Interval::new(30, 40),
        ];
        let _merge_set: MergeResults<usize, usize, Interval<usize>> = Container::new(records);
    }
}
