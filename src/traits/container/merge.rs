use crate::{traits::Coordinates, types::IntervalMeta};
use super::Container;

pub trait Merge<T, I>: Container<T, I>
where
    I: Coordinates<T>
{
    fn merge(&self) {
        let num_intervals = self.len();
        let base_interval = IntervalMeta::<usize, usize>::new(0, 0, None);
        println!("{}", num_intervals);
    }
}

#[cfg(test)]
mod testing {
    use crate::types::IntervalMetaSet;
    use super::Merge;


    #[test]
    fn test_merging() {
        let starts = vec![
            10,
            15,
            25,
        ];
        let ends = vec![
            30,
            20,
            30,
        ];
        let set = IntervalMetaSet::<usize, usize>::from_endpoints_unchecked(&starts, &ends);
        set.merge();
        // assert!(false);
    }
}
