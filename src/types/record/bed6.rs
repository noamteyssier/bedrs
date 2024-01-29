use crate::{
    traits::{ChromBounds, MetaBounds, ValueBounds},
    Bed12, Bed3, Bed4, Coordinates, Strand,
};
use num_traits::zero;

/// A representation of a Bed4 Interval.
///
/// Has six values
///     1. `chr`
///     2. `start`
///     3. `end`
///     4. `name`
///     5. `score`
///     6. `strand`
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
pub struct Bed6<C, T, N, S> {
    chr: C,
    start: T,
    end: T,
    name: N,
    score: S,
    strand: Strand,
}

impl<C, T, N, S> Coordinates<C, T> for Bed6<C, T, N, S>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
{
    fn empty() -> Self {
        Self {
            chr: C::default(),
            start: zero::<T>(),
            end: zero::<T>(),
            name: N::default(),
            score: zero::<S>(),
            strand: Strand::Unknown,
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
            score: zero::<S>(),
            strand: Strand::Unknown,
        }
    }
}
impl<'a, C, T, N, S> Coordinates<C, T> for &'a Bed6<C, T, N, S>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
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
impl<'a, C, T, N, S> Coordinates<C, T> for &'a mut Bed6<C, T, N, S>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
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

impl<C, T, N, S> Bed6<C, T, N, S>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
{
    pub fn new(chr: C, start: T, end: T, name: N, score: S, strand: Strand) -> Self {
        Self {
            chr,
            start,
            end,
            name,
            score,
            strand,
        }
    }

    pub fn name(&self) -> &N {
        &self.name
    }

    pub fn score(&self) -> &S {
        &self.score
    }

    pub fn strand(&self) -> Strand {
        self.strand
    }
}

impl<C, T, N, S> Into<Bed3<C, T>> for Bed6<C, T, N, S>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
{
    fn into(self) -> Bed3<C, T> {
        Bed3::new(self.chr, self.start, self.end)
    }
}

impl<C, T, N, S> Into<Bed4<C, T, N>> for Bed6<C, T, N, S>
where
    C: ChromBounds,
    T: ValueBounds,
    N: MetaBounds,
    S: ValueBounds,
{
    fn into(self) -> Bed4<C, T, N> {
        Bed4::new(self.chr, self.start, self.end, self.name)
    }
}

impl<C, T, N, S, Ts, Te, R, Si, St> Into<Bed12<C, T, N, S, Ts, Te, R, Si, St>> for Bed6<C, T, N, S>
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
            self.score,
            self.strand,
            Ts::default(),
            Te::default(),
            R::default(),
            zero::<T>(),
            Si::default(),
            St::default(),
        )
    }
}
