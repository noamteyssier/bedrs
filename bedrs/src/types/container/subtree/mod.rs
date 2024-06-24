mod bound;
mod closest;
mod merge;
mod sample;

use crate::{
    traits::{ChromBounds, IntervalBounds, SetError},
    Coordinates,
};
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use anyhow::Result;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// A wrapper type for a vector of interval records.
#[derive(Debug, Clone, Default)]
pub struct Subtree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    data: Vec<I>,
    max_len: Option<i32>,
    is_sorted: bool,
    _phantom: PhantomData<C>,
}

impl<I, C> FromIterator<I> for Subtree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn from_iter<It>(iter: It) -> Self
    where
        It: IntoIterator<Item = I>,
    {
        let mut data = Vec::new();
        let mut max_len = None;
        for iv in iter {
            if let Some(m) = max_len {
                max_len = Some(iv.len().max(m));
            } else {
                max_len = Some(iv.len());
            }
            data.push(iv);
        }
        Self {
            data,
            max_len,
            is_sorted: false,
            _phantom: PhantomData,
        }
    }
}

impl<I, C> Subtree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    #[must_use]
    pub fn new(data: Vec<I>) -> Self {
        Self::from_iter(data)
    }
    #[must_use]
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            max_len: None,
            is_sorted: false,
            _phantom: PhantomData,
        }
    }
    #[must_use]
    pub fn from_sorted(data: Vec<I>) -> Self {
        let mut new = Self::new(data);
        new.set_sorted();
        new
    }
    #[must_use]
    pub fn from_unsorted(data: Vec<I>) -> Self {
        let mut new = Self::new(data);
        new.sort();
        new
    }
    fn update_max_len(&mut self, record: &I) {
        if let Some(max_len) = self.max_len {
            if record.len() > max_len {
                self.max_len = Some(record.len());
            }
        } else {
            self.max_len = Some(record.len());
        }
    }
    #[must_use]
    pub fn max_len(&self) -> Option<i32> {
        self.max_len
    }
    pub fn max_len_mut(&mut self) -> &mut Option<i32> {
        &mut self.max_len
    }
    #[must_use]
    pub fn data(&self) -> &Vec<I> {
        &self.data
    }
    pub fn mut_data(&mut self) -> &mut Vec<I> {
        &mut self.data
    }
    pub fn insert(&mut self, record: I) {
        self.update_max_len(&record);
        self.data.push(record);
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn remove(&mut self, index: usize) -> I {
        self.data.remove(index)
    }
    /// Sets the internal state to sorted
    ///
    /// >> This would likely not be used directly by the user.
    /// >> If you are creating an interval set from presorted
    /// >> intervals use the `from_sorted()` method instead of
    /// >> the `new()` method.
    pub fn set_sorted(&mut self) {
        self.is_sorted = true;
    }

    pub fn set_unsorted(&mut self) {
        self.is_sorted = false;
    }

    #[must_use]
    pub fn is_sorted(&self) -> bool {
        self.is_sorted
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    pub fn sort(&mut self) {
        self.data.sort_unstable_by(Coordinates::coord_cmp);
        self.set_sorted();
    }

    /// Sorts the internal interval vector on the chromosome and start position of the intervals.
    /// but parallelizes the sorting.
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.data.par_sort_unstable_by(Coordinates::coord_cmp);
        self.set_sorted();
    }

    /// Applies a mutable function to each interval in the interval set.
    pub fn apply_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut I),
    {
        self.data.iter_mut().for_each(f);
    }

    pub fn span(&self) -> Result<I, SetError> {
        if self.is_empty() {
            return Err(SetError::EmptySet);
        } else if !self.is_sorted {
            return Err(SetError::UnsortedSet);
        }
        let first = &self.data[0];
        let last = &self.data[self.data.len() - 1];
        let mut iv = I::empty();
        iv.update_chr(first.chr());
        iv.update_start(&first.start());
        iv.update_end(&last.end());
        Ok(iv)
    }
}

impl<I, C> Index<usize> for Subtree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    type Output = I;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<I, C> IndexMut<usize> for Subtree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn index_mut(&mut self, index: usize) -> &mut I {
        &mut self.data[index]
    }
}

impl<I, C> IntoIterator for Subtree<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = I;
    type IntoIter = std::vec::IntoIter<I>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
