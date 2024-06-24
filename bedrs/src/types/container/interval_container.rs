use super::{subtree::Subtree, tree::IntervalTree};
use crate::{
    traits::{ChromBounds, IntervalBounds, SetError},
    IntervalIterOwned, IntervalIterRef,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    data: IntervalTree<I, C>,
}
impl<I, C> FromIterator<I> for IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn from_iter<It: IntoIterator<Item = I>>(iter: It) -> Self {
        Self {
            data: IntervalTree::from_iter(iter),
        }
    }
}

impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    #[must_use]
    pub fn new(records: Vec<I>) -> Self {
        Self::from_iter(records)
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn subtree(&self, name: &C) -> Option<&Subtree<I, C>> {
        self.data.subtree(name)
    }
    pub fn subtree_mut(&mut self, name: &C) -> Option<&mut Subtree<I, C>> {
        self.data.subtree_mut(name)
    }
    pub fn subtree_owned(&mut self, name: &C) -> Option<Subtree<I, C>> {
        self.data.subtree_owned(name)
    }
    pub fn subtrees(&self) -> impl Iterator<Item = &Subtree<I, C>> {
        self.data.values()
    }
    pub fn subtrees_mut(&mut self) -> impl Iterator<Item = &mut Subtree<I, C>> {
        self.data.values_mut()
    }
    #[must_use]
    pub fn num_subtrees(&self) -> usize {
        self.data.num_subtrees()
    }
    #[must_use]
    pub fn subtree_names(&self) -> Vec<&C> {
        self.data.subtree_names()
    }
    #[must_use]
    pub fn subtree_names_sorted(&self) -> Vec<&C> {
        self.data.subtree_names_sorted()
    }
    #[must_use]
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[must_use]
    pub fn is_sorted(&self) -> bool {
        self.data.is_sorted()
    }
    pub fn set_unsorted(&mut self) {
        self.data.set_unsorted();
    }
    #[allow(clippy::iter_without_into_iter)]
    #[must_use]
    pub fn iter(&self) -> IntervalIterRef<I, C> {
        IntervalIterRef::new(self)
    }
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn into_iter(self) -> IntervalIterOwned<I, C> {
        IntervalIterOwned::new(self)
    }

    /// Sets the internal state to sorted
    ///
    /// >> This would likely not be used directly by the user.
    /// >> If you are creating an interval set from presorted
    /// >> intervals use the `from_sorted()` method instead of
    /// >> the `new()` method.
    pub fn set_sorted(&mut self) {
        self.data.set_sorted();
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    pub fn sort(&mut self) {
        self.data.sort();
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    /// but parallelizes the sorting.
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.data.par_sort();
    }

    /// Inserts a new interval into the container
    ///
    /// This will not sort the container after insertion.
    /// If you need to sort the container after insertion
    /// use the `insert_sorted()` method instead.
    ///
    /// This is more efficient if you are inserting many
    /// intervals at once.
    pub fn insert(&mut self, interval: I) {
        self.data.insert_interval(interval);
    }

    /// Inserts a new interval into the container and sorts the container
    /// after insertion.
    ///
    /// This is less efficient than the `insert()` method if you are
    /// inserting many intervals at once.
    pub fn insert_sorted(&mut self, interval: I) {
        self.insert(interval);
        self.sort();
    }

    /// Creates a new container from presorted intervals
    ///
    /// First this validates that the intervals are truly presorted.
    pub fn from_sorted(records: Vec<I>) -> Result<Self, SetError> {
        if Self::valid_interval_sorting(&records) {
            Ok(Self::from_sorted_unchecked(records))
        } else {
            Err(SetError::UnsortedIntervals)
        }
    }

    /// Creates a new container from presorted intervals without
    /// validating if the intervals are truly presorted.
    #[must_use]
    pub fn from_sorted_unchecked(records: Vec<I>) -> Self {
        let mut set = Self::new(records);
        set.set_sorted();
        set
    }

    /// Creates a new *sorted* container from unsorted intervals
    #[must_use]
    pub fn from_unsorted(records: Vec<I>) -> Self {
        let mut set = Self::new(records);
        set.sort();
        set
    }

    /// Validates that a set of intervals are sorted
    #[must_use]
    pub fn valid_interval_sorting(records: &[I]) -> bool {
        records
            .iter()
            .enumerate()
            .skip(1)
            .map(|(idx, rec)| (rec, &records[idx - 1]))
            .all(|(a, b)| a.coord_cmp(b).is_ge())
    }

    /// Applies a mutable function to each interval in the container
    pub fn apply_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut I),
    {
        self.data.apply_mut(f);
    }

    pub fn span(&self, name: &C) -> Option<Result<I, SetError>> {
        self.data.span(name)
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<I> {
        Vec::from(self)
    }
}

impl<I, C> From<IntervalContainer<I, C>> for Vec<I>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn from(set: IntervalContainer<I, C>) -> Self {
        set.into_iter().collect()
    }
}

impl<I, C> From<&IntervalContainer<I, C>> for Vec<I>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn from(set: &IntervalContainer<I, C>) -> Self {
        set.iter().cloned().collect()
    }
}

impl<I, C> From<IntervalTree<I, C>> for IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn from(data: IntervalTree<I, C>) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::{bed3, BaseInterval, Bed3, Coordinates, Strand};

    // --------------------- //
    // Base BaseInterval Testing //
    // --------------------- //

    #[test]
    fn build_interval_container() {
        let records = vec![
            BaseInterval::new(1, 10),
            BaseInterval::new(2, 20),
            BaseInterval::new(3, 30),
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_base_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![BaseInterval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_base_interval_set_init_from_endpoints() {
        let n_intervals = 10;
        let records = vec![BaseInterval::new(10, 100); n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_base_from_iterator() {
        let n_intervals = 10;
        let records = vec![BaseInterval::new(10, 100); n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }
    //
    // #[test]
    // #[cfg(feature = "serde")]
    // fn test_base_serialization() {
    //     let n_intervals = 10;
    //     let records = vec![BaseInterval::new(10, 100); n_intervals];
    //     let set = IntervalContainer::new(records);
    //     let serialized = serialize(&set).unwrap();
    //     let deserialized: IntervalContainer<BaseInterval, i32> = deserialize(&serialized).unwrap();
    //     for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
    //         assert!(iv1.eq(iv2));
    //     }
    // }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_base_par_sort() {
        let n_intervals = 10;
        let records = vec![BaseInterval::new(10, 100); n_intervals];
        let mut set = IntervalContainer::new(records);
        set.par_sort();
        assert!(set.is_sorted());
    }

    // ------------------------ //
    // Genomic Interval Testing //
    // ------------------------ //

    #[test]
    fn build_genomic_interval_container() {
        let records = vec![bed3![1, 1, 10], bed3![1, 2, 20], bed3![1, 3, 30]];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_genomic_interval_set_init_from_records() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100]; n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_from_iterator() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100]; n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_genomic_from_empty_iterator() {
        let records: Vec<Bed3<i32>> = vec![];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), 0);
        // assert!(set.max_len().is_none());
        // assert!(set.span().is_err());
    }

    // #[test]
    // fn test_genomic_span() {
    //     let intervals = vec![bed3![1, 10, 100], bed3![1, 20, 200]];
    //     let set = IntervalContainer::from_sorted(intervals).unwrap();
    //     assert!(set.span().unwrap().eq(&bed3![1, 10, 200]));
    // }

    // #[test]
    // fn test_genomic_span_errors() {
    //     let intervals = vec![bed3![1, 10, 100], bed3![2, 20, 200]];
    //     let mut set = IntervalContainer::from_iter(intervals);
    //     match set.span() {
    //         Err(e) => assert_eq!(e.to_string(), "Cannot get span of unsorted interval set"),
    //         _ => panic!("Expected error"),
    //     };
    //     set.sort();
    //     match set.span() {
    //         Err(e) => assert_eq!(
    //             e.to_string(),
    //             "Cannot get span of interval set spanning multiple chromosomes"
    //         ),
    //         _ => panic!("Expected error"),
    //     };
    // }
    //
    // #[test]
    // #[cfg(feature = "serde")]
    // fn test_genomic_serialization() {
    //     use crate::bed3;
    //
    //     let n_intervals = 10;
    //     let records = vec![bed3![1, 10, 100]; n_intervals];
    //     let set = IntervalContainer::new(records);
    //     let serialized = serialize(&set).unwrap();
    //     let deserialized: IntervalContainer<Bed3<i32>, i32> = deserialize(&serialized).unwrap();
    //
    //     for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
    //         assert!(iv1.eq(iv2));
    //     }
    // }
    //
    // #[test]
    // #[cfg(feature = "rayon")]
    // fn test_genomic_par_sort() {
    //     use crate::bed3;
    //
    //     let n_intervals = 10;
    //     let records = vec![bed3![1, 10, 100]; n_intervals];
    //     let mut set = IntervalContainer::new(records.clone());
    //     set.par_sort();
    //     for (iv1, iv2) in set.records().iter().zip(records.iter()) {
    //         assert!(iv1.eq(iv2));
    //     }
    // }

    // ------------------------- //
    // Stranded Interval Testing //
    // ------------------------- //

    #[test]
    fn build_stranded_genomic_interval_container() {
        let records = vec![
            bed3![1, 1, 10, Strand::Forward],
            bed3![1, 2, 20, Strand::Forward],
            bed3![1, 3, 30, Strand::Forward],
        ];
        let container = IntervalContainer::from_unsorted(records);
        assert_eq!(container.len(), 3);
    }

    #[test]
    fn test_stranded_genomic_init_from_records() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100, Strand::Reverse]; n_intervals];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_stranded_genomic_from_iterator() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100, Strand::Reverse]; n_intervals];
        let set = IntervalContainer::from_iter(records);
        assert_eq!(set.len(), n_intervals);
    }

    #[test]
    fn test_stranded_genomic_set_sorted() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100, Strand::Reverse]; n_intervals];
        let mut set = IntervalContainer::new(records);
        assert!(!set.is_sorted());
        set.set_sorted();
        assert!(set.is_sorted());
    }

    #[test]
    fn test_stranded_genomic_set_records_mut() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100, Strand::Forward]; n_intervals];
        let mut set = IntervalContainer::new(records);

        set.iter().for_each(|r| {
            assert_eq!(r.strand().unwrap(), Strand::Forward);
        });

        set.apply_mut(|r| {
            r.update_strand(Some(Strand::Reverse));
        });

        set.iter().for_each(|r| {
            assert_eq!(r.strand().unwrap(), Strand::Reverse);
        });
    }

    // #[test]
    // fn test_span_empty() {
    //     let set: IntervalContainer<StrandedBed3<u32>, u32> = IntervalContainer::new(vec![]);
    //     let span = set.span();
    //     assert!(span.is_err());
    // }
    //
    // #[test]
    // fn test_span_unsorted() {
    //     let n_intervals = 10;
    //     let records = vec![bed3![1, 10, 100, Strand::Forward]; n_intervals];
    //     let set = IntervalContainer::new(records);
    //     let span = set.span();
    //     assert!(span.is_err());
    // }

    // #[test]
    // fn test_span_multiple_chr() {
    //     let n_intervals = 10;
    //     let mut records = vec![bed3![1, 10, 100, Strand::Forward]; n_intervals];
    //     records.push(bed3![2, 10, 100, Strand::Forward]);
    //     let set = IntervalContainer::new(records);
    //     let span = set.span();
    //     assert!(span.is_err());
    // }
    //
    // #[test]
    // fn test_span() {
    //     let records = vec![
    //         bed3![1, 10, 100, Strand::Forward],
    //         bed3![1, 1000, 2000, Strand::Forward],
    //     ];
    //     let set = IntervalContainer::from_sorted(records).unwrap();
    //     let span = set.span().unwrap();
    //     assert_eq!(span.start(), 10);
    //     assert_eq!(span.end(), 2000);
    // }

    #[test]
    fn test_sort() {
        let records = vec![
            bed3![1, 1000, 2000, Strand::Reverse],
            bed3![1, 1000, 2000, Strand::Forward],
            bed3![1, 1000, 2000, Strand::Unknown],
            bed3![1, 10, 100, Strand::Reverse],
            bed3![1, 10, 100, Strand::Forward],
            bed3![1, 10, 100, Strand::Unknown],
            bed3![2, 1000, 2000, Strand::Reverse],
            bed3![2, 1000, 2000, Strand::Forward],
            bed3![2, 1000, 2000, Strand::Unknown],
            bed3![2, 10, 100, Strand::Reverse],
            bed3![2, 10, 100, Strand::Forward],
            bed3![2, 10, 100, Strand::Unknown],
        ];
        let set = IntervalContainer::from_unsorted(records);
        assert!(set.is_sorted());
        let vec = Vec::from(set);
        assert!(vec[0].eq(&bed3![1, 10, 100, Strand::Forward]));
        assert!(vec[1].eq(&bed3![1, 10, 100, Strand::Reverse]));
        assert!(vec[2].eq(&bed3![1, 10, 100, Strand::Unknown]));
        assert!(vec[3].eq(&bed3![1, 1000, 2000, Strand::Forward]));
        assert!(vec[4].eq(&bed3![1, 1000, 2000, Strand::Reverse]));
        assert!(vec[5].eq(&bed3![1, 1000, 2000, Strand::Unknown]));
        assert!(vec[6].eq(&bed3![2, 10, 100, Strand::Forward]));
        assert!(vec[7].eq(&bed3![2, 10, 100, Strand::Reverse]));
        assert!(vec[8].eq(&bed3![2, 10, 100, Strand::Unknown]));
        assert!(vec[9].eq(&bed3![2, 1000, 2000, Strand::Forward]));
        assert!(vec[10].eq(&bed3![2, 1000, 2000, Strand::Reverse]));
        assert!(vec[11].eq(&bed3![2, 1000, 2000, Strand::Unknown]));
    }

    // #[test]
    // #[cfg(feature = "serde")]
    // fn test_serialization() {
    //     let n_intervals = 10;
    //     let records = vec![bed3![1, 10, 100, Strand::Reverse]; n_intervals];
    //     let set = IntervalContainer::new(records);
    //     let serialized = serialize(&set).unwrap();
    //     let deserialized: IntervalContainer<StrandedBed3<i32>, i32> =
    //         deserialize(&serialized).unwrap();
    //
    //     for (iv1, iv2) in set.records().iter().zip(deserialized.records().iter()) {
    //         assert!(iv1.eq(iv2));
    //     }
    // }

    #[test]
    #[cfg(feature = "rayon")]
    fn test_par_sort() {
        let n_intervals = 10;
        let records = vec![bed3![1, 10, 100, Strand::Reverse]; n_intervals];
        let mut set = IntervalContainer::new(records);
        set.par_sort();
        assert!(set.is_sorted());
    }

    #[test]
    fn test_container_init_new() {
        let records = vec![
            BaseInterval::new(15, 25),
            BaseInterval::new(10, 20),
            BaseInterval::new(5, 15),
        ];
        let set = IntervalContainer::new(records);
        assert_eq!(set.len(), 3);
        assert!(!set.is_sorted());
        assert!(!set.is_empty());
        assert_eq!(set.iter().next().unwrap().start(), 15);
    }

    #[test]
    fn test_container_init_from_sorted() {
        let records = vec![
            BaseInterval::new(5, 10),
            BaseInterval::new(10, 15),
            BaseInterval::new(15, 20),
        ];
        let set = IntervalContainer::from_sorted(records).unwrap();
        assert_eq!(set.len(), 3);
        assert!(set.is_sorted());
        assert!(!set.is_empty());
        assert_eq!(set.iter().next().unwrap().start(), 5);
    }

    #[test]
    fn test_container_init_from_unsorted() {
        let records = vec![
            BaseInterval::new(15, 25),
            BaseInterval::new(10, 20),
            BaseInterval::new(5, 15),
        ];
        let set = IntervalContainer::from_unsorted(records);
        assert_eq!(set.len(), 3);
        assert!(set.is_sorted());
        assert!(!set.is_empty());
        assert_eq!(set.iter().next().unwrap().start(), 5);
    }

    #[test]
    fn test_container_init_from_sorted_false_sorting() {
        let records = vec![
            BaseInterval::new(10, 15),
            BaseInterval::new(5, 10),
            BaseInterval::new(15, 20),
        ];
        let set = IntervalContainer::from_sorted(records);
        assert!(set.is_err());
    }

    #[test]
    fn test_container_apply_mut() {
        let records = vec![
            BaseInterval::new(15, 25),
            BaseInterval::new(10, 20),
            BaseInterval::new(5, 15),
        ];
        let mut set = IntervalContainer::from_unsorted(records);
        set.apply_mut(|rec| rec.extend(&2, None));
        let vec = Vec::from(set);
        assert_eq!(vec[0].start(), 3);
        assert_eq!(vec[0].end(), 17);
        assert_eq!(vec[1].start(), 8);
        assert_eq!(vec[1].end(), 22);
        assert_eq!(vec[2].start(), 13);
        assert_eq!(vec[2].end(), 27);
    }

    #[test]
    fn test_container_insert() {
        let mut set = IntervalContainer::empty();
        set.insert(BaseInterval::new(15, 25));
        set.insert(BaseInterval::new(10, 20));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_container_insert_sorted() {
        let mut set = IntervalContainer::empty();
        set.insert_sorted(BaseInterval::new(15, 25));
        set.insert_sorted(BaseInterval::new(10, 20));
        assert_eq!(set.len(), 2);
        assert_eq!(set.iter().next().unwrap().start(), 10);
        assert!(set.is_sorted());
    }

    #[test]
    fn container_iter() {
        let records = vec![
            BaseInterval::new(15, 25),
            BaseInterval::new(10, 20),
            BaseInterval::new(5, 15),
        ];
        let set = IntervalContainer::from_unsorted(records);
        let mut iter = set.iter();
        assert_eq!(iter.next().unwrap().start(), 5);
        assert_eq!(iter.next().unwrap().start(), 10);
        assert_eq!(iter.next().unwrap().start(), 15);
    }
}
