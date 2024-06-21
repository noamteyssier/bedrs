use crate::{
    traits::{ChromBounds, IntervalBounds},
    types::container::Subtree,
    IntervalContainer, Subtract,
};
use derive_new::new;
use std::marker::PhantomData;

#[derive(new)]
pub struct SubtractIter<'a, I, Iv, C>
where
    I: IntervalBounds<C> + 'a,
    Iv: IntervalBounds<C> + 'a,
    C: ChromBounds + 'a,
{
    inner: Option<&'a Subtree<I, C>>,
    query: &'a Iv,
    #[new(default)]
    remainder: Option<I>,
    #[new(default)]
    offset: usize,
    phantom_c: PhantomData<C>,
}
impl<'a, I, Iv, C> Iterator for SubtractIter<'a, I, Iv, C>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.inner?;
        if let Some(ref remainder) = self.remainder {
            let tmp = remainder.clone();
            self.remainder = None;
            return Some(tmp);
        }

        while self.offset < inner.len() {
            // draw the next interval
            let iv = &inner[self.offset];
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

pub struct SubtractFromIter<I, Iv, C>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    inner: IntervalContainer<I, C>,
    remainder: Iv,
    send_remainder: bool,
    names: Vec<C>,
    name_idx: usize,
    iv_idx: usize,
}
impl<C, I, Iv> SubtractFromIter<I, Iv, C>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new(container: &IntervalContainer<I, C>, query: &Iv) -> Self {
        let inner = container.merge().unwrap();
        let names = inner.subtree_names_sorted().into_iter().cloned().collect();
        Self {
            inner,
            remainder: query.clone(),
            names,
            name_idx: 0,
            iv_idx: 0,
            send_remainder: true,
        }
    }
}
impl<I, Iv, C> Iterator for SubtractFromIter<I, Iv, C>
where
    I: IntervalBounds<C>,
    Iv: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        while self.name_idx < self.names.len() {
            let subtree = self.inner.subtree(&self.names[self.name_idx]).unwrap();
            while self.iv_idx < self.inner.len() {
                // draw the next interval
                let iv = &subtree[self.iv_idx];
                self.iv_idx += 1;

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
            self.name_idx += 1;
            self.iv_idx = 0;
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
