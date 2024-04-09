use crate::traits::{SetError, ValueBounds};

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
impl<T: ValueBounds> QueryMethod<T> {
    pub fn validate(&self) -> Result<(), SetError> {
        match self {
            Self::Compare => Ok(()),
            Self::CompareBy(val) | Self::CompareExact(val) => {
                if val <= &T::zero() {
                    Err(SetError::ZeroOrNegative)
                } else {
                    Ok(())
                }
            }
            Self::CompareByQueryFraction(frac) | Self::CompareByTargetFraction(frac) => {
                if frac <= &0.0 || frac > &1.0 {
                    Err(SetError::FractionUnbounded { frac: *frac })
                } else {
                    Ok(())
                }
            }
            Self::CompareReciprocalFractionAnd(f_query, f_target)
            | Self::CompareReciprocalFractionOr(f_query, f_target) => {
                if f_query <= &0.0 || f_query > &1.0 || f_target <= &0.0 || f_target > &1.0 {
                    if f_query <= &0.0 || f_query > &1.0 {
                        Err(SetError::FractionUnbounded { frac: *f_query })
                    } else {
                        Err(SetError::FractionUnbounded { frac: *f_target })
                    }
                } else {
                    Ok(())
                }
            }
        }
    }
}
