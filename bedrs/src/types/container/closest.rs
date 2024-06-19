use crate::{
    traits::{ChromBounds, IntervalBounds, SetError},
    types::StrandMethod,
    Distance, IntervalContainer, Strand,
};
use anyhow::Result;

impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    /// Returns the closest interval to the query interval.
    pub fn closest<Iv>(&self, query: &Iv, method: StrandMethod) -> Result<Option<&I>, SetError>
    where
        Iv: IntervalBounds<C>,
    {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.closest_unchecked(query, method))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    pub fn closest_upstream<Iv>(
        &self,
        query: &Iv,
        method: StrandMethod,
    ) -> Result<Option<&I>, SetError>
    where
        Iv: IntervalBounds<C>,
    {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            // If the query is on the reverse strand, the upstream is the downstream
            // and vice versa.
            if let Some(Strand::Reverse) = query.strand() {
                Ok(self.closest_downstream_unchecked(query, method))
            } else {
                Ok(self.closest_upstream_unchecked(query, method))
            }
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    pub fn closest_downstream<Iv>(
        &self,
        query: &Iv,
        method: StrandMethod,
    ) -> Result<Option<&I>, SetError>
    where
        Iv: IntervalBounds<C>,
    {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            // If the query is on the reverse strand, the upstream and downstream
            // methods are reversed.
            if let Some(Strand::Reverse) = query.strand() {
                Ok(self.closest_upstream_unchecked(query, method))
            } else {
                Ok(self.closest_downstream_unchecked(query, method))
            }
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    pub fn closest_unchecked<Iv>(&self, query: &Iv, method: StrandMethod) -> Option<&I>
    where
        Iv: IntervalBounds<C>,
    {
        let bound = match self.bound_upstream_unchecked(query, method) {
            Some(bound) => bound,
            None => self.bound_downstream_unchecked(query, method)?,
        };
        let mut current_dist = i32::MAX;
        let mut current_lowest = bound;
        let mut position = bound;
        loop {
            if position == self.len() {
                break;
            }
            let test_iv = &self.records()[position];
            if let Some(distance) = query.distance(test_iv) {
                if distance < current_dist {
                    current_dist = distance;
                    current_lowest = position;
                } else if distance >= current_dist {
                    break;
                }
            } else {
                break;
            }
            position += 1;
        }
        Some(&self.records()[current_lowest])
    }

    pub fn closest_upstream_unchecked<Iv>(&self, query: &Iv, method: StrandMethod) -> Option<&I>
    where
        Iv: IntervalBounds<C>,
    {
        let bound_fn = match method {
            StrandMethod::Ignore => Self::bound_igstrand_upstream_unchecked::<Iv>,
            StrandMethod::MatchStrand => Self::bound_stranded_upstream_unchecked::<Iv>,
            StrandMethod::OppositeStrand => Self::bound_unstranded_upstream_unchecked::<Iv>,
        };
        if let Some(bound) = bound_fn(self, query) {
            let mut current_dist = i32::MAX;
            let mut current_lowest = bound;
            let mut position = bound;
            loop {
                if position == self.len() {
                    break;
                }
                let test_iv = &self.records()[position];
                if test_iv.gt(query) {
                    break;
                }
                match method {
                    StrandMethod::MatchStrand => {
                        if test_iv.strand() != query.strand() {
                            position += 1;
                            continue;
                        }
                    }
                    StrandMethod::OppositeStrand => {
                        if test_iv.strand() == query.strand() {
                            position += 1;
                            continue;
                        }
                    }
                    StrandMethod::Ignore => {}
                }
                if let Some(distance) = query.distance(test_iv) {
                    if distance < current_dist {
                        current_dist = distance;
                        current_lowest = position;
                    } else if distance >= current_dist {
                        break;
                    }
                } else {
                    break;
                }
                position += 1;
            }
            Some(&self.records()[current_lowest])
        } else {
            None
        }
    }

    pub fn closest_downstream_unchecked<Iv>(&self, query: &Iv, method: StrandMethod) -> Option<&I>
    where
        Iv: IntervalBounds<C>,
    {
        let bound_fn = match method {
            StrandMethod::Ignore => Self::bound_igstrand_downstream_unchecked::<Iv>,
            StrandMethod::MatchStrand => Self::bound_stranded_downstream_unchecked::<Iv>,
            StrandMethod::OppositeStrand => Self::bound_unstranded_downstream_unchecked::<Iv>,
        };
        if let Some(bound) = bound_fn(self, query) {
            let mut current_dist = i32::MAX;
            let mut current_lowest = bound;
            let mut position = bound;
            loop {
                if position == self.len() {
                    break;
                }
                let test_iv = &self.records()[position];
                if test_iv.lt(query) {
                    break;
                }
                match method {
                    StrandMethod::MatchStrand => {
                        if test_iv.strand() != query.strand() {
                            position += 1;
                            continue;
                        }
                    }
                    StrandMethod::OppositeStrand => {
                        if test_iv.strand() == query.strand() {
                            position += 1;
                            continue;
                        }
                    }
                    StrandMethod::Ignore => {}
                }
                if let Some(distance) = query.distance(test_iv) {
                    if distance < current_dist {
                        current_dist = distance;
                        current_lowest = position;
                    } else if distance >= current_dist {
                        break;
                    }
                } else {
                    break;
                }
                position += 1;
            }
            Some(&self.records()[current_lowest])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod testing {
    use crate::{
        types::StrandMethod, BaseInterval, Bed3, Coordinates, IntervalContainer, Strand,
        StrandedBed3,
    };

    #[test]
    fn closest_unsorted() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(1, 50, 60),
        ];
        let query = Bed3::new(1, 22, 23);
        let set = IntervalContainer::new(intervals);
        assert!(set.closest(&query, StrandMethod::Ignore).is_err());
        assert!(set.closest_upstream(&query, StrandMethod::Ignore).is_err());
        assert!(set
            .closest_downstream(&query, StrandMethod::Ignore)
            .is_err());
    }

    #[test]
    fn closest_empty() {
        let intervals: Vec<Bed3<i32, i32>> = vec![];
        let query = Bed3::new(1, 22, 23);
        let set = IntervalContainer::from_unsorted(intervals);
        assert!(set.closest(&query, StrandMethod::Ignore).is_err());
        assert!(set.closest_upstream(&query, StrandMethod::Ignore).is_err());
        assert!(set
            .closest_downstream(&query, StrandMethod::Ignore)
            .is_err());
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-j
    /// =====================================
    ///    x-----y
    fn closest_a() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(1, 50, 60),
        ];
        let query = Bed3::new(1, 22, 23);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(1, 10, 20)));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-------j
    /// =====================================
    ///                 x-----y
    fn closest_b() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(1, 50, 60),
        ];
        let query = Bed3::new(1, 22, 32);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(1, 30, 40)));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-----j
    /// =====================================
    ///                 x-----y
    fn closest_c() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(1, 50, 60),
        ];
        let query = Bed3::new(1, 22, 30);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(1, 30, 40)));
    }

    #[test]
    /// |1|   x-----y      |2| x-----y       x-------y
    ///                    |2|           i-j
    /// =================================================
    ///                                      x-------y
    fn closest_d() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 30, 40),
            Bed3::new(2, 50, 60),
        ];
        let query = Bed3::new(2, 46, 47);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(2, 50, 60)));
    }

    #[test]
    /// |1|   x-----y       x----y
    ///                              |2|  i-j
    /// ========================================
    /// None
    fn closest_e() {
        let intervals = vec![Bed3::new(1, 10, 20), Bed3::new(1, 30, 40)];
        let query = Bed3::new(2, 46, 47);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap();
        assert!(closest.is_none());
    }

    #[test]
    /// |1|   x-----y       x----y
    /// |1|            i-j
    /// ========================================
    ///       x-----y
    fn closest_f() {
        let intervals = vec![Bed3::new(1, 10, 20), Bed3::new(1, 30, 40)];
        let query = Bed3::new(1, 24, 26);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(1, 10, 20)));
    }

    #[test]
    /// |1|        x----y   x----y
    /// |1|   i-j
    /// ========================================
    ///           x-----y
    fn closest_g() {
        let set =
            IntervalContainer::from_unsorted(vec![Bed3::new(1, 20, 30), Bed3::new(1, 40, 50)]);
        let query = Bed3::new(1, 10, 15);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(1, 20, 30)));
    }

    #[test]
    /// 1       715     865
    /// 2       197     347
    /// 3       623     773
    /// 4       77      227
    /// 4       418     568
    /// 5       2       152
    /// 5       275     425
    /// 5       334     484
    /// 5       501     651
    /// 5       521     671
    fn closest_h() {
        let set = IntervalContainer::from_unsorted(vec![
            Bed3::new(1, 715, 865),
            Bed3::new(2, 197, 347),
            Bed3::new(3, 623, 773),
            Bed3::new(4, 77, 227),
            Bed3::new(4, 418, 568),
            Bed3::new(5, 2, 152),
            Bed3::new(5, 275, 425),
            Bed3::new(5, 334, 484),
            Bed3::new(5, 501, 651),
            Bed3::new(5, 521, 671),
        ]);
        let query = Bed3::new(1, 72, 222);
        let closest = set.closest(&query, StrandMethod::Ignore).unwrap().unwrap();
        assert!(closest.eq(&Bed3::new(1, 715, 865)));
    }

    #[test]
    /// 1   715 865 0   .   +
    /// 2   197 347 0   .   -
    /// 3   623 773 0   .   -
    /// 4   77  227 0   .   +
    /// 4   418 568 0   .   +
    /// 5   2   152 0   .   +
    /// 5   275 425 0   .   -
    /// 5   334 484 0   .   +
    /// 5   501 651 0   .   +
    /// 5   521 671 0   .   -
    fn closest_stranded_a() {
        let set = IntervalContainer::from_unsorted(vec![
            StrandedBed3::new(1, 715, 865, Strand::Forward),
            StrandedBed3::new(2, 197, 347, Strand::Reverse),
            StrandedBed3::new(3, 623, 773, Strand::Reverse),
            StrandedBed3::new(4, 77, 227, Strand::Forward),
            StrandedBed3::new(4, 418, 568, Strand::Forward),
            StrandedBed3::new(5, 2, 152, Strand::Forward),
            StrandedBed3::new(5, 275, 425, Strand::Reverse),
            StrandedBed3::new(5, 334, 484, Strand::Forward),
            StrandedBed3::new(5, 501, 651, Strand::Forward),
            StrandedBed3::new(5, 521, 671, Strand::Reverse),
        ]);
        let query = StrandedBed3::new(4, 212, 362, Strand::Forward);
        let closest = set
            .closest(&query, StrandMethod::MatchStrand)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&StrandedBed3::new(4, 77, 227, Strand::Forward)));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-------j
    /// =====================================
    ///    x-----y
    fn closest_upstream_a() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(1, 50, 60),
        ];
        let query = Bed3::new(1, 22, 32);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_upstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(1, 10, 20)));
    }

    #[test]
    /// |1|  x-----y  |2|  x-----y   x-------y
    /// |2|                   i-------j
    /// =====================================
    ///                    x-----y
    fn closest_upstream_b() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 30, 40),
            Bed3::new(2, 50, 60),
        ];
        let query = Bed3::new(2, 32, 55);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_upstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(2, 30, 40)));
    }

    #[test]
    /// |1|  x-----y  |2| x---y  x-----y   x-------y
    /// |2|                          i-------j
    /// =====================================
    ///                          x-----y
    fn closest_upstream_c() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 10, 20),
            Bed3::new(2, 30, 40),
            Bed3::new(2, 50, 60),
        ];
        let query = Bed3::new(2, 32, 55);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_upstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(2, 30, 40)));
    }

    #[test]
    ///    |--> <-_-|     |----->   |------->
    ///           |------->
    /// =====================================
    ///    |-->
    fn closest_upstream_stranded_matched() {
        let intervals = vec![
            StrandedBed3::new(1, 5, 15, Strand::Forward),
            StrandedBed3::new(1, 10, 20, Strand::Reverse),
            StrandedBed3::new(1, 30, 40, Strand::Forward),
            StrandedBed3::new(1, 50, 60, Strand::Forward),
        ];
        let query = StrandedBed3::new(1, 22, 32, Strand::Forward);
        let method = StrandMethod::MatchStrand;
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest_upstream_unchecked(&query, method).unwrap();
        assert!(closest.eq(&StrandedBed3::new(1, 5, 15, Strand::Forward)));
    }

    #[test]
    ///    |--> <---|     |----->   |------->
    ///           |------->
    /// =====================================
    ///         <---|
    fn closest_upstream_stranded_opposite() {
        let intervals = vec![
            StrandedBed3::new(1, 5, 15, Strand::Forward),
            StrandedBed3::new(1, 10, 20, Strand::Reverse),
            StrandedBed3::new(1, 30, 40, Strand::Forward),
            StrandedBed3::new(1, 50, 60, Strand::Forward),
        ];
        let query = StrandedBed3::new(1, 22, 32, Strand::Forward);
        let method = StrandMethod::OppositeStrand;
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set.closest_upstream_unchecked(&query, method).unwrap();
        assert!(closest.eq(&StrandedBed3::new(1, 10, 20, Strand::Reverse)));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-------j
    /// =====================================
    ///                 x-----y
    fn closest_downstream_a() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 30, 40),
            Bed3::new(1, 50, 60),
        ];
        let query = Bed3::new(1, 22, 32);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(1, 30, 40)));
    }

    #[test]
    /// |1|  x-----y  |2|  x-----y   x-------y
    /// |2|                   i-------j
    /// =====================================
    ///                              x-------y
    fn closest_downstream_b() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 30, 40),
            Bed3::new(2, 50, 60),
        ];
        let query = Bed3::new(2, 32, 55);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(2, 50, 60)));
    }

    #[test]
    /// |1|  x-----y  |2|  x-----y   x-------y  x-----y
    /// |2|                   i-------j
    /// =====================================
    ///                              x-------y
    fn closest_downstream_c() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(2, 30, 40),
            Bed3::new(2, 50, 60),
            Bed3::new(2, 70, 80),
        ];
        let query = Bed3::new(2, 32, 55);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(2, 50, 60)));
    }

    #[test]
    fn closest_downstream_d() {
        let intervals = vec![
            Bed3::new(1, 70, 220), // <- min
            Bed3::new(1, 142, 292),
            Bed3::new(1, 154, 304),
        ];
        let query = Bed3::new(1, 21, 71);
        let set = IntervalContainer::from_unsorted(intervals);
        let closest = set
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(1, 70, 220)));
    }

    #[test]
    fn closest_downstream_range_a() {
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let records = (0..100)
            .map(|x| BaseInterval::new(starts[x], ends[x]))
            .collect::<Vec<_>>();
        let intervals = IntervalContainer::from_unsorted(records);
        let query = BaseInterval::new(12, 15);
        let closest = intervals
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&BaseInterval::new(12, 22)));
    }

    #[test]
    fn closest_downstream_range_b() {
        let chrs = (0..100).map(|x| x % 3).collect::<Vec<_>>();
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let records = chrs
            .iter()
            .zip(starts.iter())
            .zip(ends.iter())
            .map(|((c, s), e)| Bed3::new(*c, *s, *e))
            .collect::<Vec<_>>();
        let intervals = IntervalContainer::from_unsorted(records);
        let query = Bed3::new(1, 12, 15);
        let closest = intervals
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(1, 13, 23)));
    }

    #[test]
    fn closest_downstream_range_c() {
        let chrs = (0..100).map(|x| x % 3).collect::<Vec<_>>();
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let records = chrs
            .iter()
            .zip(starts.iter())
            .zip(ends.iter())
            .map(|((c, s), e)| Bed3::new(*c, *s, *e))
            .collect::<Vec<_>>();
        let intervals = IntervalContainer::from_unsorted(records);
        let query = Bed3::new(0, 12, 15);
        let closest = intervals
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&Bed3::new(0, 12, 22)));
    }

    #[test]
    /// |--->            |---->
    ///         <---|
    /// =====================================
    /// |--->            
    fn closest_downstream_reverse_strand_a() {
        let set = IntervalContainer::from_unsorted(vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 40, 50, Strand::Forward),
        ]);
        let query = StrandedBed3::new(1, 22, 32, Strand::Reverse);
        let closest = set
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&StrandedBed3::new(1, 10, 20, Strand::Forward)));
    }

    #[test]
    /// |--->            |---->
    ///         |---->
    /// =====================================
    ///                  |--->
    fn closest_downstream_fwd_strand_a() {
        let set = IntervalContainer::from_unsorted(vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 40, 50, Strand::Forward),
        ]);
        let query = StrandedBed3::new(1, 22, 32, Strand::Forward);
        let closest = set
            .closest_downstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&StrandedBed3::new(1, 40, 50, Strand::Forward)));
    }

    #[test]
    /// |--->            |---->
    ///         <---|
    /// =====================================
    ///                  |--->
    fn closest_upstream_reverse_strand_a() {
        let set = IntervalContainer::from_unsorted(vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 40, 50, Strand::Forward),
        ]);
        let query = StrandedBed3::new(1, 22, 32, Strand::Reverse);
        let closest = set
            .closest_upstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&StrandedBed3::new(1, 40, 50, Strand::Forward)));
    }

    #[test]
    /// |--->            |---->
    ///         |---->
    /// =====================================
    /// |--->
    fn closest_upstream_fwd_strand_a() {
        let set = IntervalContainer::from_unsorted(vec![
            StrandedBed3::new(1, 10, 20, Strand::Forward),
            StrandedBed3::new(1, 40, 50, Strand::Forward),
        ]);
        let query = StrandedBed3::new(1, 22, 32, Strand::Forward);
        let closest = set
            .closest_upstream(&query, StrandMethod::Ignore)
            .unwrap()
            .unwrap();
        assert!(closest.eq(&StrandedBed3::new(1, 10, 20, Strand::Forward)));
    }
}
