use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
#[error(transparent)]
pub enum SetError {
    #[error("The interval set is unsorted.")]
    UnsortedSet,

    #[error("The interval set is empty.")]
    EmptySet,
}
