use crate::traits::{Coordinates, Overlap};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct IntervalMeta<T, M> 
where
    T: Copy,
    M: Copy,
{
    start: T,
    end: T,
    metadata: Option<M>,
}
impl<T, M> IntervalMeta<T, M>
where
    T: Copy,
    M: Copy,
{
    pub fn new(start: T, end: T, metadata: Option<M>) -> Self {
        Self {
            start,
            end,
            metadata,
        }
    }
    pub fn metadata(&self) -> &Option<M> {
        &self.metadata
    }
}
impl<T, M> Coordinates<T> for IntervalMeta<T, M> 
where
    T: Copy + Default,
    M: Copy
{
    fn start(&self) -> T {
        self.start
    }
    fn end(&self) -> T {
        self.end
    }
    fn chr(&self) -> T {
        T::default()
    }
    fn update_start(&mut self, val: &T) {
        self.start = *val;
    }
    fn update_end(&mut self, val: &T) {
        self.end = *val;
    }
    fn from(other: &Self) -> Self {
        Self {
            start: other.start(),
            end: other.end(),
            metadata: *other.metadata()
        }
    }
}
impl<T, M> Overlap<T> for IntervalMeta<T, M>
where
    T: Copy + PartialOrd + Default,
    M: Copy,
{}

impl<T, M> Ord for IntervalMeta<T, M>
where
    T: Eq + Ord + Copy + Default,
    M: Eq + Copy,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(&other.start())
    }
}

impl<T, M> PartialOrd for IntervalMeta<T, M> 
where
    T: Ord + Copy + Default,
    M: Eq + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start().partial_cmp(&other.start())
    }
}

#[cfg(test)]
mod testing {
    use super::IntervalMeta;
    use crate::traits::Coordinates;

    #[test]
    fn test_interval_meta_init() {
        let start = 10;
        let end = 100;
        let metadata: Option<usize> = None;
        let interval = IntervalMeta::new(start, end, metadata);

        assert_eq!(interval.start(), start);
        assert_eq!(interval.end(), end);
        assert_eq!(interval.metadata(), &metadata);
    }
}
