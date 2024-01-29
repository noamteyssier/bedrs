use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    Bed12, Bed3, Bed6, Coordinates, Strand,
};
use num_traits::zero;

/// A representation of a Bed4 Interval.
///
/// Has four values
///     1. `chr`
///     2. `start`
///     3. `end`
///     4. `name`
///
/// ```
/// use bedrs::{Coordinates, Bed4, Overlap};
///
/// let a = Bed4::new(1, 20, 30, 10);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
/// assert_eq!(*a.name(), 10);
///
/// let b = Bed4::new(1, 20, 30, 0);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed4<C, T, N> {
    chr: C,
    start: T,
    end: T,
    name: N,
}

impl<C, T, N> Coordinates<C, T> for Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
            name: N::default(),
        }
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
        &self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &C) {
        self.chr = val.clone();
    }
    fn from<Iv: Coordinates<C, T>>(other: &Iv) -> Self {
        Self {
            chr: other.chr().clone(),
            start: other.start(),
            end: other.end(),
            name: N::default(),
        }
    }
}
impl<'a, C, T, N> Coordinates<C, T> for &'a Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an immutable empty reference")
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
        &self.chr
    }
    #[allow(unused)]
    fn update_start(&mut self, val: &T) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn update_end(&mut self, val: &T) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn update_chr(&mut self, val: &C) {
        unreachable!("Cannot update an immutable reference")
    }
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a reference")
    }
}
impl<'a, C, T, N> Coordinates<C, T> for &'a mut Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn empty() -> Self {
        unreachable!("Cannot create an immutable empty reference")
    }
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &C {
        &self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &C) {
        self.chr = val.clone();
    }
    #[allow(unused)]
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create a new reference from a mutable reference")
    }
}

impl<C, T, N> Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(chr: C, start: T, end: T, name: N) -> Self {
        Self {
            chr,
            start,
            end,
            name,
        }
    }

    pub fn name(&self) -> &N {
        &self.name
    }
}

impl<C, T, N> Into<Bed3<C, T>> for Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn into(self) -> Bed3<C, T> {
        Bed3::new(self.chr, self.start, self.end)
    }
}

impl<C, T, N, S> Into<Bed6<C, T, N, S>> for Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: ValueBounds,
    S: ValueBounds,
{
    fn into(self) -> Bed6<C, T, N, S> {
        Bed6::new(
            self.chr,
            self.start,
            self.end,
            self.name,
            S::default(),
            Strand::Unknown,
        )
    }
}

impl<C, T, N, S, Ts, Te, R, Si, St> Into<Bed12<C, T, N, S, Ts, Te, R, Si, St>> for Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn into(self) -> Bed12<C, T, N, S, Ts, Te, R, Si, St> {
        Bed12::new(
            self.chr,
            self.start,
            self.end,
            self.name,
            S::default(),
            Strand::Unknown,
            Ts::default(),
            Te::default(),
            R::default(),
            zero::<T>(),
            Si::default(),
            St::default(),
        )
    }
}

#[cfg(test)]
mod testing {
    use crate::Bed3;
    use crate::Coordinates;

    use super::*;
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let interval = Bed4::new(1, 10, 100, 0);
        assert_eq!(*interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
        assert_eq!(interval.name(), &0);
    }

    #[test]
    fn test_name_init() {
        let interval = Bed4::new(1, 10, 100, 0);
        assert_eq!(*interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
        assert_eq!(interval.name(), &0);
    }

    #[test]
    fn test_bed4_from_bed3() {
        let iv = Bed3::new(1, 10, 100);
        let iv2: Bed4<i32, i32, i32> = iv.into();
        assert_eq!(*iv2.chr(), 1);
        assert_eq!(iv2.start(), 10);
        assert_eq!(iv2.end(), 100);
        assert_eq!(iv2.name(), &0);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = Bed4::new(1, 10, 100, 0);
        let b = Bed4::new(1, 5, 100, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = Bed4::new(1, 10, 100, 0);
        let b = Bed4::new(1, 10, 90, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = Bed4::new(2, 10, 100, 0);
        let b = Bed4::new(1, 10, 100, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = Bed4::new(1, 5, 100, 0);
        let b = Bed4::new(1, 10, 100, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = Bed4::new(1, 10, 100, 0);
        let b = Bed4::new(2, 10, 100, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = Bed4::new(1, 5, 100, 0);
        let b = Bed4::new(1, 5, 100, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = Bed4::new(2, 5, 100, 0);
        let b = Bed4::new(2, 5, 100, 0);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn genomic_interval_serde() {
        let a: Bed4<usize> = Bed4::new(1, 5, 100, 0);
        let encoding = serialize(&a).unwrap();
        let b: Bed4<usize> = deserialize(&encoding).unwrap();
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    fn function_generic_reference<C: Coordinates<usize, usize>>(iv: C) {
        assert_eq!(*iv.chr(), 1);
        assert_eq!(iv.start(), 10);
        assert_eq!(iv.end(), 100);
        assert!(iv.strand().is_none());
    }

    #[test]
    fn test_generic_reference() {
        let mut iv = Bed4::new(1, 10, 100, 0);
        function_generic_reference(&iv);
        function_generic_reference(&mut iv);
        function_generic_reference(iv);
    }
}
