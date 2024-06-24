use derive_new::new;

use crate::{
    traits::{ChromBounds, IntervalBounds},
    types::{container::Subtree, Query},
};
use std::marker::PhantomData;

#[derive(new)]
pub struct FindIter<'a, C, I, Iv>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: Option<&'a Subtree<I, C>>,
    query: &'a Iv,
    offset: usize,
    method: Query,
}
impl<'a, C, I, Iv> Iterator for FindIter<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner?;
        while self.offset < inner.len() {
            let interval = &inner[self.offset];
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

#[derive(new)]
pub struct FindIterEnumerate<'a, C, I, Iv>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: Option<&'a Subtree<I, C>>,

    query: &'a Iv,
    offset: usize,
    phantom_c: PhantomData<C>,
    method: Query,
}
impl<'a, C, I, Iv> Iterator for FindIterEnumerate<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = (usize, &'a I);
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner?;
        while self.offset < inner.len() {
            let interval = &inner[self.offset];
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

#[derive(new)]
pub struct FindIterOwned<'a, C, I, Iv>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: Option<&'a Subtree<I, C>>,
    query: Iv,
    offset: usize,
    phantom_c: PhantomData<C>,
    method: Query,
}
impl<'a, C, I, Iv> Iterator for FindIterOwned<'a, C, I, Iv>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = &'a I;
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner?;
        while self.offset < inner.len() {
            let interval = &inner[self.offset];
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
