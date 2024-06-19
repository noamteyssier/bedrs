use crate::{
    traits::{ChromBounds, IntervalBounds},
    types::Query,
};
use std::marker::PhantomData;

pub struct FindIter<'a, C, I, Iv>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a Iv,
    offset: usize,
    phantom_c: PhantomData<C>,
    method: Query,
}
impl<'a, C, I, Iv> FindIter<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv, offset: usize, method: Query) -> Self {
        Self {
            inner,
            query,
            offset,
            phantom_c: PhantomData,
            method,
        }
    }
}
impl<'a, C, I, Iv> Iterator for FindIter<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            let interval = &self.inner[self.offset];
            self.offset += 1;
            if self.method.predicate(interval, self.query) {
                return Some(interval);
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}

pub struct FindIterEnumerate<'a, C, I, Iv>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a Iv,
    offset: usize,
    phantom_c: PhantomData<C>,
    method: Query,
}
impl<'a, C, I, Iv> FindIterEnumerate<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv, offset: usize, method: Query) -> Self {
        Self {
            inner,
            query,
            offset,
            phantom_c: PhantomData,
            method,
        }
    }
}
impl<'a, C, I, Iv> Iterator for FindIterEnumerate<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = (usize, &'a I);
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            let interval = &self.inner[self.offset];
            self.offset += 1;
            if self.method.predicate(interval, self.query) {
                return Some((self.offset - 1, interval));
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}

pub struct FindIterOwned<'a, C, I, Iv>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: &'a Vec<I>,
    query: Iv,
    offset: usize,
    phantom_c: PhantomData<C>,
    method: Query,
}
impl<'a, C, I, Iv> FindIterOwned<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new(inner: &'a Vec<I>, query: Iv, offset: usize, method: Query) -> Self {
        Self {
            inner,
            query,
            offset,
            phantom_c: PhantomData,
            method,
        }
    }
}
impl<'a, C, I, Iv> Iterator for FindIterOwned<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            let interval = &self.inner[self.offset];
            self.offset += 1;
            if self.method.predicate(interval, &self.query) {
                return Some(interval);
            } else if interval.start() >= self.query.end() {
                break;
            }
        }
        None
    }
}
