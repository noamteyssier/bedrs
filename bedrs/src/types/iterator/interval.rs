use std::{collections::VecDeque, marker::PhantomData};

use crate::{
    traits::{ChromBounds, IntervalBounds},
    IntervalContainer,
};

/// An iterator over a vector of interval records.
///
/// This iterator is intended to be used with a vector of interval records
/// and requires the records to be owned / cloned.
///
/// This will drain the vector of records.
///
/// # Example
///
/// ## Using a vector of interval records
///
/// ```
/// use bedrs::prelude::*;
///
/// let intervals = vec![
///     BaseInterval::new(1, 10),
///     BaseInterval::new(2, 20),
///     BaseInterval::new(3, 30),
/// ];
///
/// // build an iterator over the vector
/// let iter = IntervalIterOwned::new(intervals);
///
/// // iterate on the iterator
/// for interval in iter {
///    println!("{:?}", interval);
/// }
/// ```
///
/// ## Iterating on a container of interval records
///
/// ```
/// use bedrs::prelude::*;
///
/// let intervals = vec![
///     BaseInterval::new(1, 10),
///     BaseInterval::new(2, 20),
///     BaseInterval::new(3, 30),
/// ];
///
/// // build a container of interval records
/// let set = IntervalContainer::from_iter(intervals);
///
/// // iterate on the container
/// for interval in set.into_iter() {
///    println!("{:?}", interval);
/// }
/// ```
pub struct IntervalIterOwned<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    queue: VecDeque<I>,
    _phantom: PhantomData<C>,
}
impl<I, C> IntervalIterOwned<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    #[must_use]
    pub fn new(mut inner: IntervalContainer<I, C>) -> Self {
        let mut queue = VecDeque::new();
        let names: Vec<_> = inner.subtree_names_sorted().into_iter().cloned().collect();
        for n in names {
            let subtree = inner.subtree_owned(&n).unwrap();
            queue.extend(subtree.into_iter());
        }
        Self {
            queue,
            _phantom: PhantomData,
        }
    }
}
impl<I, C> Iterator for IntervalIterOwned<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

/// An iterator over a slice of interval records.
///
/// This iterator is intended to be used with a slice of interval records
/// and does not require the records to be owned / cloned.
///
/// This will not drain the slice of records.
///
/// # Example
///
/// ## Using a vector of interval records
///
/// ```
/// use bedrs::prelude::*;
///
/// let intervals = vec![
///     BaseInterval::new(1, 10),
///     BaseInterval::new(2, 20),
///     BaseInterval::new(3, 30),
/// ];
///
/// // build an iterator over the vector
/// let iter = IntervalIterRef::new(&intervals);
///
/// // iterate on the iterator
/// for interval in iter {
///    println!("{:?}", interval);
/// }
///
/// // The vector is still usable after the iteration
/// assert_eq!(intervals.len(), 3);
/// ```
///
/// ## Iterating on a container of interval records
///
/// ```
/// use bedrs::prelude::*;
///
/// let intervals = vec![
///     BaseInterval::new(1, 10),
///     BaseInterval::new(2, 20),
///     BaseInterval::new(3, 30),
/// ];
///
/// // build a container of interval records
/// let set = IntervalContainer::from_iter(intervals);
///
/// // iterate on the container
/// for interval in set.iter() {
///    println!("{:?}", interval);
/// }
///
/// // The container is still usable after the iteration
/// assert_eq!(set.len(), 3);
/// ```
pub struct IntervalIterRef<'a, I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    inner: &'a IntervalContainer<I, C>,
    names: Vec<&'a C>,
    name_idx: usize,
    iv_idx: usize,
}
impl<'a, I, C> IntervalIterRef<'a, I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    #[must_use]
    pub fn new(inner: &'a IntervalContainer<I, C>) -> Self {
        Self {
            inner,
            names: inner.subtree_names_sorted(),
            name_idx: 0,
            iv_idx: 0,
        }
    }
}
impl<'a, I, C> Iterator for IntervalIterRef<'a, I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        if self.name_idx < self.names.len() {
            let subtree = self.inner.subtree(self.names[self.name_idx])?;
            if self.iv_idx < subtree.len() {
                let iv = &subtree[self.iv_idx];
                self.iv_idx += 1;
                return Some(iv);
            }
            self.iv_idx = 0;
            self.name_idx += 1;
            return self.next();
        }
        None
    }
}

#[cfg(test)]
mod testing {
    use crate::{BaseInterval, Coordinates, IntervalContainer};

    #[test]
    fn iterator_owned() {
        let intervals = vec![
            BaseInterval::new(1, 10),
            BaseInterval::new(2, 20),
            BaseInterval::new(3, 30),
        ];
        let set = IntervalContainer::from_iter(intervals);
        let mut iter = set.into_iter();
        assert_eq!(iter.next().unwrap().start(), 1);
        assert_eq!(iter.next().unwrap().start(), 2);
        assert_eq!(iter.next().unwrap().start(), 3);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iterator_ref() {
        let intervals = vec![
            BaseInterval::new(1, 10),
            BaseInterval::new(2, 20),
            BaseInterval::new(3, 30),
        ];
        let set = IntervalContainer::from_iter(intervals);
        let mut iter = set.iter();
        assert_eq!(iter.next().unwrap().start(), 1);
        assert_eq!(iter.next().unwrap().start(), 2);
        assert_eq!(iter.next().unwrap().start(), 3);
        assert!(iter.next().is_none());
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn iterator_ref_multi() {
        let intervals = vec![
            BaseInterval::new(1, 10),
            BaseInterval::new(1, 20),
            BaseInterval::new(2, 20),
            BaseInterval::new(2, 30),
            BaseInterval::new(3, 30),
            BaseInterval::new(3, 40),
        ];
        let set = IntervalContainer::from_iter(intervals);
        let mut iter = set.iter();
        assert_eq!(iter.next().unwrap().start(), 1);
        assert_eq!(iter.next().unwrap().start(), 1);
        assert_eq!(iter.next().unwrap().start(), 2);
        assert_eq!(iter.next().unwrap().start(), 2);
        assert_eq!(iter.next().unwrap().start(), 3);
        assert_eq!(iter.next().unwrap().start(), 3);
        assert!(iter.next().is_none());
        assert_eq!(set.len(), 6);
    }
}
