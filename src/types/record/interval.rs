use crate::traits::{Coordinates, Overlap};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Interval<T> {
    start: T,
    end: T,
}
impl<T> Interval<T>
where
    T: Copy,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
    pub fn from<I: Coordinates<T>>(other: &I) -> Self {
        Self {
            start: *other.start(),
            end: *other.end(),
        }
    }
    pub fn update_start(&mut self, value: &T) {
        self.start = *value;
    }
    pub fn update_end(&mut self, value: &T) {
        self.end = *value;
    }
}
impl<T> Coordinates<T> for Interval<T> {
    fn start(&self) -> &T {
        &self.start
    }
    fn end(&self) -> &T {
        &self.end
    }
}
impl<T: PartialOrd> Overlap<T> for Interval<T> {}
impl<T: Eq + Ord> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(other.start())
    }
}
impl<T: Ord> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start().partial_cmp(other.start())
    }
}

#[cfg(test)]
mod testing {
    use super::Interval;
    use crate::traits::Coordinates;

    #[test]
    fn test_interval_init() {
        let start = 10;
        let end = 100;
        let interval = Interval::new(start, end);

        assert_eq!(interval.start(), &start);
        assert_eq!(interval.end(), &end);
    }
}
