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
}
