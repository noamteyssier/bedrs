use crate::traits::{ChromBounds, IntervalBounds, ValueBounds};
use std::{collections::VecDeque, marker::PhantomData};

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
/// use bedrs::*;
///
/// let intervals = vec![
///     Interval::new(1, 10),
///     Interval::new(2, 20),
///     Interval::new(3, 30),
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
/// use bedrs::*;
///
/// let intervals = vec![
///     Interval::new(1, 10),
///     Interval::new(2, 20),
///     Interval::new(3, 30),
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
pub struct IntervalIterOwned<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    internal: VecDeque<I>,
    phantom_c: PhantomData<C>,
    phantom_t: PhantomData<T>,
}
impl<I, C, T> IntervalIterOwned<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(records: Vec<I>) -> Self {
        Self {
            internal: records.into(),
            phantom_c: PhantomData,
            phantom_t: PhantomData,
        }
    }
}
impl<I, C, T> Iterator for IntervalIterOwned<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        self.internal.pop_front()
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
/// use bedrs::*;
///
/// let intervals = vec![
///     Interval::new(1, 10),
///     Interval::new(2, 20),
///     Interval::new(3, 30),
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
/// use bedrs::*;
///
/// let intervals = vec![
///     Interval::new(1, 10),
///     Interval::new(2, 20),
///     Interval::new(3, 30),
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
pub struct IntervalIterRef<'a, I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    internal: &'a [I],
    phantom_c: PhantomData<C>,
    phantom_t: PhantomData<T>,
    state: usize,
}
impl<'a, I, C, T> IntervalIterRef<'a, I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(records: &'a [I]) -> Self {
        Self {
            internal: records,
            phantom_c: PhantomData,
            phantom_t: PhantomData,
            state: 0,
        }
    }
}
impl<'a, I, C, T> Iterator for IntervalIterRef<'a, I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state < self.internal.len() {
            let item = &self.internal[self.state];
            self.state += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{Coordinates, Interval, IntervalContainer};

    #[test]
    fn iterator_owned() {
        let intervals = vec![
            Interval::new(1, 10),
            Interval::new(2, 20),
            Interval::new(3, 30),
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
            Interval::new(1, 10),
            Interval::new(2, 20),
            Interval::new(3, 30),
        ];
        let set = IntervalContainer::from_iter(intervals);
        let mut iter = set.iter();
        assert_eq!(iter.next().unwrap().start(), 1);
        assert_eq!(iter.next().unwrap().start(), 2);
        assert_eq!(iter.next().unwrap().start(), 3);
        assert!(iter.next().is_none());
        assert_eq!(set.len(), 3);
    }
}
