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
