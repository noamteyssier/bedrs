use crate::traits::{ChromBounds, Coordinates, Overlap, ValueBounds};

pub trait UnstrandedOverlap<C, T>: Coordinates<C, T>
where
    Self: Sized,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Returns true if the two intervals overlap, but are not on the same strand.
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)       <--------|
    ///
    /// or
    ///
    /// (Self)        <--------|
    /// (Other)   |-------->
    /// ```
    fn unstranded_overlaps<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.overlaps(other)
    }
    /// Returns true if the two intervals overlap by at least `bases`, but are not on the same strand.
    fn unstranded_overlaps_by<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        !self.bounded_strand(other) && self.overlaps_by(other, bases)
    }
    /// Returns true if the two intervals overlap by exactly `bases`, but are not on the same strand.
    fn unstranded_overlaps_by_exactly<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        !self.bounded_strand(other) && self.overlaps_by_exactly(other, bases)
    }
    /// Returns the size of the overlap between the current interval and the other
    /// if the intervals are not on the same strand.
    fn unstranded_overlap_size<I: Coordinates<C, T>>(&self, other: &I) -> Option<T> {
        if self.bounded_strand(other) {
            None
        } else {
            self.overlap_size(other)
        }
    }
    /// Returns true if the current interval starts the other, but are not on the same strand.
    /// ```text
    /// (Self)    |-------->
    /// (Other)   <-----------------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 400, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 100, 400, Strand::Forward);
    /// assert!(interval1.unstranded_starts(&interval2));
    /// assert!(!interval1.unstranded_starts(&interval3));
    /// ```
    fn unstranded_starts<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.starts(other)
    }

    /// Returns true if the current interval ends the other, but are not on the same strand.
    /// ```text
    /// (Self)             |-------->
    /// (Other)   <-----------------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 300, 400, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 400, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 100, 400, Strand::Forward);
    /// assert!(interval1.unstranded_ends(&interval2));
    /// assert!(!interval1.unstranded_ends(&interval3));
    /// ```
    fn unstranded_ends<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.ends(other)
    }
    /// Returns true if the current interval equals the other, but are not on the same strand.
    /// ```text
    /// (Self)    |-------->
    /// (Other)   <--------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 200, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// assert!(interval1.unstranded_equals(&interval2));
    /// assert!(!interval1.unstranded_equals(&interval3));
    /// ```
    fn unstranded_equals<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.equals(other)
    }
    /// Returns true if the current interval is during the other, but are not on the same strand.
    /// ```text
    /// (Self)       |-------->
    /// (Other)   <-----------------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 150, 160, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 200, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// assert!(interval1.unstranded_during(&interval2));
    /// assert!(!interval1.unstranded_during(&interval3));
    /// ```
    fn unstranded_during<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.during(other)
    }
    /// Returns true if the current interval contains the other, but are not on the same strand.
    /// ```text
    /// (Self)   |----------------->
    /// (Other)      <--------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 150, 160, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 150, 160, Strand::Forward);
    /// assert!(interval1.unstranded_contains(&interval2));
    /// assert!(!interval1.unstranded_contains(&interval3));
    /// ```
    fn unstranded_contains<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.contains(other)
    }
    /// Returns true if the current interval is contained by the other, but are not on the same strand.
    /// ```text
    /// (Self)      |-------->
    /// (Other)   <-----------------|
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 150, 160, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 200, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// assert!(interval1.unstranded_contained_by(&interval2));
    /// assert!(!interval1.unstranded_contained_by(&interval3));
    /// ```
    fn unstranded_contained_by<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.contained_by(other)
    }
    /// Returns true if the current interval is adjacent to the other, but are not on the same strand.
    /// ```text
    /// (Self)    |-------->
    /// (Other)            <--------|
    ///
    /// or
    /// (Self)             <--------|
    /// (Other)   |-------->
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, UnstrandedOverlap};
    ///
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 200, 300, Strand::Reverse);
    /// let interval3 = StrandedBed3::new(1, 200, 300, Strand::Forward);
    ///
    /// assert!(interval1.unstranded_borders(&interval2));
    /// assert!(!interval1.unstranded_borders(&interval3));
    /// ```
    fn unstranded_borders<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        !self.bounded_strand(other) && self.borders(other)
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::{Strand, StrandedBed3};

    #[test]
    fn test_opposite_overlaps() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        assert!(!iv_a.unstranded_overlaps(&iv_b));
        assert!(iv_a.unstranded_overlaps(&iv_c));
    }

    #[test]
    fn test_opposite_overlaps_by() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 170, 250, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 170, 250, Strand::Reverse);
        assert!(!iv_a.unstranded_overlaps_by(&iv_b, 50));
        assert!(iv_a.unstranded_overlaps_by(&iv_c, 50));
        assert!(!iv_a.unstranded_overlaps_by(&iv_d, 50));
        assert!(!iv_a.unstranded_overlaps_by(&iv_e, 50));
    }

    #[test]
    fn test_opposite_overlaps_by_exactly() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 170, 250, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 170, 250, Strand::Reverse);
        assert!(!iv_a.unstranded_overlaps_by_exactly(&iv_b, 50));
        assert!(iv_a.unstranded_overlaps_by_exactly(&iv_c, 50));
        assert!(!iv_a.unstranded_overlaps_by_exactly(&iv_d, 50));
        assert!(!iv_a.unstranded_overlaps_by_exactly(&iv_e, 50));
        assert!(!iv_a.unstranded_overlaps_by_exactly(&iv_d, 30));
        assert!(iv_a.unstranded_overlaps_by_exactly(&iv_e, 30));
    }

    #[test]
    fn test_opposite_overlap_size() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 170, 250, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 170, 250, Strand::Reverse);
        assert_eq!(iv_a.unstranded_overlap_size(&iv_b), None);
        assert_eq!(iv_a.unstranded_overlap_size(&iv_c), Some(50));
        assert_eq!(iv_a.unstranded_overlap_size(&iv_d), None);
        assert_eq!(iv_a.unstranded_overlap_size(&iv_e), Some(30));
    }

    #[test]
    fn test_opposite_starts() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 100, 250, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 100, 250, Strand::Reverse);
        assert!(!iv_a.unstranded_starts(&iv_b));
        assert!(!iv_a.unstranded_starts(&iv_c));
        assert!(!iv_a.unstranded_starts(&iv_d));
        assert!(iv_a.unstranded_starts(&iv_e));
    }

    #[test]
    fn test_opposite_ends() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 50, 200, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 50, 200, Strand::Reverse);
        assert!(!iv_a.unstranded_ends(&iv_b));
        assert!(!iv_a.unstranded_ends(&iv_c));
        assert!(!iv_a.unstranded_ends(&iv_d));
        assert!(iv_a.unstranded_ends(&iv_e));
    }

    #[test]
    fn test_opposite_equals() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 250, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 250, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 100, 200, Strand::Reverse);
        assert!(!iv_a.unstranded_equals(&iv_b));
        assert!(!iv_a.unstranded_equals(&iv_c));
        assert!(!iv_a.unstranded_equals(&iv_d));
        assert!(iv_a.unstranded_equals(&iv_e));
    }

    #[test]
    fn test_opposite_during() {
        let iv_a = StrandedBed3::new(1, 150, 160, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 100, 200, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 100, 160, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 100, 160, Strand::Reverse);
        let iv_f = StrandedBed3::new(1, 150, 200, Strand::Forward);
        let iv_g = StrandedBed3::new(1, 150, 200, Strand::Reverse);
        assert!(!iv_a.unstranded_during(&iv_b));
        assert!(iv_a.unstranded_during(&iv_c));
        assert!(!iv_a.unstranded_during(&iv_d));
        assert!(!iv_a.unstranded_during(&iv_e));
        assert!(!iv_a.unstranded_during(&iv_f));
        assert!(!iv_a.unstranded_during(&iv_g));
    }

    #[test]
    fn test_opposite_contains() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 150, 160, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 150, 160, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 100, 160, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 100, 160, Strand::Reverse);
        let iv_f = StrandedBed3::new(1, 150, 200, Strand::Forward);
        let iv_g = StrandedBed3::new(1, 150, 200, Strand::Reverse);
        let in_set = [iv_c, iv_e, iv_g];
        let out_set = [iv_b, iv_d, iv_f];
        for iv in in_set {
            assert!(iv_a.unstranded_contains(&iv));
        }
        for iv in out_set {
            assert!(!iv_a.unstranded_contains(&iv));
        }
    }

    #[test]
    fn test_opposite_contained_by() {
        let iv_a = StrandedBed3::new(1, 150, 160, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 100, 200, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 100, 160, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 100, 160, Strand::Reverse);
        let iv_f = StrandedBed3::new(1, 150, 200, Strand::Forward);
        let iv_g = StrandedBed3::new(1, 150, 200, Strand::Reverse);
        assert!(!iv_a.unstranded_contained_by(&iv_b));
        assert!(iv_a.unstranded_contained_by(&iv_c));
        assert!(!iv_a.unstranded_contained_by(&iv_d));
        assert!(iv_a.unstranded_contained_by(&iv_e));
        assert!(!iv_a.unstranded_contained_by(&iv_f));
        assert!(iv_a.unstranded_contained_by(&iv_g));
    }

    #[test]
    fn test_opposite_borders() {
        let iv_a = StrandedBed3::new(1, 100, 200, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 200, 300, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 200, 300, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 50, 100, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 50, 100, Strand::Reverse);
        let iv_f = StrandedBed3::new(1, 199, 300, Strand::Forward);
        let iv_g = StrandedBed3::new(1, 199, 300, Strand::Reverse);
        let iv_h = StrandedBed3::new(1, 50, 101, Strand::Forward);
        let iv_i = StrandedBed3::new(1, 50, 101, Strand::Reverse);
        let iv_j = StrandedBed3::new(1, 201, 300, Strand::Forward);
        let iv_k = StrandedBed3::new(1, 201, 300, Strand::Reverse);
        let iv_l = StrandedBed3::new(1, 50, 99, Strand::Forward);
        let iv_m = StrandedBed3::new(1, 50, 99, Strand::Reverse);
        let in_set = [iv_c, iv_e];
        let out_set = [iv_b, iv_d, iv_f, iv_g, iv_h, iv_i, iv_j, iv_k, iv_l, iv_m];
        for iv in in_set {
            assert!(iv_a.unstranded_borders(&iv));
        }
        for iv in out_set {
            assert!(!iv_a.unstranded_borders(&iv));
        }
    }
}
