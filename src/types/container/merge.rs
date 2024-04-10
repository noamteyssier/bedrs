// use super::Container;
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds, ValueBounds},
    IntervalContainer, Strand,
};

/// A trait to merge overlapping interval regions within a container
impl<I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn merge_pred(a: &I, b: &I) -> bool {
        a.overlaps(b) || a.borders(b)
    }

    fn stranded_merge_pred(a: &I, b: &I) -> bool {
        a.stranded_overlaps(b) || a.stranded_borders(b)
    }

    fn update_base_coordinates(base: &mut I, iv: &I) {
        let new_min = base.start().min(iv.start());
        let new_max = base.end().max(iv.end());
        base.update_endpoints(&new_min, &new_max);
        if base.strand() == iv.strand() {
            base.update_strand(iv.strand());
        } else {
            base.update_strand(None);
        }
    }

    fn add_interval(iv: &I, cluster_intervals: &mut Vec<I>) {
        cluster_intervals.push(iv.to_owned());
    }

    fn reset_base(base: &mut I, iv: &I) {
        base.update_all_from(iv);
        base.update_strand(iv.strand());
    }

    fn init_base(iv: &I) -> I {
        let mut base = I::empty();
        base.update_all_from(iv);
        base.update_strand(iv.strand());
        base
    }

    /// Finds the first stranded record in the set
    ///
    /// Initialize the relevant stranded base interval's coordinates
    fn init_stranded_base(&self) -> Option<(Option<I>, Option<I>)> {
        // find the first stranded record
        let siv = self
            .records()
            .iter()
            .find(|x| x.strand().is_some_and(|x| x != Strand::Unknown))?;

        // initialize the base interval
        let mut tmp_iv = I::empty();
        tmp_iv.update_all_from(siv);
        tmp_iv.update_strand(siv.strand());

        // return the relevant base interval and leave the other as None
        match siv.strand().unwrap() {
            Strand::Forward => Some((Some(tmp_iv), None)),
            Strand::Reverse => Some((None, Some(tmp_iv))),
            Strand::Unknown => {
                unreachable!("Should not be possible to reach this!");
            }
        }
    }

    /// Finds the first stranded record in the set
    ///
    /// Initialize the relevant stranded base interval's coordinates
    fn init_specific_strand(&self, strand: Strand) -> Option<I> {
        // find the first stranded record
        let siv = self
            .records()
            .iter()
            .find(|x| x.strand().is_some_and(|x| x == strand))?;

        // initialize the base interval
        let mut tmp_iv = I::empty();
        tmp_iv.update_all_from(siv);
        tmp_iv.update_strand(siv.strand());
        Some(tmp_iv)
    }

    #[must_use]
    pub fn merge_unchecked(&self) -> Self {
        let mut base = I::empty();
        Self::reset_base(&mut base, &self.records()[0]);

        let mut cluster_intervals = Vec::with_capacity(self.len());

        for iv in self.records() {
            if Self::merge_pred(&base, iv) {
                Self::update_base_coordinates(&mut base, iv);
            } else {
                Self::add_interval(&base, &mut cluster_intervals);
                Self::reset_base(&mut base, iv);
            }
        }
        Self::add_interval(&base, &mut cluster_intervals);
        IntervalContainer::from_sorted_unchecked(cluster_intervals)
    }

    /// Merges all intervals only from a specific strand
    #[must_use]
    pub fn merge_spec_strand_unchecked(&self, strand: Strand) -> Option<Self> {
        let mut base = self.init_specific_strand(strand)?;
        let mut cluster_intervals = Vec::with_capacity(self.len());
        for iv in self
            .records()
            .iter()
            .filter(|x| x.strand().is_some_and(|x| x == strand))
        {
            if Self::merge_pred(&base, iv) {
                Self::update_base_coordinates(&mut base, iv);
            } else {
                Self::add_interval(&base, &mut cluster_intervals);
                Self::reset_base(&mut base, iv);
            }
        }
        Self::add_interval(&base, &mut cluster_intervals);
        Some(IntervalContainer::from_sorted_unchecked(cluster_intervals))
    }

    // Keeps two Option<I> for potential base intervals
    //
    // For each new interval will check to see if it can possibly extend
    // either of the two options.
    //
    // Can return `None` in the case that no intervals are stranded
    #[must_use]
    pub fn merge_stranded_unchecked(&self) -> Option<Self> {
        let (mut fwd, mut rev) = self.init_stranded_base()?;
        let mut cluster_intervals = Vec::with_capacity(self.len());

        for iv in self.records() {
            match iv.strand() {
                // Skip all intervals that have unknown strand
                None | Some(Strand::Unknown) => continue,

                // Forward strand processing
                Some(Strand::Forward) => {
                    // There exists a cluster on the forward strand already
                    if let Some(ref mut base) = fwd {
                        // Extend the cluster if they meet overlap predicates
                        if Self::stranded_merge_pred(base, iv) {
                            Self::update_base_coordinates(base, iv);
                        // Write the previous cluster otherwise
                        } else {
                            // Write the complement strand cluster if it is behind
                            // to maintain sorted order.
                            if let Some(ref r) = rev {
                                if base.gt(r) {
                                    Self::add_interval(r, &mut cluster_intervals);
                                    rev = None;
                                }
                            }
                            Self::add_interval(base, &mut cluster_intervals);
                            Self::reset_base(base, iv);
                        }
                    } else {
                        fwd = Some(Self::init_base(iv));
                    }
                }

                // Reverse strand processing
                Some(Strand::Reverse) => {
                    // There exists a cluster on the reverse strand already
                    if let Some(ref mut base) = rev {
                        // Extend the cluster if they meet overlap predicates
                        if Self::stranded_merge_pred(base, iv) {
                            Self::update_base_coordinates(base, iv);
                        // Write the previous cluster otherwise
                        } else {
                            // Write the complement strand cluster if it is behind
                            // to maintain sorted order.
                            if let Some(ref f) = fwd {
                                if base.gt(f) {
                                    Self::add_interval(f, &mut cluster_intervals);
                                    fwd = None;
                                }
                            }
                            Self::add_interval(base, &mut cluster_intervals);
                            Self::reset_base(base, iv);
                        }
                    } else {
                        rev = Some(Self::init_base(iv));
                    }
                }
            }
        }
        // Write remaining clusters but maintain sorted order
        match (fwd, rev) {
            (Some(fwd), None) => Self::add_interval(&fwd, &mut cluster_intervals),
            (None, Some(rev)) => Self::add_interval(&rev, &mut cluster_intervals),
            (Some(fwd), Some(rev)) => {
                if fwd.lt(&rev) {
                    Self::add_interval(&fwd, &mut cluster_intervals);
                    Self::add_interval(&rev, &mut cluster_intervals);
                } else {
                    Self::add_interval(&rev, &mut cluster_intervals);
                    Self::add_interval(&fwd, &mut cluster_intervals);
                }
            }
            (None, None) => {
                unreachable!("This shouldn't ever be reached! Please submit an issue if it does")
            }
        }
        Some(IntervalContainer::from_sorted_unchecked(cluster_intervals))
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

    /// Merges overlapping intervals within a container
    ///
    /// ```text
    /// (a)    |---->
    /// (b)      |---->
    /// (c)        <----|
    /// (d)                  |---->
    /// (e)                    |---->
    /// ===============================
    /// (1)    |------>
    /// (2)        <----|
    /// (3)                  |------>
    /// ```
    ///
    /// Can return `None` in the case where there are no stranded intervals
    pub fn merge_stranded(&self) -> Result<Option<Self>, SetError> {
        if self.is_sorted() {
            Ok(self.merge_stranded_unchecked())
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Merges overlapping intervals within a container
    /// if they are on a specific strand only.
    ///
    /// Ignores all other intervals
    ///
    /// ```text
    /// (a)    |---->
    /// (b)      |---->
    /// (c)        <----|
    /// (d)                  |---->
    /// (e)                    |---->
    /// ===============================
    /// (1)    |------>
    /// (3)                  |------>
    /// ```
    ///
    /// Can return `None` in the case where there are no stranded intervals
    pub fn merge_specific_strand(&self, strand: Strand) -> Result<Option<Self>, SetError> {
        if self.is_sorted() {
            match strand {
                Strand::Unknown => Err(SetError::CannotAcceptUnknownStrand),
                s => Ok(self.merge_spec_strand_unchecked(s)),
            }
        } else {
            Err(SetError::UnsortedSet)
        }
    }
}

#[cfg(test)]
mod testing {
    use std::fmt::Debug;

    use anyhow::Result;

    use crate::{
        traits::{ChromBounds, Coordinates, IntervalBounds, ValueBounds},
        BaseInterval, Bed3, IntervalContainer, Strand, StrandedBed3,
    };

    fn validate_set<C, I, T>(set: &IntervalContainer<I, C, T>, expected: &[I])
    where
        I: IntervalBounds<C, T> + Debug,
        C: ChromBounds,
        T: ValueBounds,
    {
        println!("\nExpected:");
        for iv in expected {
            println!("{iv:?}");
        }
        println!("\nObserved:");
        for iv in set.records() {
            println!("{iv:?}");
        }
        assert_eq!(set.len(), expected.len());
        for (c1, c2) in set.records().iter().zip(expected) {
            assert!(c1.eq(c2));
        }
    }

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

    #[test]
    fn merge_intervals_stranded() -> Result<()> {
        let records = vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 15, 25, Strand::Forward),
            StrandedBed3::new(1, 20, 30, Strand::Reverse), // overlapping but wrong strand
            StrandedBed3::new(1, 40, 50, Strand::Reverse),
            StrandedBed3::new(1, 45, 55, Strand::Reverse),
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?.unwrap();
        let expected = vec![
            StrandedBed3::new(1, 10, 25, Strand::Forward),
            StrandedBed3::new(1, 20, 30, Strand::Reverse),
            StrandedBed3::new(1, 40, 55, Strand::Reverse),
        ];

        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_interleaved() -> Result<()> {
        let records = vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 15, 25, Strand::Forward),
            StrandedBed3::new(1, 20, 30, Strand::Reverse), // overlapping but wrong strand
            StrandedBed3::new(1, 22, 32, Strand::Forward), // Overlaps the n-2 interval
            StrandedBed3::new(1, 25, 35, Strand::Reverse), // Overlaps the n-2 interval
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?.unwrap();
        let expected = vec![
            StrandedBed3::new(1, 10, 32, Strand::Forward),
            StrandedBed3::new(1, 20, 35, Strand::Reverse),
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_interleaved_capped_fwd() -> Result<()> {
        let records = vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 15, 25, Strand::Forward),
            StrandedBed3::new(1, 20, 30, Strand::Reverse), // overlapping but wrong strand
            StrandedBed3::new(1, 22, 32, Strand::Forward), // Overlaps the n-2 interval
            StrandedBed3::new(1, 25, 35, Strand::Reverse), // Overlaps the n-2 interval
            StrandedBed3::new(2, 10, 20, Strand::Forward), // Doesn't Overlap any previous
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?.unwrap();
        let expected = vec![
            StrandedBed3::new(1, 10, 32, Strand::Forward),
            StrandedBed3::new(1, 20, 35, Strand::Reverse),
            StrandedBed3::new(2, 10, 20, Strand::Forward),
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_interleaved_capped_rev() -> Result<()> {
        let records = vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 15, 25, Strand::Forward),
            StrandedBed3::new(1, 20, 30, Strand::Reverse), // overlapping but wrong strand
            StrandedBed3::new(1, 22, 32, Strand::Forward), // Overlaps the n-2 interval
            StrandedBed3::new(1, 25, 35, Strand::Reverse), // Overlaps the n-2 interval
            StrandedBed3::new(2, 10, 20, Strand::Reverse), // Doesn't Overlap any previous
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?.unwrap();
        let expected = vec![
            StrandedBed3::new(1, 10, 32, Strand::Forward),
            StrandedBed3::new(1, 20, 35, Strand::Reverse),
            StrandedBed3::new(2, 10, 20, Strand::Reverse),
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_all_missing_strand_info() -> Result<()> {
        let records = vec![
            StrandedBed3::new(1, 10, 20, Strand::Unknown),
            StrandedBed3::new(1, 15, 25, Strand::Unknown),
            StrandedBed3::new(1, 20, 30, Strand::Unknown),
            StrandedBed3::new(1, 22, 32, Strand::Unknown),
            StrandedBed3::new(1, 25, 35, Strand::Unknown),
            StrandedBed3::new(2, 10, 20, Strand::Unknown),
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        assert!(merge_set.is_none());
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_all_missing_strand_info_minimal() -> Result<()> {
        let records = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 15, 25),
            Bed3::new(1, 20, 30),
            Bed3::new(1, 22, 32),
            Bed3::new(1, 25, 35),
            Bed3::new(2, 10, 20),
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        assert!(merge_set.is_none());
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_skip_missing() -> Result<()> {
        let records = vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 15, 25, Strand::Forward),
            StrandedBed3::new(1, 20, 30, Strand::Unknown), // Missing info
            StrandedBed3::new(1, 21, 31, Strand::Unknown), // Missing info
            StrandedBed3::new(1, 22, 32, Strand::Forward), // Overlaps the n-2 interval
            StrandedBed3::new(1, 25, 35, Strand::Reverse),
            StrandedBed3::new(2, 10, 20, Strand::Reverse), // Doesn't Overlap any previous
            StrandedBed3::new(2, 21, 31, Strand::Unknown), // Missing info
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?.unwrap();
        let expected = vec![
            StrandedBed3::new(1, 10, 32, Strand::Forward),
            StrandedBed3::new(1, 25, 35, Strand::Reverse),
            StrandedBed3::new(2, 10, 20, Strand::Reverse),
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }
}
