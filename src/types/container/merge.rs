// use super::Container;
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    IntervalContainer,
};

/// A trait to merge overlapping interval regions within a container
impl<I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    #[must_use]
    pub fn merge_unchecked(&self) -> Self {
        let mut base_interval = I::from(&self.records()[0]);

        let mut cluster_intervals = Vec::with_capacity(self.len());
        let mut cluster_ids = Vec::with_capacity(self.len());
        let mut current_id = 0;

        for interval in self.records() {
            if base_interval.overlaps(interval) || base_interval.borders(interval) {
                let new_min = base_interval.start().min(interval.start());
                let new_max = base_interval.end().max(interval.end());
                base_interval.update_endpoints(&new_min, &new_max);
                if base_interval.strand() == interval.strand() {
                    base_interval.update_strand(interval.strand());
                } else {
                    base_interval.update_strand(None);
                }
            } else {
                cluster_intervals.push(base_interval.to_owned());
                base_interval.update_all_from(interval);
                current_id += 1;
            }
            cluster_ids.push(current_id);
        }
        cluster_intervals.push(base_interval.to_owned());
        IntervalContainer::from_sorted_unchecked(cluster_intervals)
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
    pub fn merge(&self) -> Result<Self, SetError> {
        if self.is_sorted() {
            Ok(self.merge_unchecked())
        } else {
            Err(SetError::UnsortedSet)
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{traits::Coordinates, BaseInterval, Bed3, IntervalContainer};

    #[test]
    fn test_merging_one_cluster() {
        let records = vec![
            BaseInterval::new(10, 30),
            BaseInterval::new(15, 20),
            BaseInterval::new(25, 30),
        ];
        let set = IntervalContainer::from_unsorted(records);
        let merge_set = set.merge().unwrap();
        let iv = merge_set.records()[0];
        assert_eq!(merge_set.len(), 1);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 30);
        // assert_eq!(merge_set.n_clusters(), 1);
        // assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        // assert_eq!(merge_set.intervals()[0].start(), 10);
        // assert_eq!(merge_set.intervals()[0].end(), 30);
    }

    #[test]
    fn test_merging_two_clusters() {
        let records = vec![
            BaseInterval::new(10, 30),
            BaseInterval::new(15, 20),
            BaseInterval::new(25, 30),
            BaseInterval::new(35, 50),
            BaseInterval::new(40, 45),
        ];
        let set = IntervalContainer::from_unsorted(records);
        let merge_set = set.merge().unwrap();
        let iv1 = merge_set.records()[0];
        let iv2 = merge_set.records()[1];

        assert_eq!(merge_set.len(), 2);
        assert_eq!(iv1.start(), 10);
        assert_eq!(iv1.end(), 30);
        assert_eq!(iv2.start(), 35);
        assert_eq!(iv2.end(), 50);

        // assert_eq!(merge_set.n_clusters(), 2);
        // assert_eq!(merge_set.clusters(), &vec![0, 0, 0, 1, 1]);
        // assert_eq!(merge_set.intervals()[0].start(), 10);
        // assert_eq!(merge_set.intervals()[0].end(), 30);
        // assert_eq!(merge_set.intervals()[1].start(), 35);
        // assert_eq!(merge_set.intervals()[1].end(), 50);
    }

    #[test]
    fn test_merging_one_cluster_unsort() {
        let records = vec![
            BaseInterval::new(10, 30),
            BaseInterval::new(15, 20),
            BaseInterval::new(25, 30),
        ];
        let set = IntervalContainer::from_iter(records);
        let merge_set = set.merge();
        assert!(merge_set.is_err());
    }

    #[test]
    fn test_merging_one_cluster_genomic() {
        let records = vec![
            Bed3::new(1, 10, 30),
            Bed3::new(1, 15, 20),
            Bed3::new(1, 25, 30),
        ];
        let set = IntervalContainer::from_unsorted(records);
        let merge_set = set.merge().unwrap();
        let iv1 = merge_set.records()[0];

        assert_eq!(merge_set.len(), 1);
        assert_eq!(iv1.start(), 10);
        assert_eq!(iv1.end(), 30);

        // assert_eq!(merge_set.n_clusters(), 1);
        // assert_eq!(merge_set.clusters(), &vec![0, 0, 0]);
        // assert_eq!(merge_set.intervals()[0].start(), 10);
        // assert_eq!(merge_set.intervals()[0].end(), 30);
    }

    #[test]
    fn test_merging_two_cluster_genomic() {
        let records = vec![
            Bed3::new(1, 10, 30),
            Bed3::new(1, 15, 20),
            Bed3::new(2, 25, 30),
        ];
        let set = IntervalContainer::from_unsorted(records);
        let merge_set = set.merge().unwrap();
        let iv1 = merge_set.records()[0];
        let iv2 = merge_set.records()[1];

        assert_eq!(merge_set.len(), 2);
        assert_eq!(iv1.start(), 10);
        assert_eq!(iv1.end(), 30);
        assert_eq!(iv2.start(), 25);
        assert_eq!(iv2.end(), 30);

        // assert_eq!(merge_set.n_clusters(), 2);
        // assert_eq!(merge_set.clusters(), &vec![0, 0, 1]);
        // assert_eq!(merge_set.intervals()[0].start(), 10);
        // assert_eq!(merge_set.intervals()[0].end(), 30);
    }

    #[test]
    fn merging_base_borders() {
        let records = vec![
            BaseInterval::new(10, 20),
            BaseInterval::new(20, 30),
            BaseInterval::new(30, 40),
        ];
        let set = IntervalContainer::from_sorted_unchecked(records);
        let merge_set = set.merge_unchecked();
        let iv = merge_set.records()[0];

        assert_eq!(merge_set.len(), 1);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 40);

        // assert_eq!(merge_set.n_clusters(), 1);
        // assert_eq!(merge_set.intervals()[0].start(), 10);
        // assert_eq!(merge_set.intervals()[0].end(), 40);
    }

    #[test]
    fn merge_container_methods() {
        let records = vec![
            BaseInterval::new(10, 20),
            BaseInterval::new(20, 30),
            BaseInterval::new(30, 40),
        ];
        let set = IntervalContainer::from_sorted_unchecked(records);
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

    // #[test]
    // #[should_panic]
    // fn merge_container_new() {
    //     let records = vec![
    //         BaseInterval::new(10, 20),
    //         BaseInterval::new(20, 30),
    //         BaseInterval::new(30, 40),
    //     ];
    //     let _merge_set: MergeResults<usize, usize, BaseInterval<usize>> = Container::new(records);
    // }
}
