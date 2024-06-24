use crate::{
    traits::{ChromBounds, IntervalBounds},
    types::meta::RecordMetadata,
};
use coitrees::{BasicCOITree, GenericInterval, IntervalNode, IntervalTree};
use derive_new::new;
use hashbrown::HashMap;

pub type COIMap<C, M> = HashMap<C, BasicCOITree<M, usize>>;

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
    pub fn query<F, I>(&self, query: &I, visit: F)
    where
        F: FnMut(&IntervalNode<M, usize>),
        I: IntervalBounds<C>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.query(query.start(), query.end(), visit);
        }
    }

    pub fn query_fallible<F, E, I>(&self, query: &I, visit: F) -> Result<(), E>
    where
        F: FnMut(&IntervalNode<M, usize>) -> Result<(), E>,
        I: IntervalBounds<C>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.query_fallible(query.start(), query.end(), visit)
        } else {
            Ok(())
        }
    }
    pub fn query_count<I>(&self, query: &I) -> usize
    where
        I: IntervalBounds<C>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.query_count(query.start(), query.end())
        } else {
            0
        }
    }
    pub fn coverage<I>(&self, query: &I) -> (usize, usize)
    where
        I: IntervalBounds<C>,
    {
        if let Some(coitree) = self.inner.get(query.chr()) {
            coitree.coverage(query.start(), query.end())
        } else {
            (0, 0)
        }
    }
}
impl<I, C, M> FromIterator<I> for COITreeContainer<M, C>
where
    I: IntervalBounds<C> + GenericInterval<M>,
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
    use super::*;
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
        let (num_overlaps, total_length) = coitrees.coverage(&query);
        assert_eq!(num_overlaps, 2);
        assert_eq!(total_length, 11);
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
