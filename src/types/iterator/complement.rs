use crate::traits::{ChromBounds, IntervalBounds, ValueBounds};
use std::{fmt::Debug, marker::PhantomData};

/// An iterator over the complement of a set of interval records.
///
/// This iterator expects the input to be sorted and pre-merged and will
/// panic if this is not the case.
pub struct ComplementIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    iter: It,
    current: Option<I>,
    last: Option<I>,
    phantom_c: PhantomData<C>,
    phantom_t: PhantomData<T>,
}
impl<It, I, C, T> ComplementIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(iter: It) -> Self {
        Self {
            iter,
            current: None,
            last: None,
            phantom_c: PhantomData,
            phantom_t: PhantomData,
        }
    }
    fn populate(&mut self) {
        if self.last.is_none() {
            self.last = self.iter.next();
        }
        if self.current.is_none() {
            self.current = self.iter.next();
        }
    }
}
impl<It, I, C, T> Iterator for ComplementIter<It, I, C, T>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C, T> + Debug,
    C: ChromBounds,
    T: ValueBounds,
{
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        self.populate();
        let current = self.current.take()?;
        let last = self.last.take()?;
        if current.chr() == last.chr() {
            if current.lt(&last) {
                panic!("Complement must be on sorted and merged intervals.")
            } else {
                let mut internal = I::from(&current);
                internal.update_start(&last.end());
                internal.update_end(&current.start());
                self.last = Some(current);
                Some(internal)
            }
        } else {
            self.last = Some(current);
            self.next()
        }
    }
}

#[cfg(test)]
mod testing {
    use super::ComplementIter;
    use crate::{
        traits::{ChromBounds, IntervalBounds, ValueBounds},
        Bed3, Interval,
    };

    fn validate_records<I, C, T>(obs: &[I], exp: &[I])
    where
        I: IntervalBounds<C, T>,
        C: ChromBounds,
        T: ValueBounds,
    {
        assert_eq!(obs.len(), exp.len());
        for (obs, exp) in obs.iter().zip(exp.iter()) {
            assert!(obs.eq(exp));
        }
    }

    #[test]
    /// x---------------y    i--------j
    /// ================================
    ///                 y----i
    fn complement_a() {
        let intervals = vec![Interval::new(10, 20), Interval::new(30, 40)];
        let expected = vec![Interval::new(20, 30)];
        let iter = intervals.into_iter();
        let comp_iter = ComplementIter::new(iter);
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }

    #[test]
    /// x---------------y    i--------j k----l
    /// =========================================
    ///                 y----i        j-k
    fn complement_b() {
        let intervals = vec![
            Interval::new(10, 20),
            Interval::new(30, 40),
            Interval::new(50, 60),
        ];
        let expected = vec![Interval::new(20, 30), Interval::new(40, 50)];
        let iter = intervals.into_iter();
        let comp_iter = ComplementIter::new(iter);
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }

    #[test]
    /// |1| x---------------y    i--------j |2| k----l
    /// ===============================================
    /// |1|                 y----i
    fn complement_c() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(2, 50, 60),
        ];
        let expected = vec![Bed3::new(1, 20, 30)];
        let iter = intervals.into_iter();
        let comp_iter = ComplementIter::new(iter);
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }

    #[test]
    /// |1| x---------------y    i--------j |2| k----l  m----n
    /// =======================================================
    /// |1|                 y----i          |2|      l--m
    fn complement_d() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(2, 10, 20),
            Bed3::new(2, 30, 40),
        ];
        let expected = vec![
            Bed3::new(1, 20, 30),
            Bed3::new(2, 20, 30),
        ];
        let iter = intervals.into_iter();
        let comp_iter = ComplementIter::new(iter);
        let complements: Vec<_> = comp_iter.collect();
        validate_records(&complements, &expected);
    }
}
