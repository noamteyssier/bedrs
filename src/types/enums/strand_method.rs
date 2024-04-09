#[derive(Debug, Default, Clone, Copy)]
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
