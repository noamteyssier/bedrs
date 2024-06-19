use crate::traits::{ChromBounds, IntervalBounds};
use std::marker::PhantomData;

/// An iterator that determines overlapping intervals
/// and returns each interval with their associated
/// cluster ID
///
/// Expects sorted intervals.
/// Undefined behavior if the intervals are not sorted.
pub struct ClusterIter<It, I, C>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    iter: It,
    span: I,
    init: bool,
    current_id: usize,
    phantom_c: PhantomData<C>,
}
impl<It, I, C> ClusterIter<It, I, C>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    pub fn new(iter: It) -> Self {
        Self {
            iter,
            span: I::empty(),
            init: false,
            current_id: 0,
            phantom_c: PhantomData,
        }
    }

    fn next_interval(&mut self) -> Option<I> {
        self.iter.next()
    }

    /// Grows an existing cluster by updating the span
    /// to include the new interval
    fn grow_cluster(&mut self, iv: &I) {
        let new_min = self.span.start().min(iv.start());
        let new_max = self.span.end().max(iv.end());
        self.span.update_endpoints(&new_min, &new_max);
    }

    /// Initializes a new cluster by updating the span
    /// and incrementing the current ID
    fn new_cluster(&mut self, iv: &I) {
        self.current_id += 1;
        self.span.update_all_from(iv);
    }

    /// Sets the span to the first interval
    /// and sets the current ID to 0
    ///
    /// Should only be called once
    fn init_iterator(&mut self, iv: &I) {
        self.span.update_all_from(iv);
        self.current_id = 0;
        self.init = true;
    }
}
impl<It, I, C> Iterator for ClusterIter<It, I, C>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    type Item = (I, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let iv = self.next_interval()?;
        if self.init {
            if self.span.overlaps(&iv) || self.span.borders(&iv) {
                self.grow_cluster(&iv);
            } else {
                self.new_cluster(&iv);
            }
        } else {
            self.init_iterator(&iv);
        }
        Some((iv, self.current_id))
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{Bed3, Coordinates};

    type ClusterTuple = (Bed3<u32, u32>, usize);
    fn validate_clusters(observed: &[ClusterTuple], expected: &[ClusterTuple]) {
        println!("Expected:");
        for exp in expected {
            println!("{exp:?}");
        }

        println!("Observed:");
        for obs in observed {
            println!("{obs:?}");
        }

        assert_eq!(observed.len(), expected.len());
        for ((obs, co), (exp, ce)) in observed.iter().zip(expected.iter()) {
            assert_eq!(co, ce);
            assert_eq!(obs.chr(), exp.chr());
            assert_eq!(obs.start(), exp.start());
            assert_eq!(obs.end(), exp.end());
        }
    }

    /// (a)    i----j
    /// (b)      k----l
    /// (c)        m----n
    /// (d)                  o----p
    /// (e)                    q----r
    /// ===============================
    /// (1)    i----j
    /// (1)      k----l
    /// (1)        m----n
    /// (2)                  o----p
    /// (2)                    q----r
    #[test]
    fn cluster_iterator() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 15, 25),
            Bed3::new(1, 20, 30),
            Bed3::new(1, 40, 50),
            Bed3::new(1, 45, 55),
        ];
        let expected = vec![
            (Bed3::new(1, 10, 20), 0),
            (Bed3::new(1, 15, 25), 0),
            (Bed3::new(1, 20, 30), 0),
            (Bed3::new(1, 40, 50), 1),
            (Bed3::new(1, 45, 55), 1),
        ];
        let observed = ClusterIter::new(intervals.into_iter()).collect::<Vec<ClusterTuple>>();
        validate_clusters(&observed, &expected);
    }

    /// (a)    i----j
    /// (b)      k----l
    /// (c)              o----p
    /// (d)                q----r
    /// (e)                        s----t
    /// (f)                         u----v
    /// ===============================
    /// (1)    i----j
    /// (1)      k----l
    /// (2)              o----p
    /// (2)                q----r
    /// (3)                        s----t
    /// (3)                         u----v
    #[test]
    fn cluster_iterator_3_clusters() {
        let intervals = vec![
            Bed3::new(1, 10, 20),
            Bed3::new(1, 15, 25),
            Bed3::new(1, 40, 50),
            Bed3::new(1, 45, 55),
            Bed3::new(1, 60, 70),
            Bed3::new(1, 65, 75),
        ];
        let expected = vec![
            (Bed3::new(1, 10, 20), 0),
            (Bed3::new(1, 15, 25), 0),
            (Bed3::new(1, 40, 50), 1),
            (Bed3::new(1, 45, 55), 1),
            (Bed3::new(1, 60, 70), 2),
            (Bed3::new(1, 65, 75), 2),
        ];
        let observed = ClusterIter::new(intervals.into_iter()).collect::<Vec<ClusterTuple>>();
        validate_clusters(&observed, &expected);
    }

    /// Container:
    /// (a)    i------------------j
    /// (b)      k----l
    /// (c)        m----n
    /// ===============================
    /// (1)    i------------------j
    /// (1)      k----l
    /// (1)        m----n
    #[test]
    fn cluster_iterator_spanned() {
        let intervals = vec![
            Bed3::new(0, 10, 500),
            Bed3::new(0, 20, 60),
            Bed3::new(0, 30, 70),
        ];
        let expected = vec![
            (Bed3::new(0, 10, 500), 0),
            (Bed3::new(0, 20, 60), 0),
            (Bed3::new(0, 30, 70), 0),
        ];
        let observed = ClusterIter::new(intervals.into_iter()).collect::<Vec<ClusterTuple>>();
        validate_clusters(&observed, &expected);
    }

    /// Container:
    /// (a)    i-----j
    /// (b)      k---------l
    /// (c)        m----n
    /// ===============================
    /// (1)    i-----j
    /// (1)      k---------l
    /// (1)        m----n
    #[test]
    fn cluster_iterator_internal_span() {
        let intervals = vec![
            Bed3::new(0, 10, 50),
            Bed3::new(0, 20, 500),
            Bed3::new(0, 30, 70),
        ];
        let expected = vec![
            (Bed3::new(0, 10, 50), 0),
            (Bed3::new(0, 20, 500), 0),
            (Bed3::new(0, 30, 70), 0),
        ];
        let observed = ClusterIter::new(intervals.into_iter()).collect::<Vec<ClusterTuple>>();
        validate_clusters(&observed, &expected);
    }

    /// Container:
    /// (a)    i-----j
    /// (a)    i-----j
    /// (b)      k---------l
    /// (b)      k---------l
    /// (c)        m----------n
    /// (c)        m----------n
    /// (d)                      o----p
    /// (d)                      o----p
    /// ===============================
    /// (1)    i-----j
    /// (1)    i-----j
    /// (1)      k---------l
    /// (1)      k---------l
    /// (1)        m----------n
    /// (1)        m----------n
    /// (2)                      o----p
    /// (2)                      o----p
    #[test]
    fn cluster_iterator_duplicates() {
        let intervals = vec![
            Bed3::new(0, 10, 50),
            Bed3::new(0, 10, 50),
            Bed3::new(0, 30, 70),
            Bed3::new(0, 30, 70),
            Bed3::new(0, 50, 90),
            Bed3::new(0, 50, 90),
            Bed3::new(0, 100, 120),
            Bed3::new(0, 100, 120),
        ];
        let expected = vec![
            (Bed3::new(0, 10, 50), 0),
            (Bed3::new(0, 10, 50), 0),
            (Bed3::new(0, 30, 70), 0),
            (Bed3::new(0, 30, 70), 0),
            (Bed3::new(0, 50, 90), 0),
            (Bed3::new(0, 50, 90), 0),
            (Bed3::new(0, 100, 120), 1),
            (Bed3::new(0, 100, 120), 1),
        ];
        let observed = ClusterIter::new(intervals.into_iter()).collect::<Vec<ClusterTuple>>();
        validate_clusters(&observed, &expected);
    }
}
