use crate::{
    traits::{errors::SetError, ChromBounds, IntervalBounds},
    types::{SubtractFromIter, SubtractIter},
    IntervalContainer,
};

/// Performs interval subtraction at the set level.
impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    /// Subtract a query interval from the set.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // (q)       x------y
    /// // (a)  i--j
    /// // (b)         k--l
    /// // (c)                m--n        
    /// // ==========================
    /// // (s1) i--j
    /// // (s2)               m--n
    ///
    /// use bedrs::{Coordinates, BaseInterval, IntervalContainer};
    ///
    /// let q = BaseInterval::new(20, 40);
    /// let a = BaseInterval::new(10, 15);
    /// let b = BaseInterval::new(25, 35);
    /// let c = BaseInterval::new(45, 50);
    /// let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();
    /// let mut subset = set.subtract(&q).unwrap();
    ///
    /// let iv = subset.next().unwrap();
    /// assert!(iv.eq(&BaseInterval::new(10, 15)));
    ///
    /// let iv = subset.next().unwrap();
    /// assert!(iv.eq(&BaseInterval::new(45, 50)));
    ///
    /// assert!(subset.next().is_none());
    /// ```
    pub fn subtract<'a, Iv>(&'a self, query: &'a Iv) -> Result<SubtractIter<I, Iv, C>, SetError>
    where
        Iv: IntervalBounds<C>,
    {
        if self.is_sorted() {
            Ok(self.subtract_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Unchecked version of [subtract](Self::subtract).
    ///
    /// Does not check if the container is sorted
    pub fn subtract_unchecked<'a, Iv>(&'a self, query: &'a Iv) -> SubtractIter<I, Iv, C>
    where
        Iv: IntervalBounds<C>,
    {
        SubtractIter::new(self.records(), query)
    }

    /// Subtract the set from a query interval.
    ///
    /// # Example
    /// ```
    /// // (q)  x----------------y
    /// // (a)     i--j
    /// // (b)            k--l
    /// // ==========================
    /// // (s1) x--i
    /// // (s2)       j---k
    /// // (s3)              l---y
    ///
    /// use bedrs::{Coordinates, BaseInterval, IntervalContainer};
    ///
    /// let q = BaseInterval::new(20, 40);
    /// let a = BaseInterval::new(25, 27);
    /// let b = BaseInterval::new(32, 35);
    /// let set = IntervalContainer::from_sorted(vec![a, b]).unwrap();
    ///
    /// let mut subset = set.subtract_from(&q).unwrap();
    /// let iv = subset.next().unwrap();
    /// assert!(iv.eq(&BaseInterval::new(20, 25)));
    ///
    /// let iv = subset.next().unwrap();
    /// assert!(iv.eq(&BaseInterval::new(27, 32)));
    ///
    /// let iv = subset.next().unwrap();
    /// assert!(iv.eq(&BaseInterval::new(35, 40)));
    ///
    /// assert!(subset.next().is_none());
    /// ```
    pub fn subtract_from<'a, Iv>(
        &'a self,
        query: &'a Iv,
    ) -> Result<SubtractFromIter<I, Iv, C>, SetError>
    where
        Iv: IntervalBounds<C>,
    {
        if self.is_sorted() {
            Ok(self.subtract_from_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    /// Unchecked version of [`subtract_from`](Self::subtract_from).
    ///
    /// Does not check if the container is sorted
    pub fn subtract_from_unchecked<'a, Iv>(&'a self, query: &'a Iv) -> SubtractFromIter<I, Iv, C>
    where
        Iv: IntervalBounds<C>,
    {
        SubtractFromIter::new(self, query)
    }
}

#[cfg(test)]
mod testing {
    use crate::{BaseInterval, Coordinates, IntervalContainer};

    #[test]
    fn set_subtract_unsorted() {
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 15);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(45, 50);
        let set = IntervalContainer::new(vec![a, b, c]);
        let subset = set.subtract(&q);
        assert!(subset.is_err());
    }

    #[test]
    fn set_subtract_from_unsorted() {
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 15);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(45, 50);
        let set = IntervalContainer::new(vec![a, b, c]);
        let subset = set.subtract_from(&q);
        assert!(subset.is_err());
    }

    #[test]
    /// (q)       x------y
    /// (a)  i--j
    /// (b)         k--l
    /// (c)                m--n        
    /// ==========================
    /// (s1) i--j
    /// (s2)               m--n
    fn set_subtract_a() {
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 15);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(45, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 15)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(45, 50)));

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 25);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(45, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(45, 50)));

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 15);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(35, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 15)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(40, 50)));

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 25);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(35, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(40, 50)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (q)     x----y
    /// (a)  i----------j
    /// (b)  j----------k
    /// ====================
    /// (s1) i--x
    /// (s2)         y--j
    /// (s3) i--x
    /// (s4)         y--j
    fn set_subtract_e() {
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 50);
        let b = BaseInterval::new(10, 50);
        let set = IntervalContainer::from_sorted_unchecked(vec![a, b]);
        let mut subset = set.subtract(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(40, 50)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(40, 50)));

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 15);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(45, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();
        let mut subset = set.subtract_from(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(20, 25)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(35, 40)));

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 25);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(45, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();

        let mut subset = set.subtract_from(&q).unwrap();

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(35, 40)));

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(10, 25);
        let b = BaseInterval::new(25, 35);
        let c = BaseInterval::new(35, 50);
        let set = IntervalContainer::from_sorted(vec![a, b, c]).unwrap();

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
        let q = BaseInterval::new(20, 40);
        let a = BaseInterval::new(25, 27);
        let b = BaseInterval::new(32, 35);
        let set = IntervalContainer::from_sorted(vec![a, b]).unwrap();

        let mut subset = set.subtract_from(&q).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(20, 25)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(27, 32)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(35, 40)));

        assert!(subset.next().is_none());
    }

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
    fn set_subtract_from_e() {
        let a = BaseInterval::new(10, 100);
        let b = BaseInterval::new(20, 30);
        let c = BaseInterval::new(40, 50);
        let d = BaseInterval::new(60, 70);
        let set = IntervalContainer::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract_from(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(50, 60)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(70, 100)));

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
    fn set_subtract_from_f() {
        let a = BaseInterval::new(10, 100);
        let b = BaseInterval::new(20, 30);
        let c = BaseInterval::new(40, 50);
        let d = BaseInterval::new(60, 110);
        let set = IntervalContainer::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract_from(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(10, 20)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(30, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(50, 60)));

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
    fn set_subtract_from_g() {
        let a = BaseInterval::new(10, 100);
        let b = BaseInterval::new(5, 15);
        let c = BaseInterval::new(40, 50);
        let d = BaseInterval::new(60, 110);
        let set = IntervalContainer::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract_from(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(15, 40)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(50, 60)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)        x------y
    /// (b)  i-j
    /// (c)          k--l
    /// (d)                  m--n
    /// ==================================
    /// (s1)       x-k
    /// (s2)            l-y
    fn set_subtract_from_h() {
        let a = BaseInterval::new(40, 60);
        let b = BaseInterval::new(20, 30);
        let c = BaseInterval::new(45, 55);
        let d = BaseInterval::new(70, 80);
        let set = IntervalContainer::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract_from(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(40, 45)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(55, 60)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)    x------y
    /// (b)  i-j
    /// (c)      k--l
    /// (d)           m--n
    /// ==================================
    /// (s1)   x-k
    /// (s2)        l-y
    fn set_subtract_from_i() {
        let a = BaseInterval::new(40, 60);
        let b = BaseInterval::new(30, 40);
        let c = BaseInterval::new(45, 55);
        let d = BaseInterval::new(60, 70);
        let set = IntervalContainer::from_sorted(vec![b, c, d]).unwrap();

        let mut subset = set.subtract_from(&a).unwrap();
        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(40, 45)));

        let iv = subset.next().unwrap();
        assert!(iv.eq(&BaseInterval::new(55, 60)));

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)    x------y
    /// (b)  i----------j
    /// (c)  j----------k
    /// ==================================
    /// None
    fn set_subtract_from_j() {
        let a = BaseInterval::new(40, 60);
        let b = BaseInterval::new(30, 70);
        let c = BaseInterval::new(30, 70);
        let set = IntervalContainer::from_sorted(vec![b, c]).unwrap();
        let mut subset = set.subtract_from(&a).unwrap();

        assert!(subset.next().is_none());
    }

    #[test]
    /// (a)    x---------------y
    /// (b)    i---j
    /// (c)          k---l
    /// (d)                 m--n
    /// ==================================
    /// (s1)       j-k
    /// (s2)             l-m
    fn set_subtract_from_k() {
        let set = IntervalContainer::from_sorted(vec![
            BaseInterval::new(20, 30),
            BaseInterval::new(40, 50),
            BaseInterval::new(60, 70),
        ])
        .unwrap();
        let span = set.span().unwrap();
        let exp1 = BaseInterval::new(30, 40);
        let exp2 = BaseInterval::new(50, 60);
        let subset = set
            .subtract_from(&span)
            .unwrap()
            .collect::<IntervalContainer<BaseInterval<usize>, usize>>();
        assert_eq!(subset.len(), 2);
        assert!(subset.records()[0].eq(&exp1));
        assert!(subset.records()[1].eq(&exp2));
    }
}
