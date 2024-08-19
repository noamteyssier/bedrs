use crate::{
    traits::{ChromBounds, IntervalBounds},
    types::meta::RecordMetadata,
};
use coitrees::{BasicCOITree, GenericInterval, IntervalNode, IntervalTree};
use derive_new::new;
use hashbrown::HashMap;

/// Type alias for a HashMap where the key is a chromosome and the value is a BasicCOITree.
/// The BasicCOITree is a data structure used for efficient interval overlap queries.
pub type COIMap<C, M> = HashMap<C, BasicCOITree<M, usize>>;

/// A struct to hold coverage information.
///
/// This struct is used to store the number of overlaps and the total length of overlaps for a given interval.
#[derive(Debug, Clone, Copy, Default, new)]
pub struct Coverage {
    pub n_overlaps: usize,
    pub total_overlap_len: usize,
}
impl From<(usize, usize)> for Coverage {
    fn from((n_overlaps, total_overlap_len): (usize, usize)) -> Self {
        Self {
            n_overlaps,
            total_overlap_len,
        }
    }
}
impl From<Coverage> for (usize, usize) {
    fn from(cov: Coverage) -> (usize, usize) {
        (cov.n_overlaps, cov.total_overlap_len)
    }
}

/// A container for COITrees.
///
/// This struct is a wrapper around a HashMap where the key is a chromosome and the value is a BasicCOITree.
/// The BasicCOITree is a data structure used for efficient interval overlap queries.
#[derive(Clone, new)]
pub struct COITreeContainer<M, C>
where
    M: RecordMetadata,
    C: ChromBounds,
{
    inner: COIMap<C, M>,
}

impl<M, C> COITreeContainer<M, C>
where
    M: RecordMetadata,
    C: ChromBounds,
{
    /// Query the COITreeContainer with a given interval.
    ///
    /// This function will call the provided function `visit` for each interval in the COITree that overlaps with the query interval.
    pub fn query<F, I>(&self, query: &I, visit: F)
    where
        F: FnMut(&IntervalNode<M, usize>),
        I: IntervalBounds<C, M>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.query(query.start(), query.end(), visit);
        }
    }

    /// Query the COITreeContainer with a given interval, but allows for the visit function to fail.
    ///
    /// This function will call the provided function `visit` for each interval in the COITree that overlaps with the query interval.
    /// If the `visit` function returns an error, this function will immediately return that error.
    pub fn query_fallible<F, E, I>(&self, query: &I, visit: F) -> Result<(), E>
    where
        F: FnMut(&IntervalNode<M, usize>) -> Result<(), E>,
        I: IntervalBounds<C, M>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.query_fallible(query.start(), query.end(), visit)
        } else {
            Ok(())
        }
    }

    /// Count the number of intervals in the COITree that overlap with the query interval.
    pub fn query_count<I>(&self, query: &I) -> usize
    where
        I: IntervalBounds<C, M>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.query_count(query.start(), query.end())
        } else {
            0
        }
    }

    /// Calculate the coverage of the query interval.
    ///
    /// This function returns a tuple where the first element is the number of intervals in the COITree that overlap with the query interval,
    /// and the second element is the total length of the overlapping intervals.
    pub fn coverage<I>(&self, query: &I) -> Coverage
    where
        I: IntervalBounds<C, M>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.coverage(query.start(), query.end()).into()
        } else {
            Coverage::default()
        }
    }
}

/// Implement the FromIterator trait for COITreeContainer.
///
/// This allows for the creation of a COITreeContainer from an iterator that yields intervals.
impl<I, C, M> FromIterator<I> for COITreeContainer<M, C>
where
    I: IntervalBounds<C, M> + GenericInterval<M>,
    C: ChromBounds,
    M: RecordMetadata,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut map: HashMap<C, Vec<I>> = HashMap::new();
        for iv in iter {
            if let Some(c_vec) = map.get_mut(iv.chr()) {
                c_vec.push(iv);
            } else {
                map.insert(iv.chr().clone(), vec![iv]);
            }
        }
        let mut inner = COIMap::new();
        map.into_iter().for_each(|(chr, ivs)| {
            let coitree = BasicCOITree::new(&ivs);
            inner.insert(chr.clone(), coitree);
        });
        Self::new(inner)
    }
}

#[cfg(test)]
mod testing {
    use crate::prelude::*;
    use anyhow::bail;
    use coitrees::GenericInterval;
    use std::io::{Cursor, Write};

    #[test]
    fn test_query_missing_chr() {
        let set = COITreeContainer::from_iter(vec![
            bed3!(1, 10, 20),
            bed3!(1, 20, 30),
            bed3!(1, 30, 40),
            bed3!(2, 10, 20),
            bed3!(2, 20, 30),
            bed3!(2, 30, 40),
        ]);
        let query = bed3!(3, 15, 25);
        let mut num_overlaps = 0;
        set.query(&query, |_| {
            num_overlaps += 1;
        });
        assert_eq!(num_overlaps, 0);
    }

    #[test]
    fn test_query() {
        let coitrees = COITreeContainer::from_iter(vec![
            bed3!(1, 10, 20),
            bed3!(1, 20, 30),
            bed3!(1, 30, 40),
            bed3!(2, 10, 20),
            bed3!(2, 20, 30),
            bed3!(2, 30, 40),
        ]);
        let query = bed3!(1, 15, 25);
        let mut num_overlaps = 0;
        coitrees.query(&query, |_| {
            num_overlaps += 1;
        });
        assert_eq!(num_overlaps, 2);
    }

    #[test]
    fn test_query_stranded() {
        let set = IntervalContainer::new(vec![
            bed3!(1, 10, 20, Strand::Forward),
            bed3!(1, 20, 30, Strand::Reverse),
            bed3!(1, 30, 40, Strand::Forward),
            bed3!(2, 10, 20, Strand::Forward),
            bed3!(2, 20, 30, Strand::Forward),
            bed3!(2, 30, 40, Strand::Forward),
        ]);
        let coitrees = COITreeContainer::from(set);
        let query = bed3!(1, 15, 25, Strand::Reverse);
        let mut num_overlaps = 0;
        coitrees.query(&query, |iv| {
            if iv.metadata().strand().eq(&query.strand().unwrap()) {
                num_overlaps += 1;
            }
        });
        assert_eq!(num_overlaps, 1);
    }

    #[test]
    fn test_query_count() {
        let set = IntervalContainer::new(vec![
            bed3!(1, 10, 20),
            bed3!(1, 20, 30),
            bed3!(1, 30, 40),
            bed3!(2, 10, 20),
            bed3!(2, 20, 30),
            bed3!(2, 30, 40),
        ]);
        let coitrees = COITreeContainer::from(set);
        let query = bed3!(2, 15, 25);
        let num_overlaps = coitrees.query_count(&query);
        assert_eq!(num_overlaps, 2);
    }

    #[test]
    fn test_query_coverage() {
        let set = IntervalContainer::new(vec![
            bed3!(1, 10, 20),
            bed3!(1, 20, 30),
            bed3!(1, 30, 40),
            bed3!(2, 10, 20),
            bed3!(2, 20, 30),
            bed3!(2, 30, 40),
        ]);
        let coitrees = COITreeContainer::from(set);
        let query = bed3!(1, 15, 25);
        let coverage = coitrees.coverage(&query);
        assert_eq!(coverage.n_overlaps, 2);
        assert_eq!(coverage.total_overlap_len, 11);
    }

    #[test]
    fn test_query_fallible_write() {
        let set = IntervalContainer::new(vec![
            bed3!(1, 10, 20),
            bed3!(1, 20, 30),
            bed3!(1, 30, 40),
            bed3!(2, 10, 20),
            bed3!(2, 20, 30),
            bed3!(2, 30, 40),
        ]);
        let coitrees = COITreeContainer::from(set);
        let query = bed3!(1, 15, 25);
        let mut writer = Cursor::new(Vec::new());
        coitrees
            .query_fallible(&query, |_| writeln!(writer, "testing"))
            .unwrap();
        assert_eq!(writer.into_inner(), b"testing\ntesting\n");
    }

    #[test]
    fn test_query_fallible_anyhow() {
        let set = IntervalContainer::new(vec![
            bed3!(1, 10, 20),
            bed3!(1, 20, 30),
            bed3!(1, 30, 40),
            bed3!(2, 10, 20),
            bed3!(2, 20, 30),
            bed3!(2, 30, 40),
        ]);
        let coitrees = COITreeContainer::from(set);
        let query = bed3!(1, 15, 25);
        let mut vec = Vec::new();
        let res = coitrees.query_fallible(&query, |iv| {
            if iv.first() > 15 {
                vec.push(iv.first());
                Ok(())
            } else {
                bail!("Error");
            }
        });
        assert!(res.is_err());
    }
}
