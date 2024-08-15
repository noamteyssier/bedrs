use crate::{traits::ChromBounds, Strand};
use coitrees::GenericInterval;
use std::cmp::Ordering;

use super::*;

pub trait GenericIntervalExt<C, T>: GenericInterval<T>
where
    T: Clone,
    C: ChromBounds,
{
    fn start(&self) -> i32 {
        self.first()
    }

    fn end(&self) -> i32 {
        self.last()
    }

    fn chr(&self) -> &C;

    fn strand(&self) -> Option<Strand> {
        None
    }

    fn update_start(&mut self, val: &i32);

    fn update_end(&mut self, val: &i32);

    fn update_chr(&mut self, val: &C);

    fn update_strand(&mut self, _strand: Option<Strand>) {
        // Do nothing by default
    }

    fn from<Iv: GenericIntervalExt<C, T>>(other: &Iv) -> Self;

    fn empty() -> Self;

    fn update_all(&mut self, chr: &C, start: &i32, end: &i32) {
        self.update_chr(chr);
        self.update_endpoints(start, end);
    }

    fn update_endpoints(&mut self, start: &i32, end: &i32) {
        self.update_start(start);
        self.update_end(end);
    }

    fn update_all_from<I: GenericIntervalExt<C, T>>(&mut self, other: &I) {
        self.update_chr(other.chr());
        self.update_endpoints(&other.start(), &other.end());
    }

    fn update_endpoints_from<I: GenericIntervalExt<C, T>>(&mut self, other: &I) {
        self.update_start(&other.start());
        self.update_end(&other.end());
    }

    fn extend_left(&mut self, val: &i32) {
        if self.start() < *val {
            self.update_start(&0);
        } else {
            self.update_start(&(self.start() - val));
        }
    }

    fn extend_right(&mut self, val: &i32, max_bound: Option<i32>) {
        let new_end = self.end() + val;
        if let Some(max) = max_bound {
            self.update_end(&new_end.min(max));
        } else {
            self.update_end(&new_end);
        }
    }

    fn extend(&mut self, val: &i32, max_bound: Option<i32>) {
        self.extend_left(val);
        self.extend_right(val, max_bound);
    }

    fn f_len(&self, frac: f64) -> i32 {
        let len_f: f64 = self.len() as f64;
        let n = len_f * frac;
        n.round() as i32
    }

    fn coord_cmp<I: GenericIntervalExt<C, T>>(&self, other: &I) -> Ordering {
        match self.chr().cmp(other.chr()) {
            Ordering::Equal => match self.start().cmp(&other.start()) {
                Ordering::Equal => match self.end().cmp(&other.end()) {
                    Ordering::Equal => self.strand().cmp(&other.strand()),
                    order => order,
                },
                order => order,
            },
            order => order,
        }
    }

    fn biased_coord_cmp<I: GenericIntervalExt<C, T>>(&self, other: &I, bias: i32) -> Ordering {
        match self.chr().cmp(other.chr()) {
            Ordering::Equal => {
                let comp = if other.start() < bias {
                    None // can't compare the intervals since they both bias below zero
                } else {
                    Some(self.start().cmp(&(other.start() - bias)))
                };
                if let Some(comp) = comp {
                    match comp {
                        Ordering::Equal => match self.end().cmp(&other.end()) {
                            Ordering::Equal => self.strand().cmp(&other.strand()),
                            order => order,
                        },
                        order => order,
                    }
                } else {
                    Ordering::Equal
                }
            }
            order => order,
        }
    }

    fn biased_lt<I: GenericIntervalExt<C, T>>(&self, other: &I, bias: i32) -> bool {
        self.biased_coord_cmp(other, bias) == Ordering::Less
    }

    fn lt<I: GenericIntervalExt<C, T>>(&self, other: &I) -> bool {
        self.coord_cmp(other) == Ordering::Less
    }

    fn gt<I: GenericIntervalExt<C, T>>(&self, other: &I) -> bool {
        self.coord_cmp(other) == Ordering::Greater
    }

    fn eq<I: GenericIntervalExt<C, T>>(&self, other: &I) -> bool {
        self.coord_cmp(other) == Ordering::Equal
    }

    fn pprint(&self) -> String {
        format!(
            "{:?}:{:?}-{:?}:{}",
            self.chr(),
            self.start(),
            self.end(),
            self.strand().unwrap_or(Strand::Unknown)
        )
    }
}

impl<I, C, T> Overlap<C, T> for I
where
    I: GenericIntervalExt<C, T>,
    C: ChromBounds,
    T: Clone,
{
}

impl<I, C, T> UnstrandedOverlap<C, T> for I
where
    I: GenericIntervalExt<C, T>,
    C: ChromBounds,
    T: Clone,
{
}

impl<I, C, T> StrandedOverlap<C, T> for I
where
    I: GenericIntervalExt<C, T>,
    C: ChromBounds,
    T: Clone,
{
}

impl<I, C, T> Distance<C, T> for I
where
    I: GenericIntervalExt<C, T>,
    C: ChromBounds,
    T: Clone,
{
}

impl<I, C, T> Intersect<C, T> for I
where
    I: GenericIntervalExt<C, T>,
    C: ChromBounds,
    T: Clone,
{
}
