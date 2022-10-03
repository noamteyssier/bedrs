use crate::{
    traits::{IntervalBounds, ValueBounds},
    types::iterator::SubtractIter,
    Container,
};

pub trait SetSubtract<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    fn subtract(&self, query: &I) -> Option<SubtractIter<T, I>> {
        if self.is_sorted() {
            Some(self.subtract_unchecked(query))
        } else {
            None
        }
    }

    fn subtract_unchecked(&self, query: &I) -> SubtractIter<T, I> {
        SubtractIter::new(self.records(), query)
    }
}

#[cfg(test)]
mod testing {
    use super::SetSubtract;
    use crate::{Container, Coordinates, Interval, IntervalSet};

    #[test]
    /// (a)   x--------------------y
    /// (b)       i--j
    /// (c)            k--l
    /// (d)                  m--n
    /// ==================================
    /// (s1) x---i
    /// (s2)         j-k
    /// (s3)              l--m
    /// (s4)                    n--y
    fn test_set_subtraction_a() {
        let a = Interval::new(10, 100);
        let b = Interval::new(20, 30);
        let c = Interval::new(40, 50);
        let d = Interval::new(60, 70);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(50, 60)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(70, 100)));
    }

    #[test]
    /// (a)   x--------------------y
    /// (b)       i--j
    /// (c)            k--l
    /// (d)                  m-------n
    /// ==================================
    /// (s1) x---i
    /// (s2)         j-k
    /// (s3)              l--m
    fn test_set_subtraction_b() {
        let a = Interval::new(10, 100);
        let b = Interval::new(20, 30);
        let c = Interval::new(40, 50);
        let d = Interval::new(60, 110);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(50, 60)));
    }

    #[test]
    /// (a)    x--------------------y
    /// (b)  i---j
    /// (c)          k--l
    /// (d)                  m--------n
    /// ==================================
    /// (s1)     j--k
    /// (s2)            l----m
    fn test_set_subtraction_c() {
        let a = Interval::new(10, 100);
        let b = Interval::new(5, 15);
        let c = Interval::new(40, 50);
        let d = Interval::new(60, 110);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(15, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(50, 60)));
    }

    #[test]
    /// (a)        x------y
    /// (b)  i-j
    /// (c)          k--l
    /// (d)                  m--n
    /// ==================================
    /// (s1) i-j
    /// (s2)       x-k
    /// (s3)            l-y
    /// (s4)                 m--n
    fn test_set_subtraction_d() {
        let a = Interval::new(40, 60);
        let b = Interval::new(20, 30);
        let c = Interval::new(45, 55);
        let d = Interval::new(70, 80);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();
        let subset = set.subtract(&a).unwrap();
        for s in subset {
            println!("{:?}", s);
        }

        let mut subset = set.subtract(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(20, 30)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(40, 45)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(55, 60)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(70, 80)));

    }
}
