use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    types::Score,
    Bed12, Bed4, Bed6, Coordinates, Strand,
};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    fn update_strand(&mut self, strand: Option<Strand>) {
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

impl<C, T, N> From<Bed3<C, T>> for Bed4<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(bed: Bed3<C, T>) -> Self {
        Self::new(bed.chr, bed.start, bed.end, N::default())
    }
}

impl<C, T, N> From<Bed3<C, T>> for Bed6<C, T, N>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
{
    fn from(bed: Bed3<C, T>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            N::default(),
            Score::default(),
            Strand::Unknown,
        )
    }
}

impl<C, T, N, Ts, Te, R, Si, St> From<Bed3<C, T>> for Bed12<C, T, N, Ts, Te, R, Si, St>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(bed: Bed3<C, T>) -> Self {
        Self::new(
            bed.chr,
            bed.start,
            bed.end,
            N::default(),
            Score::default(),
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
    use super::*;

    #[test]
    fn test_init_numeric() {
        let a = Bed3::new(1, 20, 30);
        assert_eq!(*a.chr(), 1);
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
    }

    #[test]
    fn test_init_named() {
        let a = Bed3::new("chr1", 20, 30);
        assert_eq!(*a.chr(), "chr1");
        assert_eq!(a.start(), 20);
        assert_eq!(a.end(), 30);
    }

    #[test]
    fn convert_to_bed4() {
        let a = Bed3::new("chr1", 20, 30);
        let b: Bed4<_, _, i32> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), &0);
    }

    #[test]
    fn convert_to_bed6() {
        let a = Bed3::new("chr1", 20, 30);
        let b: Bed6<_, _, i32> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), &0);
        assert_eq!(b.score(), Score::Empty);
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
    }

    #[test]
    fn convert_to_bed12() {
        let a = Bed3::new("chr1", 20, 30);
        let b: Bed12<_, _, i32, i32, i32, i32, i32, i32> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
        assert_eq!(b.name(), &0);
        assert_eq!(b.score(), Score::Empty);
        assert_eq!(b.strand().unwrap(), Strand::Unknown);
        assert_eq!(b.thick_start(), 0);
        assert_eq!(b.thick_end(), 0);
        assert_eq!(b.item_rgb(), &0);
        assert_eq!(b.block_count(), 0);
        assert_eq!(b.block_sizes(), &0);
        assert_eq!(b.block_starts(), &0);
    }

    #[test]
    fn from_bed4() {
        let a = Bed4::new("chr1", 20, 30, 40);
        let b: Bed3<_, _> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
    }

    #[test]
    fn from_bed6() {
        let a = Bed6::new("chr1", 20, 30, 40, 50.into(), Strand::Forward);
        let b: Bed3<_, _> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
    }

    #[test]
    fn from_bed12() {
        let a = Bed12::new(
            "chr1",
            20,
            30,
            40,
            50.into(),
            Strand::Forward,
            60,
            70,
            80,
            90,
            100,
            110,
        );
        let b: Bed3<_, _> = a.into();
        assert_eq!(*b.chr(), "chr1");
        assert_eq!(b.start(), 20);
        assert_eq!(b.end(), 30);
    }
}
