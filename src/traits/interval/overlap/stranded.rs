use crate::traits::{ChromBounds, Coordinates, Overlap, ValueBounds};

pub trait StrandedOverlap<C, T>: Coordinates<C, T>
where
    Self: Sized,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Returns true if the current interval overlaps the other
    /// and both intervals are on the same chromosome and strand.
    ///
    /// Considers all three of:
    /// 1. The chromosome
    /// 2. The strand
    /// 3. The interval overlap
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)       |-------->
    ///
    /// or
    ///
    /// (Self)        <--------|
    /// (Other)   <--------|
    /// ```
    fn stranded_overlaps<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.overlaps(other)
    }

    /// Returns true if the current interval overlaps the other by at least `bases`
    /// and both intervals are on the same chromosome and strand.
    ///
    /// Considers all three of:
    /// 1. The chromosome
    /// 2. The strand
    /// 3. The interval overlap
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)       |-------->
    /// ```
    /// or
    /// ```text
    /// (Self)        <--------
    /// (Other)   <--------
    /// ```
    fn stranded_overlaps_by<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.stranded_overlap_size(other)
            .map_or(false, |n| n >= bases)
    }
    /// Returns true if the current interval overlaps the other by exactly `bases`
    /// and both intervals are on the same chromosome and strand.
    fn stranded_overlaps_by_exactly<I: Coordinates<C, T>>(&self, other: &I, bases: T) -> bool {
        self.stranded_overlap_size(other)
            .map_or(false, |n| n == bases)
    }
    /// Returns the size of the overlap between the current interval and the other
    /// if the intervals are on the same chromosome and strand.
    fn stranded_overlap_size<I: Coordinates<C, T>>(&self, other: &I) -> Option<T> {
        if self.bounded_strand(other) {
            self.overlap_size(other)
        } else {
            None
        }
    }
    /// Returns true if the current interval starts the other and
    /// both intervals are on the same strand
    /// ```text
    /// (Self)    |-------->
    /// (Other)   |----------------->
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 400, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 100, 400, Strand::Reverse);
    /// assert!(interval1.stranded_starts(&interval2));
    /// assert!(!interval1.stranded_starts(&interval3));
    /// ```
    fn stranded_starts<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.starts(other)
    }

    /// Returns true if the current interval ends the other and
    /// both intervals are on the same strand
    /// ```text
    /// (Self)             |-------->
    /// (Other)   |----------------->
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 300, 400, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 400, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 100, 400, Strand::Reverse);
    /// assert!(interval1.stranded_ends(&interval2));
    /// assert!(!interval1.stranded_ends(&interval3));
    /// ```
    fn stranded_ends<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.ends(other)
    }
    /// Returns true if the current interval equals the other and they are on the same strand
    /// considers both the interval overlap and the chromosome.
    /// ```text
    /// (Self)    |-------->
    /// (Other)   |-------->
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 100, 200, Strand::Reverse);
    /// assert!(interval1.stranded_equals(&interval2));
    /// assert!(!interval1.stranded_equals(&interval3));
    /// ```
    fn stranded_equals<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.equals(other)
    }
    /// Returns true if the current interval is during the other and
    /// both intervals are on the same strand -
    /// ```text
    /// (Self)      |---->
    /// (Other)   |-------->
    /// ```
    /// # Example
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    /// let interval1 = StrandedBed3::new(1, 150, 160, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 100, 200, Strand::Reverse);
    /// assert!(interval1.stranded_during(&interval2));
    /// assert!(!interval1.stranded_during(&interval3));
    /// ```
    fn stranded_during<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.during(other)
    }
    /// Returns true if the current interval contains the other and
    /// both intervals are on the same strand -
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)     |---->
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    ///
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 150, 160, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 150, 160, Strand::Reverse);
    ///
    /// assert!(interval1.stranded_contains(&interval2));
    /// assert!(!interval1.stranded_contains(&interval3));
    /// ```
    fn stranded_contains<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.contains(other)
    }
    /// Returns true if the current interval is contained by the other and
    /// both intervals are on the same strand -
    ///
    /// ```text
    /// (Self)      |---->
    /// (Other)   |-------->
    ///
    /// or
    /// (Self)      <----|
    /// (Other)   <--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    ///
    /// let interval1 = StrandedBed3::new(1, 150, 160, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 100, 200, Strand::Reverse);
    ///
    /// assert!(interval1.stranded_contained_by(&interval2));
    /// assert!(!interval1.stranded_contained_by(&interval3));
    /// ```
    fn stranded_contained_by<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        other.stranded_contains(self)
    }

    /// Returns true if the current interval borders the other and
    /// both intervals are on the same strand -
    ///
    /// ```text
    /// (Self)    |-------->
    /// (Other)            |-------->
    ///
    /// or
    /// (Self)             <--------|
    /// (Other)   <--------|
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::{StrandedBed3, Strand, Coordinates, Overlap, StrandedOverlap};
    ///
    /// let interval1 = StrandedBed3::new(1, 100, 200, Strand::Forward);
    /// let interval2 = StrandedBed3::new(1, 200, 300, Strand::Forward);
    /// let interval3 = StrandedBed3::new(1, 200, 300, Strand::Reverse);
    ///
    /// assert!(interval1.stranded_borders(&interval2));
    /// assert!(!interval1.stranded_borders(&interval3));
    /// ```
    fn stranded_borders<I: Coordinates<C, T>>(&self, other: &I) -> bool {
        self.bounded_strand(other) && self.interval_borders(other)
    }
}
