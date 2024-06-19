use crate::traits::SetError;

/// An enumeration of the different methods of querying a query
/// interval and a target interval
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum QueryMethod {
    /// Compare the query and target intervals using the `overlaps` method
    #[default]
    Compare,

    /// Compare the query and target intervals using the `overlaps_by` method
    CompareBy(i32),

    /// Compare the query and target intervals using the `overlaps_by_exactly` method
    CompareExact(i32),

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
impl QueryMethod {
    pub fn validate(&self) -> Result<(), SetError> {
        match self {
            Self::Compare => Ok(()),
            Self::CompareBy(val) | Self::CompareExact(val) => {
                if *val <= 0 {
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

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_debug() {
        let str_compare = format!("{:?}", QueryMethod::Compare);
        assert_eq!(str_compare, "Compare");

        let str_compare_by = format!("{:?}", QueryMethod::CompareBy(5));
        assert_eq!(str_compare_by, "CompareBy(5)");

        let str_compare_exact = format!("{:?}", QueryMethod::CompareExact(5));
        assert_eq!(str_compare_exact, "CompareExact(5)");

        let str_compare_by_query_fraction =
            format!("{:?}", QueryMethod::CompareByQueryFraction(0.5));
        assert_eq!(str_compare_by_query_fraction, "CompareByQueryFraction(0.5)");

        let str_compare_by_target_fraction =
            format!("{:?}", QueryMethod::CompareByTargetFraction(0.5));
        assert_eq!(
            str_compare_by_target_fraction,
            "CompareByTargetFraction(0.5)"
        );

        let str_compare_reciprocal_fraction_and =
            format!("{:?}", QueryMethod::CompareReciprocalFractionAnd(0.5, 0.5));
        assert_eq!(
            str_compare_reciprocal_fraction_and,
            "CompareReciprocalFractionAnd(0.5, 0.5)"
        );

        let str_compare_reciprocal_fraction_or =
            format!("{:?}", QueryMethod::CompareReciprocalFractionOr(0.5, 0.5));
        assert_eq!(
            str_compare_reciprocal_fraction_or,
            "CompareReciprocalFractionOr(0.5, 0.5)"
        );
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let a = QueryMethod::Compare;
        let b = a.clone();
        assert_eq!(a, b);
    }
}
