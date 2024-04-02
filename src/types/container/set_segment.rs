// use super::Container;
use crate::{
    traits::{ChromBounds, IntervalBounds, SetError, ValueBounds},
    IntervalContainer,
};

impl<I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    fn grow_cluster(span: &mut I, iv: &I, starts: &mut Vec<T>, ends: &mut Vec<T>) {
        // Update the span
        let new_min = span.start().min(iv.start());
        let new_max = span.end().max(iv.end());
        span.update_start(&new_min);
        span.update_end(&new_max);

        // store the start and end of current interval
        starts.push(iv.start());
        ends.push(iv.end());
    }

    fn process_starts(reference: &I, starts: &[T], segments: &mut Vec<I>) {
        for idx in 0..starts.len() - 1 {
            let a = starts[idx];
            let b = starts[idx + 1];
            let mut seg = I::from(reference);
            seg.update_start(&a);
            seg.update_end(&b);
            segments.push(seg);
        }
    }

    fn process_center(reference: &I, starts: &[T], ends: &[T], segments: &mut Vec<I>) {
        let a = &starts[starts.len() - 1];
        let b = &ends[0];
        let mut inner_segment = I::from(reference);
        inner_segment.update_start(a);
        inner_segment.update_end(b);
        segments.push(inner_segment);
    }

    fn process_ends(reference: &I, ends: &[T], segments: &mut Vec<I>) {
        for idx in 0..ends.len() - 1 {
            let a = ends[idx];
            let b = ends[idx + 1];
            let mut seg = I::from(reference);
            seg.update_start(&a);
            seg.update_end(&b);
            segments.push(seg);
        }
    }

    fn segment_cluster(reference: &I, starts: &mut [T], ends: &mut [T], segments: &mut Vec<I>) {
        starts.sort_unstable();
        ends.sort_unstable();
        Self::process_starts(reference, starts, segments);
        Self::process_center(reference, starts, ends, segments);
        Self::process_ends(reference, ends, segments);
    }

    /// Resets the cluster with the new interval
    fn init_cluster(span: &mut I, interval: &I, starts: &mut Vec<T>, ends: &mut Vec<T>) {
        // Reset the span with the new interval
        span.update_all_from(interval);

        // With the new span, reset the starts and ends
        starts.clear();
        ends.clear();

        // store the start and end of current interval
        Self::grow_cluster(span, interval, starts, ends);
    }

    #[must_use]
    pub fn segment_unchecked(&self) -> Self {
        let mut segments = Vec::with_capacity(self.len());
        let mut starts = Vec::with_capacity(self.len());
        let mut ends = Vec::with_capacity(self.len());
        let mut span = I::from(&self.records()[0]);

        for interval in self.records() {
            // Case where intervals are part of the same span
            if span.overlaps(interval) || span.borders(interval) {
                Self::grow_cluster(&mut span, interval, &mut starts, &mut ends);
            // Case where intervals are not part of the same span
            } else {
                // Segment the cluster
                Self::segment_cluster(interval, &mut starts, &mut ends, &mut segments);
                // Initialize a new cluster
                Self::init_cluster(&mut span, interval, &mut starts, &mut ends);
            }
        }

        // Process any remainder members
        if !starts.is_empty() {
            Self::segment_cluster(&span, &mut starts, &mut ends, &mut segments);
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
    use crate::{Bed3, Coordinates, Overlap};

    fn validate_segments<T: ValueBounds>(observed: &[Bed3<i32, T>], expected: &[Bed3<i32, T>]) {
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
        let intervals = vec![
            Bed3::new(0, 10, 50),
            Bed3::new(0, 30, 70),
            Bed3::new(0, 20, 60),
        ];
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
        let intervals = vec![
            Bed3::new(0, 10, 50),
            Bed3::new(0, 20, 60),
            Bed3::new(0, 30, 70),
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            Bed3::new(0, 10, 20),
            Bed3::new(0, 20, 30),
            Bed3::new(0, 30, 50),
            Bed3::new(0, 50, 60),
            Bed3::new(0, 60, 70),
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
            Bed3::new(0, 10, 50),
            Bed3::new(0, 20, 60),
            Bed3::new(0, 30, 70),
            Bed3::new(0, 80, 90),
            Bed3::new(0, 85, 95),
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            Bed3::new(0, 10, 20),
            Bed3::new(0, 20, 30),
            Bed3::new(0, 30, 50),
            Bed3::new(0, 50, 60),
            Bed3::new(0, 60, 70),
            Bed3::new(0, 80, 85),
            Bed3::new(0, 85, 90),
            Bed3::new(0, 90, 95),
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
            Bed3::new(0, 10, 50),
            Bed3::new(0, 20, 60),
            Bed3::new(0, 30, 70),
            Bed3::new(0, 80, 90),
            Bed3::new(0, 85, 95),
            Bed3::new(0, 100, 110),
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            Bed3::new(0, 10, 20),
            Bed3::new(0, 20, 30),
            Bed3::new(0, 30, 50),
            Bed3::new(0, 50, 60),
            Bed3::new(0, 60, 70),
            Bed3::new(0, 80, 85),
            Bed3::new(0, 85, 90),
            Bed3::new(0, 90, 95),
            Bed3::new(0, 100, 110),
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
        let intervals = vec![
            Bed3::new(0, 10, 500),
            Bed3::new(0, 20, 60),
            Bed3::new(0, 30, 70),
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            Bed3::new(0, 10, 20),
            Bed3::new(0, 20, 30),
            Bed3::new(0, 30, 60),
            Bed3::new(0, 60, 70),
            Bed3::new(0, 70, 500),
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
        let intervals = vec![
            Bed3::new(0, 10, 50),
            Bed3::new(0, 20, 500),
            Bed3::new(0, 30, 70),
        ];
        let set = IntervalContainer::from_sorted_unchecked(intervals);
        let segments = set.segment().unwrap();
        let expected = vec![
            Bed3::new(0, 10, 20),
            Bed3::new(0, 20, 30),
            Bed3::new(0, 30, 50),
            Bed3::new(0, 50, 70),
            Bed3::new(0, 70, 500),
        ];
        let observed = segments.records();
        validate_segments(observed, &expected);
    }
}
