use super::Container;
use crate::traits::{IntervalBounds, ValueBounds};

pub trait Find<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    type ContainerType: Container<T, I>;
    fn find(&self, query: &I) -> Self::ContainerType {
        let records = self
            .records()
            .iter()
            .filter(|x| x.overlaps(query))
            .map(|x| x.to_owned())
            .collect();
        Self::ContainerType::new(records)
    }
}

#[cfg(test)]
mod testing {
    use super::Find;
    use crate::{
        traits::Container,
        types::{Interval, IntervalSet},
    };

    #[test]
    fn test_find() {
        let query = Interval::new(17, 27);
        let starts = vec![10, 15, 20, 25];
        let ends = vec![40, 45, 50, 55];
        let set = IntervalSet::from_endpoints_unchecked(&starts, &ends);
        let overlaps = set.find(&query);
        assert_eq!(overlaps.len(), 4);
    }
}
