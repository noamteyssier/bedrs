use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    Bed12, Bed3, Bed6, Coordinates, Strand,
};
use num_traits::zero;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
    N: MetaBounds,
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

    pub fn update_name(&mut self, val: &N) {
        self.name = val.clone();
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
    N: MetaBounds,
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
    use super::*;

    #[test]
    fn test_init_chrom_numeric() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        assert_eq!(b.chr(), &1);
    }

    #[test]
    fn test_init_chrom_string() {
        let b = Bed4::new("chr1".to_string(), 10, 20, "test".to_string());
        assert_eq!(b.chr(), "chr1");
    }

    #[test]
    fn test_init_name_numeric() {
        let b = Bed4::new(1, 10, 20, 30);
        assert_eq!(b.name(), &30);
    }

    #[test]
    fn test_init_name_string() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        assert_eq!(b.name(), "test");
    }

    #[test]
    fn convert_to_bed3() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        let b3: Bed3<_, _> = b.into();
        assert_eq!(b3.chr(), &1);
        assert_eq!(b3.start(), 10);
        assert_eq!(b3.end(), 20);
    }

    #[test]
    fn convert_to_bed6() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        let b6: Bed6<i32, i32, String, i32> = b.into();
        assert_eq!(b6.chr(), &1);
        assert_eq!(b6.start(), 10);
        assert_eq!(b6.end(), 20);
        assert_eq!(b6.name(), "test");
    }

    #[test]
    fn convert_to_bed12() {
        let b = Bed4::new(1, 10, 20, "test".to_string());
        let b12: Bed12<i32, i32, String, i32, i32, i32, i32, i32, i32> = b.into();
        assert_eq!(b12.chr(), &1);
        assert_eq!(b12.start(), 10);
        assert_eq!(b12.end(), 20);
        assert_eq!(b12.name(), "test");
        assert_eq!(b12.score(), &0);
        assert_eq!(b12.strand(), Strand::Unknown);
        assert_eq!(b12.thick_start(), 0);
        assert_eq!(b12.thick_end(), 0);
        assert_eq!(b12.item_rgb(), &0);
        assert_eq!(b12.block_count(), 0);
        assert_eq!(b12.block_sizes(), &0);
        assert_eq!(b12.block_starts(), &0);
    }
}
