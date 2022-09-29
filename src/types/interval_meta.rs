use crate::traits::{Coordinates, Overlap};

#[derive(Debug, Clone)]
pub struct IntervalMeta<T, M> {
    start: T,
    end: T,
    metadata: Option<M>,
}
impl<T, M> IntervalMeta<T, M> {
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
    pub fn start(&self) -> &T {
        &self.start
    }
    pub fn end(&self) -> &T {
        &self.end
    }
}
impl<T, M> Coordinates<T> for IntervalMeta<T, M> {
    fn start(&self) -> &T {
        self.start()
    }
    fn end(&self) -> &T {
        self.end()
    }
}
impl<T: PartialOrd, M> Overlap<T> for IntervalMeta<T, M> {}

#[cfg(test)]
mod testing {
    use super::IntervalMeta;

    #[test]
    fn test_interval_meta_init() {
        let start = 10;
        let end = 100;
        let metadata: Option<usize> = None;
        let interval = IntervalMeta::new(start, end, metadata);

        assert_eq!(interval.start(), &start);
        assert_eq!(interval.end(), &end);
        assert_eq!(interval.metadata(), &metadata);
    }
}
