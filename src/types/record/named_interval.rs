use crate::traits::{ChromBounds, Coordinates, ValueBounds};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NamedInterval<N, T> {
    chr: N,
    start: T,
    end: T,
}

impl<N, T> Coordinates<N, T> for NamedInterval<N, T>
where
    N: ChromBounds,
    T: ValueBounds,
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> &N {
        &self.chr
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn update_chr(&mut self, val: &N) {
        self.chr = val.clone();
    }
    fn from(other: &Self) -> Self {
        Self {
            chr: other.chr().clone(),
            start: other.start(),
            end: other.end(),
        }
    }
}

impl<N, T> NamedInterval<N, T>
where
    N: ChromBounds,
    T: ValueBounds,
{
    pub fn new(chr: N, start: T, end: T) -> Self {
        Self { chr, start, end }
    }
}

#[cfg(test)]
mod testing {
    use std::cmp::Ordering;

    use super::NamedInterval;
    use crate::Coordinates;

    #[cfg(feature = "serde")]
    use bincode::{deserialize, serialize};

    #[test]
    fn named_interval_init() {
        let interval = NamedInterval::new("chr1".to_string(), 10, 100);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
        assert_eq!(interval.chr(), "chr1");
    }

    #[test]
    fn named_interval_init_bytes() {
        let interval = NamedInterval::new(b"chr1".to_vec(), 10, 100);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
        assert_eq!(interval.chr(), b"chr1");
    }

    #[test]
    fn named_interval_init_ref() {
        let name = "chr1".to_string();
        let interval = NamedInterval::new(name.as_str(), 10, 100);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
        assert_eq!(interval.chr(), &name);
    }

    #[test]
    fn named_interval_init_bytes_ref() {
        let name = b"chr1".to_vec();
        let interval = NamedInterval::new(name.as_slice(), 10, 100);
        assert_eq!(interval.start(), 10);
        assert_eq!(interval.end(), 100);
        assert_eq!(interval.chr(), b"chr1");
    }

    #[test]
    fn named_interval_bytes_lex() {
        let a = NamedInterval::new(b"chr1".to_vec(), 10, 100);
        let b = NamedInterval::new(b"chr2".to_vec(), 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = NamedInterval::new(b"chr1".to_vec(), 10, 100);
        let b = NamedInterval::new(b"chr1".to_vec(), 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = NamedInterval::new(b"chr2".to_vec(), 10, 100);
        let b = NamedInterval::new(b"chr1".to_vec(), 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = NamedInterval::new(b"chr2".to_vec(), 10, 100);
        let b = NamedInterval::new(b"chr10".to_vec(), 10, 99);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = NamedInterval::new(b"chr10".to_vec(), 10, 100);
        let b = NamedInterval::new(b"chr2".to_vec(), 10, 99);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn named_interval_ordering_gt() {
        let a = NamedInterval::new("chr1", 10, 100);
        let b = NamedInterval::new("chr1", 5, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = NamedInterval::new("chr1", 10, 100);
        let b = NamedInterval::new("chr1", 10, 99);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        let a = NamedInterval::new("chr2", 10, 100);
        let b = NamedInterval::new("chr1", 5, 99);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);

        // lexographic ordering
        let a = NamedInterval::new("chr2", 10, 100);
        let b = NamedInterval::new("chr10", 5, 99);
        assert_eq!(a.coord_cmp(&b), Ordering::Greater);
    }

    #[test]
    fn named_interval_ordering_lt() {
        let a = NamedInterval::new("chr1", 5, 100);
        let b = NamedInterval::new("chr1", 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = NamedInterval::new("chr1", 10, 99);
        let b = NamedInterval::new("chr1", 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        let a = NamedInterval::new("chr1", 5, 99);
        let b = NamedInterval::new("chr2", 1, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);

        // lexographic ordering
        let a = NamedInterval::new("chr10", 5, 99);
        let b = NamedInterval::new("chr2", 1, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Less);
    }

    #[test]
    fn named_interval_ordering_eq() {
        let a = NamedInterval::new("chr1", 10, 100);
        let b = NamedInterval::new("chr1", 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);

        let a = NamedInterval::new("chr2", 10, 100);
        let b = NamedInterval::new("chr2", 10, 100);
        assert_eq!(a.coord_cmp(&b), Ordering::Equal);
    }
    #[test]
    #[cfg(feature = "serde")]
    fn named_interval_serde() {
        let a = NamedInterval::new("chr1", 5, 100);
        let encoding = serialize(&a).unwrap();
        let b: NamedInterval<&str, usize> = deserialize(&encoding).unwrap();
        assert_eq!(a, b);
    }
}
