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

            // case where there is no overlap
            if !iv.overlaps(&self.remainder) {

                // if the interval is right shifted
                // make sure to store the current interval
                // and send any remainder
                if iv.gt(&self.remainder) {
                    if self.send_remainder {
                        let some_iv = self.remainder.to_owned();
                        self.remainder = iv.to_owned();
                        return Some(some_iv);
                    } 
                }
                return Some(iv.to_owned())
            }

            // perform the subtraction
            let mut sub_intervals = self.remainder.subtract(iv).expect("in subtraction set");

            // case where interval is interally overlapped
            let return_iv = if sub_intervals.len().eq(&2) {
                self.remainder = sub_intervals.pop().unwrap();
                sub_intervals.pop()
            } else {
                // pop the interval for inspection
                let some_iv = sub_intervals.pop().unwrap();

                // case where interval is left-shifted to query
                if some_iv.gt(&self.remainder) {
                    self.remainder.update_start(&some_iv.start());
                    continue;

                // case where interval is right-shifted to query
                } else {
                    self.send_remainder = false;
                    Some(some_iv)
                }
            };

            return return_iv;
        }

        // sends any remaining remainder
        if self.send_remainder {
            self.send_remainder = false;
            Some(self.remainder.to_owned())
        } else {
            None
        }
    }
}
