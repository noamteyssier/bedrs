#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StrandMethod {
    /// Ignore the strand of the query and target intervals
    #[default]
    Ignore,
    /// Match the strand of the query and target intervals
    /// if they are on the same strand
    MatchStrand,
    /// Match the strand of the query and target intervals
    /// if they are *NOT* on the same strand
    OppositeStrand,
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", StrandMethod::Ignore), "Ignore");
        assert_eq!(format!("{:?}", StrandMethod::MatchStrand), "MatchStrand");
        assert_eq!(
            format!("{:?}", StrandMethod::OppositeStrand),
            "OppositeStrand"
        );
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_clone() {
        let a = StrandMethod::Ignore;
        let b = a.clone();
        assert_eq!(a, b);
    }
}
