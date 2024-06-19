// use super::Container;
use crate::{
    traits::{ChromBounds, IntervalBounds, SetError},
    IntervalContainer,
};

impl<I, C> IntervalContainer<I, C>
where
    I: IntervalBounds<C>,
    C: ChromBounds,
{
    fn grow_cluster(span: &mut I, iv: &I, endpoints: &mut Vec<i32>, n_iv: &mut usize) {
        // Update the span
        let new_min = span.start().min(iv.start());
        let new_max = span.end().max(iv.end());
        span.update_start(&new_min);
        span.update_end(&new_max);

        // store the start and end of current interval
        endpoints.push(iv.start());
        endpoints.push(iv.end());
        *n_iv += 1;
    }

    fn process_starts(reference: &I, endpoints: &[i32], num_iv: usize, segments: &mut Vec<I>) {
        for idx in 0..num_iv - 1 {
            let a = endpoints[idx];
            let b = endpoints[idx + 1];
            if a == b {
                continue;
            }
            let mut seg = I::from(reference);
            seg.update_start(&a);
            seg.update_end(&b);
            segments.push(seg);
        }
    }

    fn process_center(reference: &I, endpoints: &[i32], num_iv: usize, segments: &mut Vec<I>) {
        let a = &endpoints[num_iv - 1];
        let b = &endpoints[num_iv];
        if a >= b {
            return;
        }
        let mut inner_segment = I::from(reference);
        inner_segment.update_start(a);
        inner_segment.update_end(b);
        segments.push(inner_segment);
    }

    fn process_ends(reference: &I, endpoints: &[i32], num_iv: usize, segments: &mut Vec<I>) {
        for idx in 0..num_iv - 1 {
            let a = endpoints[num_iv + idx];
            let b = endpoints[num_iv + idx + 1];
            if a == b {
                continue;
            }
            let mut seg = I::from(reference);
            seg.update_start(&a);
            seg.update_end(&b);
            segments.push(seg);
        }
    }

    fn segment_cluster(reference: &I, endpoints: &mut [i32], n_iv: usize, segments: &mut Vec<I>) {
        endpoints.sort_unstable();
        Self::process_starts(reference, endpoints, n_iv, segments);
        Self::process_center(reference, endpoints, n_iv, segments);
        Self::process_ends(reference, endpoints, n_iv, segments);
    }

    /// Resets the cluster with the new interval
    fn init_cluster(span: &mut I, interval: &I, endpoints: &mut Vec<i32>, n_iv: &mut usize) {
        // Reset the span with the new interval
        span.update_all_from(interval);

        // With the new span, reset the starts and ends
        endpoints.clear();

        // Reset the number of intervals
        *n_iv = 0;

        // store the start and end of current interval
        Self::grow_cluster(span, interval, endpoints, n_iv);
    }

    #[must_use]
    pub fn segment_unchecked(&self) -> Self {
        let mut segments = Vec::with_capacity(self.len());
        let mut endpoints = Vec::with_capacity(self.len());
        let mut span = I::from(&self.records()[0]);
        let mut n_iv = 0;

        for interval in self.records() {
            // Case where intervals are part of the same span
            if span.overlaps(interval) || span.borders(interval) {
                Self::grow_cluster(&mut span, interval, &mut endpoints, &mut n_iv);
            // Case where intervals are not part of the same span
            } else {
                // Segment the cluster
                Self::segment_cluster(interval, &mut endpoints, n_iv, &mut segments);
                // Initialize a new cluster
                Self::init_cluster(&mut span, interval, &mut endpoints, &mut n_iv);
            }
        }

        // Process any remainder members
        if !endpoints.is_empty() {
            Self::segment_cluster(&span, &mut endpoints, n_iv, &mut segments);
        }

        // Create a IntervalContainer with the segmented intervals
        Self::from_sorted_unchecked(segments)
    }

    pub fn segment(&self) -> Result<Self, SetError> {
        if self.is_sorted() {
            Ok(self.segment_unchecked())
        } else {
            Err(SetError::UnsortedSet)
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{bed3, Bed3, Coordinates, Overlap};

    fn validate_segments(observed: &[Bed3<i32>], expected: &[Bed3<i32>]) {
        println!("Expected:");
        for exp in expected {
            println!("{exp:?}");
        }

        println!("Observed:");
        for obs in observed {
            println!("{obs:?}");
        }

        assert_eq!(observed.len(), expected.len());
        for (obs, exp) in observed.iter().zip(expected.iter()) {
            assert_eq!(obs.chr(), exp.chr());
            assert_eq!(obs.start(), exp.start());
            assert_eq!(obs.end(), exp.end());
        }

        let n_obs = observed.len();
        for idx in 0..n_obs - 1 {
            let a = &observed[idx];
            let b = &observed[idx + 1];
            let pred = a.borders(b) || !a.overlaps(b);
            assert!(pred);
        }
    }

    #[test]
    fn segment_unsorted_container() {
        let intervals = vec![bed3![0, 10, 50], bed3![0, 30, 70], bed3![0, 20, 60]];
        let set = IntervalContainer::from_iter(intervals);
        let segments = set.segment();
        assert!(segments.is_err());
    }

    /// Container:
    /// (a)    i----j
    /// (b)      k----l
    /// (c)        m----n
    /// ===============================
    /// (1)    i-k
    /// (2)      k-m
    /// (3)        m-j
    /// (4)          j-l
    /// (5)            l-n
    #[test]
    fn segment_container() {
        let intervals = vec![bed3![0, 10, 50], bed3![0, 20, 60], bed3![0, 30, 70]];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 20],
            bed3![0, 20, 30],
            bed3![0, 30, 50],
            bed3![0, 50, 60],
            bed3![0, 60, 70],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i----j
    /// (b)      k----l
    /// (c)        m----n
    /// (d)                  o----p
    /// (e)                    q----r
    /// ===============================
    /// (1)    i-k
    /// (2)      k-m
    /// (3)        m-j
    /// (4)          j-l
    /// (5)            l-n
    /// (6)                  o-q
    /// (7)                    q--p
    /// (8)                       p-r
    #[test]
    fn segment_container_clustered() {
        let intervals = vec![
            bed3![0, 10, 50],
            bed3![0, 20, 60],
            bed3![0, 30, 70],
            bed3![0, 80, 90],
            bed3![0, 85, 95],
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 20],
            bed3![0, 20, 30],
            bed3![0, 30, 50],
            bed3![0, 50, 60],
            bed3![0, 60, 70],
            bed3![0, 80, 85],
            bed3![0, 85, 90],
            bed3![0, 90, 95],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i----j
    /// (b)      k----l
    /// (c)        m----n
    /// (d)                  o----p
    /// (e)                    q----r
    /// (f)                            s----t
    /// ===============================
    /// (1)    i-k
    /// (2)      k-m
    /// (3)        m-j
    /// (4)          j-l
    /// (5)            l-n
    /// (6)                  o-q
    /// (7)                    q--p
    /// (8)                       p-r
    /// (9)                            s----t
    #[test]
    fn segment_container_clustered_with_single() {
        let intervals = vec![
            bed3![0, 10, 50],
            bed3![0, 20, 60],
            bed3![0, 30, 70],
            bed3![0, 80, 90],
            bed3![0, 85, 95],
            bed3![0, 100, 110],
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 20],
            bed3![0, 20, 30],
            bed3![0, 30, 50],
            bed3![0, 50, 60],
            bed3![0, 60, 70],
            bed3![0, 80, 85],
            bed3![0, 85, 90],
            bed3![0, 90, 95],
            bed3![0, 100, 110],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i------------------j
    /// (b)      k----l
    /// (c)        m----n
    /// ===============================
    /// (1)    i-k
    /// (2)      k-m
    /// (3)        m--l
    /// (4)           l-n
    /// (5)             n---------j
    #[test]
    fn segment_container_spanned() {
        let intervals = vec![bed3![0, 10, 500], bed3![0, 20, 60], bed3![0, 30, 70]];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 20],
            bed3![0, 20, 30],
            bed3![0, 30, 60],
            bed3![0, 60, 70],
            bed3![0, 70, 500],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i-----j
    /// (b)      k---------l
    /// (c)        m----n
    /// ===============================
    /// (1)    i-k
    /// (2)      k-m
    /// (3)        m-j
    /// (4)          j--n
    /// (5)             n--l
    #[test]
    fn segment_container_spanned_inner() {
        let intervals = vec![bed3![0, 10, 50], bed3![0, 20, 500], bed3![0, 30, 70]];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 20],
            bed3![0, 20, 30],
            bed3![0, 30, 50],
            bed3![0, 50, 70],
            bed3![0, 70, 500],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
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
    /// (1)    i-k
    /// (2)      k-m
    /// (3)        m--j
    /// (4)           j----l
    /// (5)                l-n
    /// (6)                      o---p
    #[test]
    fn segment_container_duplicate_intervals() {
        let intervals = vec![
            bed3![0, 10, 50],
            bed3![0, 10, 50],
            bed3![0, 30, 70],
            bed3![0, 30, 70],
            bed3![0, 50, 90],
            bed3![0, 50, 90],
            bed3![0, 100, 120],
            bed3![0, 100, 120],
        ];
        let set = IntervalContainer::from_unsorted(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 30],
            bed3![0, 30, 50],
            bed3![0, 50, 70],
            bed3![0, 70, 90],
            bed3![0, 100, 120],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i-----j
    /// (a)    i---------l
    /// (b)      k---------m
    /// (c)        n----------p
    /// ===============================
    /// (1)    i-k
    /// (2)      k-n
    /// (3)        n--j
    /// (4)           j--l
    /// (5)              l-m
    /// (6)                m--p
    #[test]
    fn segment_container_duplicate_start_sites() {
        let intervals = vec![
            bed3![0, 10, 50],
            bed3![0, 10, 70],
            bed3![0, 30, 90],
            bed3![0, 50, 110],
        ];
        let set = IntervalContainer::from_unsorted(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 30],
            bed3![0, 30, 50],
            bed3![0, 50, 70],
            bed3![0, 70, 90],
            bed3![0, 90, 110],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i-------j
    /// (b)      k---------m
    /// (c)        n-------m
    /// (d)           o----------p
    /// ===============================
    /// (1)    i-k
    /// (2)      k-n
    /// (3)        n-o
    /// (4)          oj
    /// (5)           j---m
    /// (6)               m-----p
    #[test]
    fn segment_container_duplicate_end_sites() {
        let intervals = vec![
            bed3![0, 10, 50],
            bed3![0, 20, 60],
            bed3![0, 30, 60],
            bed3![0, 40, 110],
        ];
        let set = IntervalContainer::from_unsorted(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 10, 20],
            bed3![0, 20, 30],
            bed3![0, 30, 40],
            bed3![0, 40, 50],
            bed3![0, 50, 60],
            bed3![0, 60, 110],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i---------------------------j
    /// (b)      k---------m
    /// (c)                     n-------o
    /// ========================================
    /// (1)    i-k
    /// (2)      k---------m
    /// (3)                m----n
    /// (4)                     n-------o
    /// (5)                             o--j
    #[test]
    fn segment_container_spanned_internal_segments() {
        let intervals = vec![bed3![0, 100, 200], bed3![0, 110, 150], bed3![0, 160, 180]];
        let set = IntervalContainer::from_unsorted(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 100, 110],
            bed3![0, 110, 150],
            bed3![0, 150, 160],
            bed3![0, 160, 180],
            bed3![0, 180, 200],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }

    /// Container:
    /// (a)    i---------------------------j
    /// (b)      k---------m
    /// (c)                     n-------o
    /// (d)                                    p--q
    /// ==============================================
    /// (1)    i-k
    /// (2)      k---------m
    /// (3)                m----n
    /// (4)                     n-------o
    /// (5)                             o--j
    /// (6)                                    p--q
    #[test]
    fn segment_container_spanned_internal_segments_with_clusters() {
        let intervals = vec![
            bed3![0, 100, 200],
            bed3![0, 110, 150],
            bed3![0, 160, 180],
            bed3![0, 250, 270],
        ];
        let set = IntervalContainer::from_unsorted(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            bed3![0, 100, 110],
            bed3![0, 110, 150],
            bed3![0, 150, 160],
            bed3![0, 160, 180],
            bed3![0, 180, 200],
            bed3![0, 250, 270],
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }
}
