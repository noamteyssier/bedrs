use crate::{
    traits::{IntervalBounds, ValueBounds},
    Subtract,
};
use std::marker::PhantomData;

pub struct SubtractIter<'a, T, I>
where
    I: IntervalBounds<T> + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    remainder: I,
    offset: usize,
    send_remainder: bool,
    phantom_t: PhantomData<T>,
}
impl<'a, T, I> SubtractIter<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &I) -> Self {
        Self {
            inner,
            remainder: query.to_owned(),
            offset: 0,
            send_remainder: true,
            phantom_t: PhantomData,
        }
    }
}
impl<'a, T, I> Iterator for SubtractIter<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            // draw the next interval
            let iv = &self.inner[self.offset];
            self.offset += 1;

            // equality returns nothing - go to next;
            if iv.eq(&self.remainder) {
                continue;
            }

            // break the loop if we've gone past the query
            if !iv.overlaps(&self.remainder) && iv.gt(&self.remainder) {
                break;
            }

            // perform the subtraction
            let mut sub_intervals = self.remainder.subtract(iv).expect("in subtraction set");

            // case where interval is interally overlapped
            if sub_intervals.len().eq(&2) {
                self.remainder = sub_intervals.pop().unwrap();

            // case where interval is right-shifted to query
            } else {
                self.send_remainder = false;
            }

            return sub_intervals.pop();
        }

        // sends any relevant remainder
        if self.send_remainder {
            self.send_remainder = false;
            Some(self.remainder.to_owned())
        } else {
            None
        }
    }
}
