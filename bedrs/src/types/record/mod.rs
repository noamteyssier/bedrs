mod base_interval;
mod bed12;
mod bed3;
mod bed4;
mod bed6;
mod bedgraph;
mod coordinate;
mod gtf;
mod meta_interval;
mod record;
mod stranded_genomic_interval;

pub use base_interval::BaseInterval;
pub use bed12::Bed12;
// pub use bed3::Bed3;
// pub use bed4::Bed4;
pub use bed6::Bed6;
pub use bedgraph::BedGraph;
pub use coordinate::Coordinate;
pub use gtf::Gtf;
pub use meta_interval::MetaInterval;
pub use record::Record;
pub use stranded_genomic_interval::StrandedBed3;

use super::meta::{MetaBed3, MetaBed4};

pub type Bed3<C> = Record<C, MetaBed3>;
pub type Bed4<C, N> = Record<C, MetaBed4<N>>;
