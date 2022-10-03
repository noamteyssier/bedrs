use crate::{
    traits::{IntervalBounds, ValueBounds},
    types::{SubtractFromIter, SubtractIter},
    Container,
};

pub trait SetSubtract<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    fn subtract<'a>(&'a self, query: &'a I) -> Option<SubtractIter<T, I>> {
        if self.is_sorted() {
            Some(self.subtract_unchecked(query))
        } else {
            None
        }
    }

    fn subtract_unchecked<'a>(&'a self, query: &'a I) -> SubtractIter<T, I> {
        SubtractIter::new(self.records(), query)
    }

    fn subtract_from<'a>(&'a self, query: &'a I) -> Option<SubtractFromIter<T, I>> {
        if self.is_sorted() {
            Some(self.subtract_from_unchecked(query))
        } else {
            None
        }
    }

    fn subtract_from_unchecked<'a>(&'a self, query: &'a I) -> SubtractFromIter<T, I> {
        SubtractFromIter::new(self, query)
    }
}

#[cfg(test)]
mod testing {
    use super::SetSubtract;
    use crate::{Container, Coordinates, Interval, IntervalSet};

    #[test]
    /// (q)       x------y
    /// (a)  i--j
    /// (b)         k--l
    /// (c)                m--n        
    /// ==========================
    /// (s1) i--j
    /// (s2)               m--n
    fn set_subtract_a() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 15);
        let b = Interval::new(25, 35);
        let c = Interval::new(45, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 15)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(45, 50)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i-------j
    /// (b)         k--l
    /// (c)                m--n        
    /// ==========================
    /// (s1) i---x
    /// (s2)               m--n
    fn set_subtract_b() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 25);
        let b = Interval::new(25, 35);
        let c = Interval::new(45, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(45, 50)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i--j
    /// (b)         k--l
    /// (c)            m------n        
    /// ==========================
    /// (s1) i--j
    /// (s2)             y----n
    fn set_subtract_c() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 15);
        let b = Interval::new(25, 35);
        let c = Interval::new(35, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 15)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(40, 50)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i------j
    /// (b)         k--l
    /// (c)            m------n        
    /// ==========================
    /// (s1) i----x
    /// (s2)             y----n
    fn set_subtract_d() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 25);
        let b = Interval::new(25, 35);
        let c = Interval::new(35, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(40, 50)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i--j
    /// (b)         k--l
    /// (c)                m--n        
    /// ==========================
    /// (s1)      x-k
    /// (s2)           l-y
    fn set_subtract_from_a() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 15);
        let b = Interval::new(25, 35);
        let c = Interval::new(45, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract_from(&q).unwrap();

        let iv = subset.next().unwrap();
        println!("{:?}", iv);
        assert!(iv.eq(&Interval::new(20, 25)));

        let iv = subset.next().unwrap();
        println!("{:?}", iv);
        assert!(iv.eq(&Interval::new(35, 40)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i------j
    /// (b)         k--l
    /// (c)                m--n        
    /// ==========================
    /// (s1)           l-y
    fn set_subtract_from_b() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 25);
        let b = Interval::new(25, 35);
        let c = Interval::new(45, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();

        let mut subset = set.subtract_from(&q).unwrap();

        let iv = subset.next().unwrap();
        println!("{:?}", iv);
        assert!(iv.eq(&Interval::new(35, 40)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i------j
    /// (b)         k--l
    /// (c)            m----n        
    /// ==========================
    /// None
    fn set_subtract_from_c() {
        let q = Interval::new(20, 40);
        let a = Interval::new(10, 25);
        let b = Interval::new(25, 35);
        let c = Interval::new(35, 50);
        let set = IntervalSet::from_sorted(vec![a, b, c]).unwrap();

        let mut subset = set.subtract_from(&q).unwrap();
        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)  x----------------y
    /// (a)     i--j
    /// (b)            k--l
    /// ==========================
    /// (s1) x--i
    /// (s2)       j---k
    /// (s3)              l---y
    fn set_subtract_from_d() {
        let q = Interval::new(20, 40);
        let a = Interval::new(25, 27);
        let b = Interval::new(32, 35);
        let set = IntervalSet::from_sorted(vec![a, b]).unwrap();

        let mut subset = set.subtract_from(&q).unwrap();
        let iv = subset.next().unwrap();
        // println!("{:?}", iv);
        assert!(iv.eq(&Interval::new(20, 25)));

        let iv = subset.next().unwrap();
        // println!("{:?}", iv);
        assert!(iv.eq(&Interval::new(27, 32)));

        let iv = subset.next().unwrap();
        // println!("{:?}", iv);
        assert!(iv.eq(&Interval::new(35, 40)));

        assert!(subset.next().is_none());
    }
}
