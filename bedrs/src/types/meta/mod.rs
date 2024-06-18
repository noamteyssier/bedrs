pub mod bed12;
pub mod bed3;
pub mod bed4;
pub mod bed6;
pub mod bedgraph;
pub mod gtf;
pub mod meta_interval;
pub mod stranded_bed3;

pub use bed12::MetaBed12;
pub use bed3::MetaBed3;
pub use bed4::MetaBed4;
pub use bed6::MetaBed6;
pub use bedgraph::MetaBedGraph;
pub use gtf::MetaGtf;
pub use meta_interval::MetaMetaInterval;
pub use stranded_bed3::MetaStrandedBed3;

use crate::Strand;

pub trait RecordMetadata: std::fmt::Debug + Default + Clone {
    fn strand(&self) -> Option<Strand>;
    /// Does nothing by default
    fn update_strand(&mut self, _strand: Option<Strand>) {}
}
