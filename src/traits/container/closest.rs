use crate::{
    traits::{ChromBounds, IntervalBounds, SetError, ValueBounds},
    Bound, Container, Distance,
};
use anyhow::Result;

pub trait Closest<C, T, I>: Container<C, T, I>
where
    C: ChromBounds,
    T: ValueBounds,
    I: IntervalBounds<C, T>,
{
    fn closest(&self, query: &I) -> Result<Option<&I>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.closest_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    fn closest_upstream(&self, query: &I) -> Result<Option<&I>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.closest_upstream_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    fn closest_downstream(&self, query: &I) -> Result<Option<&I>, SetError> {
        if self.is_sorted() {
            if self.records().is_empty() {
                return Err(SetError::EmptySet);
            }
            Ok(self.closest_downstream_unchecked(query))
        } else {
            Err(SetError::UnsortedSet)
        }
    }

    fn closest_unchecked(&self, query: &I) -> Option<&I> {
        if let Some(bound) = self.chr_bound_unchecked(query) {
            let mut current_dist = T::max_value();
            let mut current_lowest = bound;
            let mut position = bound;
            loop {
                if position == self.len() {
                    break;
                }
                let test_iv = &self.records()[position];
                let distance = query.distance(test_iv)?;
                if distance < current_dist {
                    current_dist = distance;
                    current_lowest = position;
                } else if distance > current_dist || distance == current_dist {
                    break;
                }
                position += 1;
            }
            Some(&self.records()[current_lowest])
        } else {
            None
        }
    }

    fn closest_upstream_unchecked(&self, query: &I) -> Option<&I> {
        if let Some(bound) = self.chr_bound_upstream_unchecked(query) {
            let mut current_dist = T::max_value();
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
                let distance = query.distance(test_iv)?;
                if distance < current_dist {
                    current_dist = distance;
                    current_lowest = position;
                } else if distance > current_dist || distance == current_dist {
                    break;
                }
                position += 1;
            }
            Some(&self.records()[current_lowest])
        } else {
            None
        }
    }

    fn closest_downstream_unchecked(&self, query: &I) -> Option<&I> {
        if let Some(bound) = self.chr_bound_downstream_unchecked(query) {
            let mut current_dist = T::max_value();
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
                let distance = query.distance(test_iv)?;
                if distance < current_dist {
                    current_dist = distance;
                    current_lowest = position;
                } else if distance > current_dist || distance == current_dist {
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
    use super::Closest;
    use crate::{
        Container, Coordinates, GenomicInterval, GenomicIntervalSet, Interval, IntervalSet,
    };

    #[test]
    fn closest_unsorted() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 23);
        let set = GenomicIntervalSet::new(intervals);
        assert!(set.closest(&query).is_err());
        assert!(set.closest_upstream(&query).is_err());
        assert!(set.closest_downstream(&query).is_err());
    }

    #[test]
    fn closest_empty() {
        let intervals = vec![];
        let query = GenomicInterval::new(1, 22, 23);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        assert!(set.closest(&query).is_err());
        assert!(set.closest_upstream(&query).is_err());
        assert!(set.closest_downstream(&query).is_err());
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-j
    /// =====================================
    ///    x-----y
    fn closest_a() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 23);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 10, 20));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-------j
    /// =====================================
    ///                 x-----y
    fn closest_b() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 32);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 30, 40));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-----j
    /// =====================================
    ///                 x-----y
    fn closest_c() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 30);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 30, 40));
    }

    #[test]
    /// |1|   x-----y      |2| x-----y       x-------y
    ///                    |2|           i-j
    /// =================================================
    ///                                      x-------y
    fn closest_d() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 30, 40),
            GenomicInterval::new(2, 50, 60),
        ];
        let query = GenomicInterval::new(2, 46, 47);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(2, 50, 60));
    }

    #[test]
    /// |1|   x-----y       x----y
    ///                              |2|  i-j
    /// ========================================
    /// None
    fn closest_e() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
        ];
        let query = GenomicInterval::new(2, 46, 47);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest(&query).unwrap();
        assert!(closest.is_none());
    }

    #[test]
    /// |1|   x-----y       x----y
    /// |1|            i-j
    /// ========================================
    ///       x-----y
    fn closest_f() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
        ];
        let query = GenomicInterval::new(1, 24, 26);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 10, 20));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-------j
    /// =====================================
    ///    x-----y
    fn closest_upstream_a() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 32);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_upstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 10, 20));
    }

    #[test]
    /// |1|  x-----y  |2|  x-----y   x-------y
    /// |2|                   i-------j
    /// =====================================
    ///                    x-----y
    fn closest_upstream_b() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 30, 40),
            GenomicInterval::new(2, 50, 60),
        ];
        let query = GenomicInterval::new(2, 32, 55);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_upstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(2, 30, 40));
    }

    #[test]
    /// |1|  x-----y  |2| x---y  x-----y   x-------y
    /// |2|                          i-------j
    /// =====================================
    ///                          x-----y
    fn closest_upstream_c() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 10, 20),
            GenomicInterval::new(2, 30, 40),
            GenomicInterval::new(2, 50, 60),
        ];
        let query = GenomicInterval::new(2, 32, 55);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_upstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(2, 30, 40));
    }

    #[test]
    ///    x-----y      x-----y   x-------y
    ///           i-------j
    /// =====================================
    ///                 x-----y
    fn closest_downstream_a() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(1, 30, 40),
            GenomicInterval::new(1, 50, 60),
        ];
        let query = GenomicInterval::new(1, 22, 32);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_downstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 30, 40));
    }

    #[test]
    /// |1|  x-----y  |2|  x-----y   x-------y
    /// |2|                   i-------j
    /// =====================================
    ///                              x-------y
    fn closest_downstream_b() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 30, 40),
            GenomicInterval::new(2, 50, 60),
        ];
        let query = GenomicInterval::new(2, 32, 55);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_downstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(2, 50, 60));
    }

    #[test]
    /// |1|  x-----y  |2|  x-----y   x-------y  x-----y
    /// |2|                   i-------j
    /// =====================================
    ///                              x-------y
    fn closest_downstream_c() {
        let intervals = vec![
            GenomicInterval::new(1, 10, 20),
            GenomicInterval::new(2, 30, 40),
            GenomicInterval::new(2, 50, 60),
            GenomicInterval::new(2, 70, 80),
        ];
        let query = GenomicInterval::new(2, 32, 55);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_downstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(2, 50, 60));
    }

    #[test]
    fn closest_downstream_d() {
        let intervals = vec![
            GenomicInterval::new(1, 70, 220), // <- min
            GenomicInterval::new(1, 142, 292),
            GenomicInterval::new(1, 154, 304),
        ];
        let query = GenomicInterval::new(1, 21, 71);
        let set = GenomicIntervalSet::from_unsorted(intervals);
        let closest = set.closest_downstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 70, 220));
    }

    #[test]
    fn closest_downstream_range_a() {
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let mut intervals = IntervalSet::from_endpoints(&starts, &ends).unwrap();
        intervals.sort();
        let query = Interval::new(12, 15);
        let closest = intervals.closest_downstream(&query).unwrap().unwrap();
        assert!(closest.eq(&Interval::new(12, 22)));
    }

    #[test]
    fn closest_downstream_range_b() {
        let chrs = (0..100).map(|x| x % 3).collect::<Vec<_>>();
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let mut intervals = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends).unwrap();
        intervals.sort();
        let query = GenomicInterval::new(1, 12, 15);
        let closest = intervals.closest_downstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(1, 13, 23));
    }

    #[test]
    fn closest_downstream_range_c() {
        let chrs = (0..100).map(|x| x % 3).collect::<Vec<_>>();
        let starts = (0..100).step_by(1).collect::<Vec<_>>();
        let ends = (10..110).step_by(1).collect::<Vec<_>>();
        let mut intervals = GenomicIntervalSet::from_endpoints(&chrs, &starts, &ends).unwrap();
        intervals.sort();
        let query = GenomicInterval::new(0, 12, 15);
        let closest = intervals.closest_downstream(&query).unwrap().unwrap();
        assert_eq!(closest, &GenomicInterval::new(0, 12, 22));
    }
}
