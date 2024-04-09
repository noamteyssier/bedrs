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
