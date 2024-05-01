use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[error(transparent)]
pub enum SetError {
    #[error("The interval set is unsorted.")]
    UnsortedSet,

    #[error("The interval set is empty.")]
    EmptySet,

    #[error("The provided intervals are not sorted.")]
    UnsortedIntervals,

    #[error("The maximum interval length is unknown")]
    MissingMaxLen,

    #[error("Sample size is larger than the number of intervals.")]
    SampleSizeTooLarge,

    #[error("Provided fraction {frac} is oversized. Must be (0, 1]")]
    FractionUnbounded { frac: f64 },

    #[error("Provided value must be greater than 0")]
    ZeroOrNegative,

    #[error("Cannot accept a strand input that is unknown")]
    CannotAcceptUnknownStrand,
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_set_error() {
        let err = SetError::UnsortedSet;
        assert_eq!(format!("{err}"), "The interval set is unsorted.");
        let err = SetError::EmptySet;
        assert_eq!(format!("{err}"), "The interval set is empty.");
        let err = SetError::UnsortedIntervals;
        assert_eq!(format!("{err}"), "The provided intervals are not sorted.");
        let err = SetError::MissingMaxLen;
        assert_eq!(format!("{err}"), "The maximum interval length is unknown");
        let err = SetError::SampleSizeTooLarge;
        assert_eq!(
            format!("{err}"),
            "Sample size is larger than the number of intervals."
        );
        let err = SetError::FractionUnbounded { frac: 1.0 };
        assert_eq!(
            format!("{err}"),
            "Provided fraction 1 is oversized. Must be (0, 1]"
        );
        let err = SetError::ZeroOrNegative;
        assert_eq!(format!("{err}"), "Provided value must be greater than 0");
        let err = SetError::CannotAcceptUnknownStrand;
        assert_eq!(
            format!("{err}"),
            "Cannot accept a strand input that is unknown"
        );
    }

    #[test]
    fn test_set_error_debug() {
        let err = SetError::UnsortedSet;
        assert_eq!(format!("{err:?}"), "UnsortedSet");
    }
}
