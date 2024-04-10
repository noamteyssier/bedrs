use super::{QueryMethod, StrandMethod};
use crate::traits::{ChromBounds, IntervalBounds, SetError, ValueBounds};

#[derive(Debug, Default, Clone, Copy)]
pub struct Query<T: ValueBounds> {
    predicate: QueryMethod<T>,
    strandedness: StrandMethod,
}
impl<T> Query<T>
where
    T: ValueBounds,
{
    #[must_use]
    pub fn new(predicate: QueryMethod<T>, strandedness: StrandMethod) -> Self {
        Self {
            predicate,
            strandedness,
        }
    }
    #[must_use]
    pub fn new_predicate(predicate: QueryMethod<T>) -> Self {
        Self {
            predicate,
            strandedness: StrandMethod::default(),
        }
    }
    #[must_use]
    pub fn new_strandedness(strandedness: StrandMethod) -> Self {
        Self {
            predicate: QueryMethod::default(),
            strandedness,
        }
    }
    pub fn validate(&self) -> Result<(), SetError> {
        self.predicate.validate()
    }
    /// Determine whether a query interval overlaps a target interval
    /// using a specific overlap method
    pub fn predicate<I, Iv, C>(&self, target: &I, query: &Iv) -> bool
    where
        I: IntervalBounds<C, T>,
        Iv: IntervalBounds<C, T>,
        C: ChromBounds,
    {
        match self.strandedness {
            StrandMethod::Ignore => self.ignored_strand(target, query),
            StrandMethod::MatchStrand => self.bounded_strand(target, query),
            StrandMethod::OppositeStrand => self.unbounded_strand(target, query),
        }
    }
    fn ignored_strand<I, Iv, C>(&self, target: &I, query: &Iv) -> bool
    where
        I: IntervalBounds<C, T>,
        Iv: IntervalBounds<C, T>,
        C: ChromBounds,
    {
        match self.predicate {
            QueryMethod::Compare => target.overlaps(query),
            QueryMethod::CompareBy(val) => target.overlaps_by(query, val),
            QueryMethod::CompareExact(val) => target.overlaps_by_exactly(query, val),
            QueryMethod::CompareByQueryFraction(frac) => {
                let min_overlap = query.f_len(frac);
                target.overlaps_by(query, min_overlap)
            }
            QueryMethod::CompareByTargetFraction(frac) => {
                let min_overlap = target.f_len(frac);
                target.overlaps_by(query, min_overlap)
            }
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
                let query_min_overlap = query.f_len(f_query);
                let target_min_overlap = target.f_len(f_target);
                if let Some(ix) = target.overlap_size(query) {
                    query_min_overlap <= ix && target_min_overlap <= ix
                } else {
                    false
                }
            }
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
                let query_min_overlap = query.f_len(f_query);
                let target_min_overlap = target.f_len(f_target);
                if let Some(ix) = target.overlap_size(query) {
                    query_min_overlap <= ix || target_min_overlap <= ix
                } else {
                    false
                }
            }
        }
    }
    fn bounded_strand<I, Iv, C>(&self, target: &I, query: &Iv) -> bool
    where
        I: IntervalBounds<C, T>,
        Iv: IntervalBounds<C, T>,
        C: ChromBounds,
    {
        match self.predicate {
            QueryMethod::Compare => target.stranded_overlaps(query),
            QueryMethod::CompareBy(val) => target.stranded_overlaps_by(query, val),
            QueryMethod::CompareExact(val) => target.stranded_overlaps_by_exactly(query, val),
            QueryMethod::CompareByQueryFraction(frac) => {
                let min_overlap = query.f_len(frac);
                target.stranded_overlaps_by(query, min_overlap)
            }
            QueryMethod::CompareByTargetFraction(frac) => {
                let min_overlap = target.f_len(frac);
                target.stranded_overlaps_by(query, min_overlap)
            }
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
                let query_min_overlap = query.f_len(f_query);
                let target_min_overlap = target.f_len(f_target);
                if let Some(ix) = target.stranded_overlap_size(query) {
                    query_min_overlap <= ix && target_min_overlap <= ix
                } else {
                    false
                }
            }
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
                let query_min_overlap = query.f_len(f_query);
                let target_min_overlap = target.f_len(f_target);
                if let Some(ix) = target.stranded_overlap_size(query) {
                    query_min_overlap <= ix || target_min_overlap <= ix
                } else {
                    false
                }
            }
        }
    }
    fn unbounded_strand<I, Iv, C>(&self, target: &I, query: &Iv) -> bool
    where
        I: IntervalBounds<C, T>,
        Iv: IntervalBounds<C, T>,
        C: ChromBounds,
    {
        match self.predicate {
            QueryMethod::Compare => target.unstranded_overlaps(query),
            QueryMethod::CompareBy(val) => target.unstranded_overlaps_by(query, val),
            QueryMethod::CompareExact(val) => target.unstranded_overlaps_by_exactly(query, val),
            QueryMethod::CompareByQueryFraction(frac) => {
                let min_overlap = query.f_len(frac);
                target.unstranded_overlaps_by(query, min_overlap)
            }
            QueryMethod::CompareByTargetFraction(frac) => {
                let min_overlap = target.f_len(frac);
                target.unstranded_overlaps_by(query, min_overlap)
            }
            QueryMethod::CompareReciprocalFractionAnd(f_query, f_target) => {
                let query_min_overlap = query.f_len(f_query);
                let target_min_overlap = target.f_len(f_target);
                if let Some(ix) = target.unstranded_overlap_size(query) {
                    query_min_overlap <= ix && target_min_overlap <= ix
                } else {
                    false
                }
            }
            QueryMethod::CompareReciprocalFractionOr(f_query, f_target) => {
                let query_min_overlap = query.f_len(f_query);
                let target_min_overlap = target.f_len(f_target);
                if let Some(ix) = target.unstranded_overlap_size(query) {
                    query_min_overlap <= ix || target_min_overlap <= ix
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use crate::{Strand, StrandedBed3};

    // Compare

    #[test]
    fn strand_ignore_compare() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 30, 40, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 30, 40, Strand::Reverse);
        let query = Query::new(QueryMethod::Compare, StrandMethod::Ignore);
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 30, 40, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 30, 40, Strand::Reverse);
        let query = Query::new(QueryMethod::Compare, StrandMethod::MatchStrand);
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 30, 40, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 30, 40, Strand::Reverse);
        let query = Query::new(QueryMethod::Compare, StrandMethod::OppositeStrand);
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    // CompareBy

    #[test]
    fn strand_ignore_compare_by() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 12, 22, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 12, 22, Strand::Reverse);
        let query = Query::new(QueryMethod::CompareBy(7), StrandMethod::Ignore);
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(query.predicate(&iv_a, &iv_d));
        assert!(query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare_by() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 12, 22, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 12, 22, Strand::Reverse);
        let query = Query::new(QueryMethod::CompareBy(7), StrandMethod::MatchStrand);
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare_by() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 12, 22, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 12, 22, Strand::Reverse);
        let query = Query::new(QueryMethod::CompareBy(7), StrandMethod::OppositeStrand);
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(query.predicate(&iv_a, &iv_e));
    }

    // CompareExact

    #[test]
    fn strand_ignore_compare_exact() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 12, 22, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 12, 22, Strand::Reverse);
        let query = Query::new(QueryMethod::CompareExact(5), StrandMethod::Ignore);
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare_exact() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 12, 22, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 12, 22, Strand::Reverse);
        let query = Query::new(QueryMethod::CompareExact(5), StrandMethod::MatchStrand);
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare_exact() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 12, 22, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 12, 22, Strand::Reverse);
        let query = Query::new(QueryMethod::CompareExact(5), StrandMethod::OppositeStrand);
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    // CompareByQueryFraction

    #[test]
    fn strand_ignore_compare_by_query_fraction() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 10, 100, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareByQueryFraction(0.5),
            StrandMethod::Ignore,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare_by_query_fraction() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 10, 100, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareByQueryFraction(0.5),
            StrandMethod::MatchStrand,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare_by_query_fraction() {
        let iv_a = StrandedBed3::new(1, 10, 20, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 10, 100, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareByQueryFraction(0.5),
            StrandMethod::OppositeStrand,
        );
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    // CompareByTargetFraction

    #[test]
    fn strand_ignore_compare_by_target_fraction() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareByTargetFraction(0.5),
            StrandMethod::Ignore,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare_by_target_fraction() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareByTargetFraction(0.5),
            StrandMethod::MatchStrand,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare_by_target_fraction() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareByTargetFraction(0.5),
            StrandMethod::OppositeStrand,
        );
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    // CompareReciprocalFractionAnd

    #[test]
    fn strand_ignore_compare_reciprocal_fraction_and() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareReciprocalFractionAnd(0.5, 0.5),
            StrandMethod::Ignore,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare_reciprocal_fraction_and() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareReciprocalFractionAnd(0.5, 0.5),
            StrandMethod::MatchStrand,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare_reciprocal_fraction_and() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareReciprocalFractionAnd(0.5, 0.5),
            StrandMethod::OppositeStrand,
        );
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    // CompareReciprocalFractionOr

    #[test]
    fn strand_ignore_compare_reciprocal_fraction_or() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareReciprocalFractionOr(0.5, 0.5),
            StrandMethod::Ignore,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(query.predicate(&iv_a, &iv_d));
        assert!(query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_match_compare_reciprocal_fraction_or() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareReciprocalFractionOr(0.5, 0.5),
            StrandMethod::MatchStrand,
        );
        assert!(query.predicate(&iv_a, &iv_b));
        assert!(!query.predicate(&iv_a, &iv_c));
        assert!(query.predicate(&iv_a, &iv_d));
        assert!(!query.predicate(&iv_a, &iv_e));
    }

    #[test]
    fn strand_opposite_compare_reciprocal_fraction_or() {
        let iv_a = StrandedBed3::new(1, 10, 100, Strand::Forward);
        let iv_b = StrandedBed3::new(1, 50, 150, Strand::Forward);
        let iv_c = StrandedBed3::new(1, 50, 150, Strand::Reverse);
        let iv_d = StrandedBed3::new(1, 15, 25, Strand::Forward);
        let iv_e = StrandedBed3::new(1, 15, 25, Strand::Reverse);
        let query = Query::new(
            QueryMethod::CompareReciprocalFractionOr(0.5, 0.5),
            StrandMethod::OppositeStrand,
        );
        assert!(!query.predicate(&iv_a, &iv_b));
        assert!(query.predicate(&iv_a, &iv_c));
        assert!(!query.predicate(&iv_a, &iv_d));
        assert!(query.predicate(&iv_a, &iv_e));
    }
}
