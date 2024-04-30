use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    IntervalContainer, Subtract,
};
use std::marker::PhantomData;

pub struct SubtractIter<'a, I, Iv, C, T>
where
    I: IntervalBounds<C, T> + 'a,
    Iv: IntervalBounds<C, T> + 'a,
    C: ChromBounds + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a Iv,
    remainder: Option<I>,
    offset: usize,
    phantom_t: PhantomData<T>,
    phantom_c: PhantomData<C>,
}
impl<'a, I, Iv, C, T> SubtractIter<'a, I, Iv, C, T>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a Iv) -> Self {
        Self {
            inner,
            query,
            remainder: None,
            offset: 0,
            phantom_t: PhantomData,
            phantom_c: PhantomData,
        }
    }
}
impl<'a, I, Iv, C, T> Iterator for SubtractIter<'a, I, Iv, C, T>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref remainder) = self.remainder {
            let tmp = remainder.clone();
            self.remainder = None;
            return Some(tmp);
        }

        while self.offset < self.inner.len() {
            // draw the next interval
            let iv = &self.inner[self.offset];
            self.offset += 1;

            // skips interval if it is equal to query
            if iv.eq(self.query) {
                continue;
            }

            // skips interval if it is contained by query
            if iv.contained_by(self.query) {
                continue;
            }

            // Perform the subtraction
            let mut sub = iv.subtract(self.query).unwrap();

            // store the remainder if there is one
            if sub.len() == 2 {
                self.remainder = sub.pop();
            }
            return sub.pop();
        }
        None
    }
}

pub struct SubtractFromIter<I, Iv, C, T>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    inner: IntervalContainer<I, C, T>,
    remainder: Iv,
    send_remainder: bool,
    offset: usize,
    phantom_t: PhantomData<T>,
    phantom_c: PhantomData<C>,
}
impl<C, T, I, Iv> SubtractFromIter<I, Iv, C, T>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(container: &IntervalContainer<I, C, T>, query: &Iv) -> Self {
        let merged_container = container.merge_unchecked();
        Self {
            inner: merged_container,
            remainder: query.clone(),
            offset: 0,
            send_remainder: true,
            phantom_t: PhantomData,
            phantom_c: PhantomData,
        }
    }
}
impl<I, Iv, C, T> Iterator for SubtractFromIter<I, Iv, C, T>
where
    I: IntervalBounds<C, T>,
    Iv: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            // draw the next interval
            let iv = &self.inner.records()[self.offset];
            self.offset += 1;

            // equality returns nothing - go to next;
            if iv.eq(&self.remainder) {
                continue;
            }

            if iv.contains(&self.remainder) {
                self.send_remainder = false;
                return None;
            }

            // case where there is no overlap
            if !iv.overlaps(&self.remainder) {
                // if the interval is right shifted
                // and there is a remainder send the remainder
                // otherwise return none
                if iv.gt(&self.remainder) && self.send_remainder {
                    let some_iv = self.remainder.to_owned();
                    let some_iv = I::from(&some_iv);
                    self.send_remainder = false;
                    return Some(some_iv);
                }

                // if left shifted: skip to next interval
                continue;
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
                }
                // case where interval is right-shifted to query
                self.send_remainder = false;
                Some(some_iv)
            };

            let return_iv = return_iv.map(|iv| I::from(&iv));
            return return_iv;
        }

        // sends any remaining remainder
        if self.send_remainder {
            self.send_remainder = false;
            Some(I::from(&self.remainder))
            // Some(self.remainder.to_owned())
        } else {
            None
        }
    }
}
