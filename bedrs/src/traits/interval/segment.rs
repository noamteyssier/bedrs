use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Coordinates, Intersect, Overlap, Subtract,
};

pub trait Segment<C, T>: Coordinates<C, T> + Overlap<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    #[must_use]
    fn build_self<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let mut sub = Self::from(other);
        sub.update_all(self.chr(), &self.start(), &self.end());
        sub
    }
    #[must_use]
    fn build_other<I: Coordinates<C, T>>(&self, other: &I) -> Self {
        let mut sub = Self::from(other);
        sub.update_all(other.chr(), &other.start(), &other.end());
        sub
    }

    /// Insert the left-hand side interval segment (i.e. the left-hand subtraction) into
    /// the segments vector
    fn insert_lhs<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        let sub = self.subtract(other).unwrap();
        segments.extend(sub);
    }

    /// Insert the central segment into the segments vector
    /// (i.e. the intersection of the pairs)
    fn insert_center<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        if let Some(ix) = other.intersect(self) {
            segments.push(ix);
        }
    }

    /// Insert the right-hand side interval segment (i.e. the right-hand subtraction) into
    /// the segments vector
    fn insert_rhs<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        let sub = other.subtract(self).unwrap();
        for s in sub {
            segments.push(self.build_other(&s));
        }
    }

    /// Insert the contained interval segment into the segments vector
    /// first insert the left-hand subtraction, then the center, then the right-hand subtraction
    fn insert_internal_contained<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        let sub = self.subtract(other).unwrap();
        segments.push(self.build_other(&sub[0]));
        self.insert_center(other, segments);
        segments.push(self.build_other(&sub[1]));
    }

    /// Insert the contained interval segment into the segments vector
    /// first insert the left-hand subtraction, then the center, then the right-hand subtraction
    fn insert_external_contained<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        let sub = other.subtract(self).unwrap();
        segments.push(self.build_other(&sub[0]));
        self.insert_center(other, segments);
        segments.push(self.build_other(&sub[1]));
    }

    /// Insert the left-hand overlap into the segments vector
    /// first insert the left-hand subtraction, then the center, then the right-hand subtraction
    fn insert_lh_overlap<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        self.insert_lhs(other, segments);
        self.insert_center(other, segments);
        self.insert_rhs(other, segments);
    }

    /// Insert the right-hand overlap into the segments vector
    /// first insert the right-hand subtraction, then the center, then the left-hand subtraction
    fn insert_rh_overlap<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        self.insert_rhs(other, segments);
        self.insert_center(other, segments);
        self.insert_lhs(other, segments);
    }

    /// Insert the overlap into the segments vector but checks which side the overlap is on
    /// to ensure sorting
    fn run_overlap<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        if self.lt(other) {
            self.insert_lh_overlap(other, segments);
        } else {
            self.insert_rh_overlap(other, segments);
        }
    }

    /// Insert the unaltered input interval pairs into the segments vector
    fn insert_input<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        segments.push(self.build_self(other));
        segments.push(self.build_other(other));
    }

    /// Insert the interval into the segments vector
    fn insert_self<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        segments.push(self.build_self(other));
    }

    /// Handles the case where the self interval contains the other interval
    fn run_internal_containment<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        if other.starts(self) {
            self.insert_center(other, segments);
            self.insert_lhs(other, segments);
        } else if other.ends(self) {
            self.insert_lhs(other, segments);
            self.insert_center(other, segments);
        } else {
            self.insert_internal_contained(other, segments);
        }
    }

    /// Handles the case where the other interval contains the self interval
    fn run_external_containment<I: Coordinates<C, T>>(&self, other: &I, segments: &mut Vec<Self>) {
        if self.starts(other) {
            self.insert_center(other, segments);
            self.insert_rhs(other, segments);
        } else if self.ends(other) {
            self.insert_rhs(other, segments);
            self.insert_center(other, segments);
        } else {
            self.insert_external_contained(other, segments);
        }
    }

    #[must_use]
    fn segment<I: IntervalBounds<C, T>>(&self, other: &I) -> Vec<Self> {
        let mut segments = Vec::new();
        if self.equals(other) {
            self.insert_self(other, &mut segments);
        } else if self.contains(other) {
            self.run_internal_containment(other, &mut segments);
        } else if other.contains(self) {
            self.run_external_containment(other, &mut segments);
        } else if self.overlaps(other) {
            self.run_overlap(other, &mut segments);
        } else {
            self.insert_input(other, &mut segments);
        }
        segments
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::Bed3;

    fn validate_segments<T: ValueBounds>(observed: &[Bed3<i32, T>], expected: &[Bed3<i32, T>]) {
        assert_eq!(observed.len(), expected.len());

        println!("Expected:");
        for exp in expected {
            println!("{exp:?}");
        }

        println!("Observed:");
        for obs in observed {
            println!("{obs:?}");
        }
        for (obs, exp) in observed.iter().zip(expected.iter()) {
            assert_eq!(obs.chr(), exp.chr());
            assert_eq!(obs.start(), exp.start());
            assert_eq!(obs.end(), exp.end());
        }
    }

    /// a:    x--------y
    /// b:                  w--------z
    /// ==========================
    /// 1:    x--------y
    /// 2:                  w--------z
    #[test]
    fn non_overlapping() {
        let iv1 = Bed3::new(1, 20, 30);
        let iv2 = Bed3::new(1, 40, 50);
        let expected = vec![Bed3::new(1, 20, 30), Bed3::new(1, 40, 50)];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:    x--------y
    /// b:    x--------y
    /// ==========================
    /// 1:    x--------y
    #[test]
    fn segments_equal() {
        let iv1 = Bed3::new(1, 20, 30);
        let iv2 = Bed3::new(1, 20, 30);
        let expected = vec![Bed3::new(1, 20, 30)];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:        x--------y
    /// b:    w--------z
    /// ==========================
    /// 1:    w---x
    /// 2:        x----z
    /// 3:             z---y
    #[test]
    fn segments_overlap_left() {
        let iv1 = Bed3::new(1, 25, 35);
        let iv2 = Bed3::new(1, 20, 30);
        let expected = vec![
            Bed3::new(1, 20, 25),
            Bed3::new(1, 25, 30),
            Bed3::new(1, 30, 35),
        ];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:    x--------y
    /// b:        w--------z
    /// ==========================
    /// 1:    x---w
    /// 2:        w----y
    /// 3:             y---z
    #[test]
    fn segments_overlap_right() {
        let iv1 = Bed3::new(1, 20, 30);
        let iv2 = Bed3::new(1, 25, 35);
        let expected = vec![
            Bed3::new(1, 20, 25),
            Bed3::new(1, 25, 30),
            Bed3::new(1, 30, 35),
        ];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:    x--------y
    /// b:      w----z
    /// ==========================
    /// 1:    x-w
    /// 2:      w----z
    /// 3:           z-y
    #[test]
    fn segments_internal_containment() {
        let iv1 = Bed3::new(1, 20, 40);
        let iv2 = Bed3::new(1, 25, 35);
        let expected = vec![
            Bed3::new(1, 20, 25),
            Bed3::new(1, 25, 35),
            Bed3::new(1, 35, 40),
        ];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:    x--------y
    /// b:    x----z
    /// ==========================
    /// 1:    x----z
    /// 2:         z---y
    #[test]
    fn segments_internal_starts() {
        let iv1 = Bed3::new(1, 20, 40);
        let iv2 = Bed3::new(1, 20, 30);
        let expected = vec![Bed3::new(1, 20, 30), Bed3::new(1, 30, 40)];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:    x--------y
    /// b:        w----y
    /// ==========================
    /// 1:    x---w
    /// 2:        w----y
    #[test]
    fn segments_internal_ends() {
        let iv1 = Bed3::new(1, 20, 40);
        let iv2 = Bed3::new(1, 30, 40);
        let expected = vec![Bed3::new(1, 20, 30), Bed3::new(1, 30, 40)];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:      w----z
    /// b:    x--------y
    /// ==========================
    /// 1:    x-w
    /// 2:      w----z
    /// 3:           z-y
    #[test]
    fn segments_external_containment() {
        let iv1 = Bed3::new(1, 25, 35);
        let iv2 = Bed3::new(1, 20, 40);
        let expected = vec![
            Bed3::new(1, 20, 25),
            Bed3::new(1, 25, 35),
            Bed3::new(1, 35, 40),
        ];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:    x----z
    /// b:    x--------y
    /// ==========================
    /// 1:    x----z
    /// 2:         z---y
    #[test]
    fn segments_external_starts() {
        let iv1 = Bed3::new(1, 20, 30);
        let iv2 = Bed3::new(1, 20, 40);
        let expected = vec![Bed3::new(1, 20, 30), Bed3::new(1, 30, 40)];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }

    /// a:        w----y
    /// b:    x--------y
    /// ==========================
    /// 1:    x---w
    /// 2:        w----y
    #[test]
    fn segments_external_ends() {
        let iv1 = Bed3::new(1, 30, 40);
        let iv2 = Bed3::new(1, 20, 40);
        let expected = vec![Bed3::new(1, 20, 30), Bed3::new(1, 30, 40)];
        let observed = iv1.segment(&iv2);
        validate_segments(&observed, &expected);
    }
}
