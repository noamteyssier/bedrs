use crate::{
    traits::{IntervalBounds, ValueBounds},
    types::MergeResults,
    Container, Merge, Subtract,
};
use std::marker::PhantomData;

pub struct SubtractIter<'a, T, I>
where
    I: IntervalBounds<T> + 'a,
    T: ValueBounds + 'a,
{
    inner: &'a Vec<I>,
    query: &'a I,
    offset: usize,
    phantom_t: PhantomData<T>,
}
impl<'a, T, I> SubtractIter<'a, T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    pub fn new(inner: &'a Vec<I>, query: &'a I) -> Self {
        Self {
            inner,
            query,
            offset: 0,
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
            return sub.pop();
        }
        None
    }
}

pub struct SubtractFromIter<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    inner: MergeResults<T, I>,
    remainder: Option<I>,
    stack: Option<I>,
    offset: usize,
    phantom_t: PhantomData<T>,
}
impl<T, I> SubtractFromIter<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    pub fn new<C: Container<T, I>>(container: &C, query: &I) -> Self {
        let merged_container = container.merge_unchecked();
        Self {
            inner: merged_container,
            remainder: Some(query.clone()),
            stack: None,
            offset: 0,
            phantom_t: PhantomData,
        }
    }
}
impl<T, I> Iterator for SubtractFromIter<T, I>
where
    I: IntervalBounds<T>,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.offset < self.inner.len() {
            // draw the next interval either from the process
            // stack or from the internal stockpile
            if self.stack.is_none() {
                let iv = &self.inner.records()[self.offset];
                self.offset += 1;
                self.stack = Some(iv.clone());
            }
            let iv = match &self.stack {
                Some(iv) => iv,
                None => {
                    break;
                }
            };

            // println!("====");
            // println!("Interval -> {:?} {:?}", iv.start(), iv.end());

            // Perform subtraction logic if there is a remainder left
            if let Some(ref mut remainder) = self.remainder {
                // println!("Remainder -> {:?} {:?}", remainder.start(), remainder.end());

                // Case where the interval does not overlap with the remainder
                // and it is right-shifted
                //
                // this returns the remainder and keeps the interval in the stack
                if !iv.overlaps(remainder) && iv.gt(remainder) {
                    let tmp = remainder.clone();
                    self.remainder = None;
                    return Some(tmp);
                }

                // If the remainder is contained by the interval then it is
                // fully squashed and no further intervals are necessary
                if iv.contains(remainder) {
                    self.remainder = None;
                    return None;
                }

                // Performs the subtraction
                let mut sub = iv.subtract(remainder).unwrap();

                // Case where there is a left or right shift in the subtraction
                if sub.len() == 1 {
                    let siv = sub.pop().unwrap();

                    // If the interval is left-shifted the remainder must be updated
                    // to reflect the endpoints of the cut interval
                    if siv.lt(remainder) {
                        if siv.end().ge(&remainder.start()) {
                            remainder.update_start(&iv.end());
                        } else {
                            self.stack = None;
                            continue;
                        }
                    // Otherwise we can just return the right-shifted cut interval
                    } else {
                        return Some(siv);
                    }
                // If the interval is contained by the interval then there are two cut
                // sub intervals.
                //
                // The RHS interval becomes the new remainder, and the LHS interval is returned
                } else {
                    self.remainder = sub.pop();
                    let result_siv = sub.pop();
                    self.stack = None;
                    return result_siv;
                }

                // Clears the stack at the end of all logical choices
                self.stack = None;
            }
        }

        // Case where there's still a remainder and the iterator
        // is past all the set intervals
        if let Some(ref remainder) = self.remainder {
            let tmp = remainder.clone();
            self.remainder = None;
            return Some(tmp);
        }

        // terminal case
        None
    }
}
