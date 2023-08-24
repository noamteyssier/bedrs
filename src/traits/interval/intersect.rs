use crate::{
    traits::{ChromBounds, ValueBounds},
    Coordinates, Overlap,
};

/// Calculates the intersection between two coordinates.
pub trait Intersect<C, T>: Coordinates<C, T> + Overlap<C, T>
where
    C: ChromBounds,
    T: ValueBounds,
{
    fn build_intersection_interval<I: Coordinates<C, T>>(&self, other: &I) -> I {
        let chr = self.chr();
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        let mut interval = I::from(other);
        interval.update_all(&chr, &start, &end);
        interval
    }

    /// Calculates the intersection between two coordinates if they overlap.
    ///
    /// ```
    /// use bedrs::{Coordinates, Intersect, Interval};
    ///
    /// let a = Interval::new(10, 20);
    /// let b = Interval::new(15, 25);
    /// let ix = a.intersect(&b).unwrap();
    ///
    /// assert_eq!(ix.start(), 15);
    /// assert_eq!(ix.end(), 20);
    /// ```
    fn intersect<I: Coordinates<C, T>>(&self, other: &I) -> Option<I> {
        if self.overlaps(other) {
            let ix = self.build_intersection_interval(other);
            Some(ix)
        } else {
            None
        }
    }

    /// Calculates the intersection between two coordinates if they overlap
    /// and are on the same strand.
    ///
    /// ```
    /// use bedrs::{Coordinates, Intersect, StrandedGenomicInterval, Strand};
    ///
    /// let a = StrandedGenomicInterval::new(1, 10, 20, Strand::Forward);
    /// let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
    /// let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);
    ///
    /// let ix = a.stranded_intersect(&b).unwrap();
    /// assert_eq!(ix.start(), 15);
    /// assert_eq!(ix.end(), 20);
    /// assert_eq!(ix.strand(), Some(Strand::Forward));
    ///
    /// assert!(a.stranded_intersect(&c).is_none());
    /// ```
    fn stranded_intersect<I: Coordinates<C, T>>(&self, other: &I) -> Option<I> {
        if self.stranded_overlaps(other) {
            let ix = self.build_intersection_interval(other);
            Some(ix)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod testing {
    use super::Intersect;
    use crate::{Coordinates, GenomicInterval, Interval, Strand, StrandedGenomicInterval};

    #[test]
    ///       x-------y
    ///    i------j
    /// =====================
    ///       x---j
    fn intersection_case_a() {
        let a = Interval::new(20, 30);
        let b = Interval::new(15, 25);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 25);
    }

    #[test]
    fn intersection_case_a_genomic() {
        let a = GenomicInterval::new(1, 20, 30);
        let b = GenomicInterval::new(1, 15, 25);
        let c = GenomicInterval::new(2, 15, 25);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 25);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_a_stranded() {
        let a = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 15, 25, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 15, 25, Strand::Reverse);
        let d = StrandedGenomicInterval::new(2, 15, 25, Strand::Forward);
        let e = StrandedGenomicInterval::new(2, 15, 25, Strand::Reverse);
        let ix = a.stranded_intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 25);
        assert_eq!(ix.strand(), Some(Strand::Forward));
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }

    #[test]
    ///    x-------y
    ///        i------j
    /// ==================
    ///        i---y
    fn intersection_case_b() {
        let a = Interval::new(20, 30);
        let b = Interval::new(25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 30);
    }

    #[test]
    fn intersection_case_b_genomic() {
        let a = GenomicInterval::new(1, 20, 30);
        let b = GenomicInterval::new(1, 25, 35);
        let c = GenomicInterval::new(2, 25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 30);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_b_stranded() {
        let a = StrandedGenomicInterval::new(1, 20, 30, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 25, 35, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 25, 35, Strand::Reverse);
        let d = StrandedGenomicInterval::new(2, 25, 35, Strand::Forward);
        let e = StrandedGenomicInterval::new(2, 25, 35, Strand::Reverse);
        let ix = a.stranded_intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 30);
        assert_eq!(ix.strand(), Some(Strand::Forward));
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }

    #[test]
    ///    x--------y
    ///       i--j
    /// ==================
    ///       i--j
    fn intersection_case_c() {
        let a = Interval::new(20, 40);
        let b = Interval::new(25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
    }

    #[test]
    fn intersection_case_c_genomic() {
        let a = GenomicInterval::new(1, 20, 40);
        let b = GenomicInterval::new(1, 25, 35);
        let c = GenomicInterval::new(2, 25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_c_stranded() {
        let a = StrandedGenomicInterval::new(1, 20, 40, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 25, 35, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 25, 35, Strand::Reverse);
        let d = StrandedGenomicInterval::new(2, 25, 35, Strand::Forward);
        let e = StrandedGenomicInterval::new(2, 25, 35, Strand::Reverse);
        let ix = a.stranded_intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
        assert_eq!(ix.strand(), Some(Strand::Forward));
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }

    #[test]
    ///       x--y
    ///    i--------j
    /// ==================
    ///       x--y
    fn intersection_case_d() {
        let a = Interval::new(25, 35);
        let b = Interval::new(20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
    }

    #[test]
    fn intersection_case_d_genomic() {
        let a = GenomicInterval::new(1, 25, 35);
        let b = GenomicInterval::new(1, 20, 40);
        let c = GenomicInterval::new(2, 20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_d_stranded() {
        let a = StrandedGenomicInterval::new(1, 25, 35, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 20, 40, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 20, 40, Strand::Reverse);
        let d = StrandedGenomicInterval::new(2, 20, 40, Strand::Forward);
        let e = StrandedGenomicInterval::new(2, 20, 40, Strand::Reverse);
        let ix = a.stranded_intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
        assert_eq!(ix.strand(), Some(Strand::Forward));
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }

    #[test]
    ///       x--y
    ///       i--j
    /// ==================
    ///       x--y
    fn intersection_case_e() {
        let a = Interval::new(20, 40);
        let b = Interval::new(20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 40);
    }

    #[test]
    fn intersection_case_e_genomic() {
        let a = GenomicInterval::new(1, 20, 40);
        let b = GenomicInterval::new(1, 20, 40);
        let c = GenomicInterval::new(2, 20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 40);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_e_stranded() {
        let a = StrandedGenomicInterval::new(1, 20, 40, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 20, 40, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 20, 40, Strand::Reverse);
        let d = StrandedGenomicInterval::new(2, 20, 40, Strand::Forward);
        let e = StrandedGenomicInterval::new(2, 20, 40, Strand::Reverse);
        let ix = a.stranded_intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 40);
        assert_eq!(ix.strand(), Some(Strand::Forward));
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }

    #[test]
    ///    x--y
    ///         i--j
    /// ==================
    ///       none
    fn intersection_case_none() {
        let a = Interval::new(10, 15);
        let b = Interval::new(20, 25);
        let ix = a.intersect(&b);
        assert!(ix.is_none());
    }

    #[test]
    fn intersection_case_none_genomic() {
        let a = GenomicInterval::new(1, 10, 15);
        let b = GenomicInterval::new(1, 20, 25);
        let c = GenomicInterval::new(2, 20, 25);
        assert!(a.intersect(&b).is_none());
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_none_stranded() {
        let a = StrandedGenomicInterval::new(1, 10, 15, Strand::Forward);
        let b = StrandedGenomicInterval::new(1, 20, 25, Strand::Forward);
        let c = StrandedGenomicInterval::new(1, 20, 25, Strand::Reverse);
        let d = StrandedGenomicInterval::new(2, 20, 25, Strand::Forward);
        let e = StrandedGenomicInterval::new(2, 20, 25, Strand::Reverse);
        assert!(a.stranded_intersect(&b).is_none());
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }
}
