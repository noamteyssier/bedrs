pub mod container;
pub mod enums;
pub mod iterator;
pub mod record;
pub use container::IntervalContainer;
pub use enums::{Frame, Query, QueryMethod, Score, Strand, StrandMethod};
pub use iterator::{
    ClusterIter, FindIter, FindIterOwned, FindIterSorted, FindIterSortedOwned, IntersectIter,
    IntervalIterOwned, IntervalIterRef, MergeIter, SubtractFromIter, SubtractIter,
};
pub use record::{
    BaseInterval, Bed12, Bed3, Bed4, Bed6, BedGraph, Gtf, MetaInterval, StrandedBed3,
};
