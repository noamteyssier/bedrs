use crate::{traits::ChromBounds, Coordinates, Overlap};

/// Calculates the intersection between two coordinates.
pub trait Intersect<C>: Coordinates<C> + Overlap<C>
where
    C: ChromBounds,
{
    fn build_intersection_interval<I: Coordinates<C>>(&self, other: &I) -> I {
        let chr = self.chr();
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        let mut interval = I::from(other);
        interval.update_all(chr, &start, &end);
        interval
    }

    /// Calculates the intersection between two coordinates if they overlap.
    ///
    /// ```
    /// use bedrs::{Coordinates, Intersect, BaseInterval};
    ///
    /// let a = BaseInterval::new(10, 20);
    /// let b = BaseInterval::new(15, 25);
    /// let ix = a.intersect(&b).unwrap();
    ///
    /// assert_eq!(ix.start(), 15);
    /// assert_eq!(ix.end(), 20);
    /// ```
    fn intersect<I: Coordinates<C>>(&self, other: &I) -> Option<I> {
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
    /// use bedrs::{Coordinates, Intersect, StrandedBed3, Strand};
    ///
    /// let a = StrandedBed3::new(1, 10, 20, Strand::Forward);
    /// let b = StrandedBed3::new(1, 15, 25, Strand::Forward);
    /// let c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
    ///
    /// let ix = a.stranded_intersect(&b).unwrap();
    /// assert_eq!(ix.start(), 15);
    /// assert_eq!(ix.end(), 20);
    /// assert_eq!(ix.strand(), Some(Strand::Forward));
    ///
    /// assert!(a.stranded_intersect(&c).is_none());
    /// ```
    fn stranded_intersect<I: Coordinates<C>>(&self, other: &I) -> Option<I> {
        if self.stranded_overlaps(other) {
            let ix = self.build_intersection_interval(other);
            Some(ix)
        } else {
            None
        }
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod testing {
    use super::Intersect;
    use crate::{bed3, BaseInterval, Coordinates, Strand, StrandedBed3};

    #[test]
    ///       x-------y
    ///    i------j
    /// =====================
    ///       x---j
    fn intersection_case_a() {
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(15, 25);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 25);
    }

    #[test]
    fn intersection_case_a_genomic() {
        let a = bed3![1, 20, 30];
        let b = bed3![1, 15, 25];
        let c = bed3![2, 15, 25];
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 25);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_a_stranded() {
        let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
        let b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let d = StrandedBed3::new(2, 15, 25, Strand::Forward);
        let e = StrandedBed3::new(2, 15, 25, Strand::Reverse);
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
        let a = BaseInterval::new(20, 30);
        let b = BaseInterval::new(25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 30);
    }

    #[test]
    fn intersection_case_b_genomic() {
        let a = bed3![1, 20, 30];
        let b = bed3![1, 25, 35];
        let c = bed3![2, 25, 35];
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 30);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_b_stranded() {
        let a = StrandedBed3::new(1, 20, 30, Strand::Forward);
        let b = StrandedBed3::new(1, 25, 35, Strand::Forward);
        let c = StrandedBed3::new(1, 25, 35, Strand::Reverse);
        let d = StrandedBed3::new(2, 25, 35, Strand::Forward);
        let e = StrandedBed3::new(2, 25, 35, Strand::Reverse);
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
        let a = BaseInterval::new(20, 40);
        let b = BaseInterval::new(25, 35);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
    }

    #[test]
    fn intersection_case_c_genomic() {
        let a = bed3![1, 20, 40];
        let b = bed3![1, 25, 35];
        let c = bed3![2, 25, 35];
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_c_stranded() {
        let a = StrandedBed3::new(1, 20, 40, Strand::Forward);
        let b = StrandedBed3::new(1, 25, 35, Strand::Forward);
        let c = StrandedBed3::new(1, 25, 35, Strand::Reverse);
        let d = StrandedBed3::new(2, 25, 35, Strand::Forward);
        let e = StrandedBed3::new(2, 25, 35, Strand::Reverse);
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
        let a = BaseInterval::new(25, 35);
        let b = BaseInterval::new(20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
    }

    #[test]
    fn intersection_case_d_genomic() {
        let a = bed3![1, 25, 35];
        let b = bed3![1, 20, 40];
        let c = bed3![2, 20, 40];
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 25);
        assert_eq!(ix.end(), 35);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_d_stranded() {
        let a = StrandedBed3::new(1, 25, 35, Strand::Forward);
        let b = StrandedBed3::new(1, 20, 40, Strand::Forward);
        let c = StrandedBed3::new(1, 20, 40, Strand::Reverse);
        let d = StrandedBed3::new(2, 20, 40, Strand::Forward);
        let e = StrandedBed3::new(2, 20, 40, Strand::Reverse);
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
        let a = BaseInterval::new(20, 40);
        let b = BaseInterval::new(20, 40);
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 40);
    }

    #[test]
    fn intersection_case_e_genomic() {
        let a = bed3![1, 20, 40];
        let b = bed3![1, 20, 40];
        let c = bed3![2, 20, 40];
        let ix = a.intersect(&b).unwrap();
        assert_eq!(ix.start(), 20);
        assert_eq!(ix.end(), 40);
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_e_stranded() {
        let a = StrandedBed3::new(1, 20, 40, Strand::Forward);
        let b = StrandedBed3::new(1, 20, 40, Strand::Forward);
        let c = StrandedBed3::new(1, 20, 40, Strand::Reverse);
        let d = StrandedBed3::new(2, 20, 40, Strand::Forward);
        let e = StrandedBed3::new(2, 20, 40, Strand::Reverse);
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
        let a = BaseInterval::new(10, 15);
        let b = BaseInterval::new(20, 25);
        let ix = a.intersect(&b);
        assert!(ix.is_none());
    }

    #[test]
    fn intersection_case_none_genomic() {
        let a = bed3![1, 10, 15];
        let b = bed3![1, 20, 25];
        let c = bed3![2, 20, 25];
        assert!(a.intersect(&b).is_none());
        assert!(a.intersect(&c).is_none());
    }

    #[test]
    fn intersection_case_none_stranded() {
        let a = StrandedBed3::new(1, 10, 15, Strand::Forward);
        let b = StrandedBed3::new(1, 20, 25, Strand::Forward);
        let c = StrandedBed3::new(1, 20, 25, Strand::Reverse);
        let d = StrandedBed3::new(2, 20, 25, Strand::Forward);
        let e = StrandedBed3::new(2, 20, 25, Strand::Reverse);
        assert!(a.stranded_intersect(&b).is_none());
        assert!(a.stranded_intersect(&c).is_none());
        assert!(a.stranded_intersect(&d).is_none());
        assert!(a.stranded_intersect(&e).is_none());
    }
}
