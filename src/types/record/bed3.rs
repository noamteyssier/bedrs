use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    Bed12, Bed4, Bed6, Coordinates, Strand,
};
use num_traits::zero;

/// A representation of a Bed3 Interval.
///
/// Has three values
///     1. `chr`
///     2. `start`
///     3. `end`
///
/// ```
/// use bedrs::{Coordinates, Bed3, Overlap};
///
/// let a = Bed3::new(1, 20, 30);
/// assert_eq!(*a.chr(), 1);
/// assert_eq!(a.start(), 20);
/// assert_eq!(a.end(), 30);
///
/// let b = Bed3::new(1, 20, 30);
/// assert!(a.overlaps(&b));
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bed3<C, T> {
    chr: C,
    start: T,
    end: T,
}

impl<C, T> Coordinates<C, T> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
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
        }
    }
}
impl<'a, C, T> Coordinates<C, T> for &'a Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
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
impl<'a, C, T> Coordinates<C, T> for &'a mut Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
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

impl<C, T> Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    pub fn new(chr: C, start: T, end: T) -> Self {
        Self { chr, start, end }
    }
}

impl<C, T, N> Into<Bed4<C, T, N>> for Bed3<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
    N: ValueBounds,
{
    fn into(self) -> Bed4<C, T, N> {
        Bed4::new(self.chr, self.start, self.end, N::default())
    }
}

impl<C, T, N, S> Into<Bed6<C, T, N, S>> for Bed3<C, T>
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
            N::default(),
            S::default(),
            Strand::Unknown,
        )
    }
}

impl<C, T, N, S, Ts, Te, R, Si, St> Into<Bed12<C, T, N, S, Ts, Te, R, Si, St>> for Bed3<C, T>
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
            N::default(),
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
    use crate::{traits::Coordinates, types::Bed3};
    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let interval = Bed3::new(1, 10, 100);
        assert_eq!(*interval.chr(), 1);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = Bed3::new(1, 10, 100);
        let b = Bed3::new(1, 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = Bed3::new(1, 10, 100);
        let b = Bed3::new(1, 10, 90);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = Bed3::new(2, 10, 100);
        let b = Bed3::new(1, 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = Bed3::new(1, 5, 100);
        let b = Bed3::new(1, 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = Bed3::new(1, 10, 100);
        let b = Bed3::new(2, 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = Bed3::new(1, 5, 100);
        let b = Bed3::new(1, 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = Bed3::new(2, 5, 100);
        let b = Bed3::new(2, 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn genomic_interval_serde() {
        let a: Bed3<usize> = Bed3::new(1, 5, 100);
        let encoding = serialize(&a).unwrap();
        let b: Bed3<usize> = deserialize(&encoding).unwrap();
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
        let mut iv = Bed3::new(1, 10, 100);
        function_generic_reference(&iv);
        function_generic_reference(&mut iv);
        function_generic_reference(iv);
    }
}
