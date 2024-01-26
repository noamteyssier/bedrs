use crate::traits::ValueBounds;

/// An enumeration of the different methods of querying a query
/// interval and a target interval
///
/// TODO: Validate that the query method is valid and remove Result from Find methods
#[derive(Debug, Default, Clone, Copy)]
pub enum QueryMethod<T: ValueBounds> {
    /// Compare the query and target intervals using the `overlaps` method
    #[default]
    Compare,

    /// Compare the query and target intervals using the `overlaps_by` method
    CompareBy(T),

    /// Compare the query and target intervals using the `overlaps_by_exactly` method
    CompareExact(T),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the query interval
    CompareByQueryFraction(f64),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the target interval
    CompareByTargetFraction(f64),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the query and target intervals
    /// respectively and accepting the query only if both of the fractions are met
    CompareReciprocalFractionAnd(f64, f64),

    /// Compare the query and target intervals using the `overlaps_by` method
    /// but calculating the minimum overlap as a fraction of the query and target intervals
    /// respectively and accepting the query if either of the fractions are met
    CompareReciprocalFractionOr(f64, f64),
}
