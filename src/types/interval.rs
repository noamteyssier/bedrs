#[derive(Debug, Clone)]
pub struct Interval<T, M> {
    start: T,
    end: T,
    metadata: Option<M>,
}
impl<T, M> Interval<T, M> {
    pub fn new(start: T, end: T, metadata: Option<M>) -> Self {
        Self { start, end, metadata}
    }
    pub fn start(&self) -> &T {
        &self.start
    }
    pub fn end(&self) -> &T {
        &self.end
    }
    pub fn metadata(&self) -> &Option<M> {
        &self.metadata
    }
}

#[cfg(test)]
mod testing {
    use super::Interval;

    #[test]
    fn test_interval_init() {
        let start = 10;
        let end = 100;
        let metadata: Option<usize> = None;
        let interval = Interval::new(start, end, metadata);

        assert_eq!(interval.start(), &start);
        assert_eq!(interval.end(), &end);
        assert_eq!(interval.metadata(), &metadata);
    }
}
