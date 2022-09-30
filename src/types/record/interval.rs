use crate::traits::{Coordinates, Overlap};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct Interval<T>
where
    T: Copy + Default
{
    start: T,
    end: T,
}
impl<T> Interval<T>
where
    T: Copy + Default
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
    pub fn from<I: Coordinates<T>>(other: &I) -> Self {
        Self {
            start: other.start(),
            end: other.end(),
        }
    }
    pub fn update_start(&mut self, value: &T) {
        self.start = *value;
    }
    pub fn update_end(&mut self, value: &T) {
        self.end = *value;
    }
}
impl<T> Coordinates<T> for Interval<T>
where
    T: Copy + Default
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn from(other: &Self) -> Self {
        Self {
            start: other.start(),
            end: other.end(),
        }
    }
}
impl<T: PartialOrd> Overlap<T> for Interval<T> 
where
    T: Copy + PartialOrd + Default,
{
}
impl<T> Ord for Interval<T>
where
    T: Eq + Ord + Copy + Default,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(&other.start())
    }
}
impl<T> PartialOrd for Interval<T>
where
    T: Ord + Copy + Default,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start().partial_cmp(&other.start())
    }
}

#[cfg(test)]
mod testing {
    use crate::{traits::Coordinates, types::Interval};
    use std::cmp::Ordering;

    #[test]
    fn test_interval_init() {
        let start = 10;
        let end = 100;
        let interval = Interval::new(start, end);

        assert_eq!(interval.start(), start);
        assert_eq!(interval.end(), end);
    }

    #[test]
    fn test_interval_ordering_gt() {
        let a = Interval::new(10, 100);
        let b = Interval::new(5, 100);
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_interval_ordering_lt() {
        let a = Interval::new(5, 100);
        let b = Interval::new(10, 100);
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn test_interval_ordering_eq() {
        let a = Interval::new(5, 100);
        let b = Interval::new(5, 100);
        assert_eq!(a.cmp(&b), Ordering::Equal);

        let a = Interval::new(5, 100);
        let b = Interval::new(5, 90);
        assert_eq!(a.cmp(&b), Ordering::Equal);
    }
}
