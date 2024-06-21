// use super::Container;
use super::IntervalTree;
use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds},
    IntervalContainer, Strand,
};

#[derive(Debug, Clone, Copy)]
pub enum MergeCondition {
    Standard,
    Stranded,
    OnlyStrand(Strand),
}

/// A trait to merge overlapping interval regions within a container
impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn merge(&self) -> Result<Self, SetError> {
        self.impl_merge(MergeCondition::Standard)
    }

    pub fn merge_stranded(&self) -> Result<Self, SetError> {
        self.impl_merge(MergeCondition::Stranded)
    }

    pub fn merge_specific_strand(&self, strand: Strand) -> Result<Self, SetError> {
        self.impl_merge(MergeCondition::OnlyStrand(strand))
    }

    pub fn merge_method(&self, method: MergeCondition) -> Result<Self, SetError> {
        match method {
            MergeCondition::Standard => self.merge(),
            MergeCondition::Stranded => self.merge_stranded(),
            MergeCondition::OnlyStrand(strand) => self.merge_specific_strand(strand),
        }
    }

    fn impl_merge(&self, method: MergeCondition) -> Result<Self, SetError> {
        if self.is_sorted() {
            let mut tree = IntervalTree::new();
            for name in self.subtree_names() {
                let subtree = self.subtree(name).unwrap();
                let merged_subtree = match method {
                    MergeCondition::Standard => Some(subtree.merge()?),
                    MergeCondition::Stranded => subtree.merge_stranded()?,
                    MergeCondition::OnlyStrand(strand) => subtree.merge_specific_strand(strand)?,
                };
                if let Some(merged_subtree) = merged_subtree {
                    tree.insert_subtree(name.clone(), merged_subtree);
                }
            }
            // tree can only be empty if either the original set was empty or all intervals were removed
            // during stranded method filtering
            if tree.is_empty() {
                if self.is_empty() {
                    Ok(Self::empty())
                } else {
                    Err(SetError::NoStrandedIntervals)
                }
            } else {
                Ok(Self::from(tree))
            }
        } else {
            Err(SetError::UnsortedSet)
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        bed3,
        traits::{ChromBounds, IntervalBounds, SetError},
        BaseInterval, Coordinates, IntervalContainer, Strand,
    };
    use anyhow::Result;
    use std::fmt::Debug;

    fn validate_set<C, I>(set: &IntervalContainer<I, C>, expected: &[I])
    where
        I: IntervalBounds<C> + Debug,
        C: ChromBounds,
    {
        println!("\nExpected:");
        for iv in expected {
            println!("{iv:?}");
        }
        println!("\nObserved:");
        for iv in set.iter() {
            println!("{iv:?}");
        }
        assert_eq!(set.len(), expected.len());
        for (c1, c2) in set.iter().zip(expected) {
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
        let iv = merge_set.iter().next().unwrap();
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
        assert_eq!(merge_set.len(), 2);

        let records = Vec::from(merge_set);
        println!("{:?}", records);
        let iv1 = records[0];
        let iv2 = records[1];

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
        let records = vec![bed3![1, 10, 30], bed3![1, 15, 20], bed3![1, 25, 30]];
        let set = IntervalContainer::from_unsorted(records);
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.len(), 1);

        let iv1 = merge_set.iter().next().unwrap();
        assert_eq!(iv1.start(), 10);
        assert_eq!(iv1.end(), 30);
    }

    #[test]
    fn test_merging_two_cluster_genomic() {
        let records = vec![bed3![1, 10, 30], bed3![1, 15, 20], bed3![2, 25, 30]];
        let set = IntervalContainer::from_unsorted(records);
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.len(), 2);

        let records = Vec::from(merge_set);
        let iv1 = records[0];
        let iv2 = records[1];
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
        let merge_set = set.merge().unwrap();
        assert_eq!(merge_set.len(), 1);

        let records = merge_set.to_vec();
        let iv = records[0];
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 40);
    }

    #[test]
    fn merge_intervals_stranded() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse], // overlapping but wrong strand
            bed3![1, 40, 50, Strand::Reverse],
            bed3![1, 45, 55, Strand::Reverse],
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        let expected = vec![
            bed3![1, 10, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse],
            bed3![1, 40, 55, Strand::Reverse],
        ];

        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_interleaved() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse], // overlapping but wrong strand
            bed3![1, 22, 32, Strand::Forward], // Overlaps the n-2 interval
            bed3![1, 25, 35, Strand::Reverse], // Overlaps the n-2 interval
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        let expected = vec![
            bed3![1, 10, 32, Strand::Forward],
            bed3![1, 20, 35, Strand::Reverse],
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_interleaved_capped_fwd() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse], // overlapping but wrong strand
            bed3![1, 22, 32, Strand::Forward], // Overlaps the n-2 interval
            bed3![1, 25, 35, Strand::Reverse], // Overlaps the n-2 interval
            bed3![2, 10, 20, Strand::Forward], // Doesn't Overlap any previous
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        let expected = vec![
            bed3![1, 10, 32, Strand::Forward],
            bed3![1, 20, 35, Strand::Reverse],
            bed3![2, 10, 20, Strand::Forward],
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_interleaved_capped_rev() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse], // overlapping but wrong strand
            bed3![1, 22, 32, Strand::Forward], // Overlaps the n-2 interval
            bed3![1, 25, 35, Strand::Reverse], // Overlaps the n-2 interval
            bed3![2, 10, 20, Strand::Reverse], // Doesn't Overlap any previous
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        let expected = vec![
            bed3![1, 10, 32, Strand::Forward],
            bed3![1, 20, 35, Strand::Reverse],
            bed3![2, 10, 20, Strand::Reverse],
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_all_missing_strand_info() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20, Strand::Unknown],
            bed3![1, 15, 25, Strand::Unknown],
            bed3![1, 20, 30, Strand::Unknown],
            bed3![1, 22, 32, Strand::Unknown],
            bed3![1, 25, 35, Strand::Unknown],
            bed3![2, 10, 20, Strand::Unknown],
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded();
        assert!(matches!(merge_set, Err(SetError::NoStrandedIntervals)));
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_all_missing_strand_info_minimal() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20],
            bed3![1, 15, 25],
            bed3![1, 20, 30],
            bed3![1, 22, 32],
            bed3![1, 25, 35],
            bed3![2, 10, 20],
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded();
        assert!(matches!(merge_set, Err(SetError::NoStrandedIntervals)));
        Ok(())
    }

    #[test]
    fn merge_intervals_stranded_skip_missing() -> Result<()> {
        let records = vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Unknown], // Missing info
            bed3![1, 21, 31, Strand::Unknown], // Missing info
            bed3![1, 22, 32, Strand::Forward], // Overlaps the n-2 interval
            bed3![1, 25, 35, Strand::Reverse],
            bed3![2, 10, 20, Strand::Reverse], // Doesn't Overlap any previous
            bed3![2, 21, 31, Strand::Unknown], // Missing info
        ];
        let set = IntervalContainer::from_sorted(records)?;
        let merge_set = set.merge_stranded()?;
        let expected = vec![
            bed3![1, 10, 32, Strand::Forward],
            bed3![1, 25, 35, Strand::Reverse],
            bed3![2, 10, 20, Strand::Reverse],
        ];
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_fwd() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse],
            bed3![1, 22, 32, Strand::Forward],
            bed3![1, 25, 35, Strand::Reverse],
            bed3![2, 10, 20, Strand::Forward],
        ])?;
        let expected = vec![
            bed3![1, 10, 32, Strand::Forward],
            bed3![2, 10, 20, Strand::Forward],
        ];
        let merge_set = set.merge_specific_strand(Strand::Forward)?;
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_rev() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20, Strand::Forward],
            bed3![1, 15, 25, Strand::Forward],
            bed3![1, 20, 30, Strand::Reverse],
            bed3![1, 22, 32, Strand::Forward],
            bed3![1, 25, 35, Strand::Reverse],
            bed3![2, 10, 20, Strand::Forward],
        ])?;
        let expected = vec![bed3![1, 20, 35, Strand::Reverse]];
        let merge_set = set.merge_specific_strand(Strand::Reverse)?;
        validate_set(&merge_set, &expected);
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_unknown() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20, Strand::Unknown],
            bed3![1, 15, 25, Strand::Unknown],
            bed3![1, 20, 30, Strand::Unknown],
            bed3![1, 22, 32, Strand::Unknown],
            bed3![1, 25, 35, Strand::Unknown],
            bed3![2, 10, 20, Strand::Unknown],
        ])?;
        let merge_set = set.merge_specific_strand(Strand::Unknown);
        assert!(merge_set.is_err());
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_unknown_strand_fwd() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20, Strand::Unknown],
            bed3![1, 15, 25, Strand::Unknown],
            bed3![1, 20, 30, Strand::Unknown],
            bed3![1, 22, 32, Strand::Unknown],
            bed3![1, 25, 35, Strand::Unknown],
            bed3![2, 10, 20, Strand::Unknown],
        ])?;
        let merge_set = set.merge_specific_strand(Strand::Forward);
        assert!(matches!(merge_set, Err(SetError::NoStrandedIntervals)));
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_unknown_strand_rev() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20, Strand::Unknown],
            bed3![1, 15, 25, Strand::Unknown],
            bed3![1, 20, 30, Strand::Unknown],
            bed3![1, 22, 32, Strand::Unknown],
            bed3![1, 25, 35, Strand::Unknown],
            bed3![2, 10, 20, Strand::Unknown],
        ])?;
        let merge_set = set.merge_specific_strand(Strand::Reverse);
        assert!(matches!(merge_set, Err(SetError::NoStrandedIntervals)));
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_missing_strand_fwd() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20],
            bed3![1, 15, 25],
            bed3![1, 20, 30],
            bed3![1, 22, 32],
            bed3![1, 25, 35],
            bed3![2, 10, 20],
        ])?;
        let merge_set = set.merge_specific_strand(Strand::Forward);
        assert!(matches!(merge_set, Err(SetError::NoStrandedIntervals)));
        Ok(())
    }

    #[test]
    fn merge_intervals_specific_strand_missing_strand_rev() -> Result<()> {
        let set = IntervalContainer::from_sorted(vec![
            bed3![1, 10, 20],
            bed3![1, 15, 25],
            bed3![1, 20, 30],
            bed3![1, 22, 32],
            bed3![1, 25, 35],
            bed3![2, 10, 20],
        ])?;
        let merge_set = set.merge_specific_strand(Strand::Reverse);
        assert!(matches!(merge_set, Err(SetError::NoStrandedIntervals)));
        Ok(())
    }
}
