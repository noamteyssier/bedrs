use crate::{traits::{ValueBounds, IntervalBounds}, Container, types::iterator::StitchIter};


pub trait Stitch<T, I>: Container<T, I>
where
    T: ValueBounds,
    I: IntervalBounds<T>,
{
    fn stitch(&self, query: &I) -> Option<StitchIter<T, I>> {
        if self.is_sorted() {
            Some(self.stitch_unchecked(query))
        } else {
            None
        }
    }

    fn stitch_unchecked(&self, query: &I) -> StitchIter<T, I> {
        StitchIter::new(self.records(), query)
    }
}

#[cfg(test)]
mod testing {
    use super::Stitch;
    use crate::{Container, Coordinates, Interval, IntervalSet};

    #[test]
    /// (a)   x--------------------y
    /// (b)       i--j
    /// (c)            k--l
    /// (d)                  m--n
    /// ==================================
    /// (s1)  x--i
    /// (s2)         j-k
    /// (s3)              l--m
    /// (s4)                    n--y
    fn test_stitch_a() {
        let a = Interval::new(10, 100);
        let b = Interval::new(20, 30);
        let c = Interval::new(40, 50);
        let d = Interval::new(60, 70);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.stitch(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(50, 60)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(70, 100)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)   x--------------------y
    /// (b)       i--j
    /// (c)            k--l
    /// (d)                  m-------n
    /// ==================================
    /// (s1)  x--i
    /// (s2)         j-k
    /// (s3)              l--m
    fn test_stitch_b() {
        let a = Interval::new(10, 100);
        let b = Interval::new(20, 30);
        let c = Interval::new(40, 50);
        let d = Interval::new(60, 110);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.stitch(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(50, 60)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)    x--------------------y
    /// (b)  i---j
    /// (c)          k--l
    /// (d)                  m--------n
    /// ==================================
    /// (s1)     j--k
    /// (s2)            l----m
    fn test_stitch_c() {
        let a = Interval::new(10, 100);
        let b = Interval::new(5, 15);
        let c = Interval::new(40, 50);
        let d = Interval::new(60, 110);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.stitch(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(15, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(50, 60)));

        assert!(subset.next().is_none());
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
    fn test_stitch_d() {
        let a = Interval::new(40, 60);
        let b = Interval::new(20, 30);
        let c = Interval::new(45, 55);
        let d = Interval::new(70, 80);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.stitch(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(20, 30)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(40, 45)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(55, 60)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(70, 80)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)    x------y
    /// (b)  i-j
    /// (c)      k--l
    /// (d)           m--n
    /// ==================================
    /// (s1) i-j
    /// (s2)   x-k
    /// (s3)        l-y
    /// (s4)          m--n
    fn test_stitch_e() {
        let a = Interval::new(40, 60);
        let b = Interval::new(30, 40);
        let c = Interval::new(45, 55);
        let d = Interval::new(60, 70);
        let set = IntervalSet::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.stitch(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(40, 45)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(55, 60)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&Interval::new(60, 70)));
    }

    // #[test]
    // /// (a)    x------y
    // /// (b)  i----------j
    // /// (c)  j----------k
    // /// ==================================
    // /// (s1) i-x
    // /// (s2)          y-j
    // /// (s1) j-x
    // /// (s2)          y-k
    // fn test_stitch_f() {
    //     let a = Interval::new(40, 60);
    //     let b = Interval::new(30, 70);
    //     let c = Interval::new(30, 70);
    //     let set = IntervalSet::from_sorted(vec![b, c]).unwrap();

    //     for s in set.stitch(&a).unwrap() {
    //         println!("{:?}", s);
    //     }
    //     assert!(false);

    //     // let mut subset = set.stitch(&a).unwrap();

    //     // let iv = subset.next().unwrap();
    //     // assert!(iv.eq(&Interval::new(30, 40)));

    //     // let iv = subset.next().unwrap();
    //     // assert!(iv.eq(&Interval::new(60, 70)));

    //     // let iv = subset.next().unwrap();
    //     // assert!(iv.eq(&Interval::new(30, 40)));

    //     // let iv = subset.next().unwrap();
    //     // assert!(iv.eq(&Interval::new(60, 70)));
    // }
}
