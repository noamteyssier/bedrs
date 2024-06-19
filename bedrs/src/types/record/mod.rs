mod base_interval;
// mod bed12;
// mod bed3;
// mod bed4;
// mod bed6;
// mod bedgraph;
mod coordinate;
// mod gtf;
// mod meta_interval;
mod record;
// mod stranded_genomic_interval;

pub use base_interval::BaseInterval;
// pub use bed12::Bed12;
// pub use bed3::Bed3;
// pub use bed4::Bed4;
// pub use bed6::Bed6;
// pub use bedgraph::BedGraph;
pub use coordinate::Coordinate;
// pub use gtf::Gtf;
// pub use meta_interval::MetaInterval;
pub use record::Record;
// pub use stranded_genomic_interval::StrandedBed3;

use super::meta::{
    MetaBed12, MetaBed3, MetaBed4, MetaBed6, MetaBedGraph, MetaGtf, MetaMetaInterval,
    MetaStrandedBed3,
};

pub type Bed3<C> = Record<C, MetaBed3>;
pub type Bed4<C, N> = Record<C, MetaBed4<N>>;
pub type Bed6<C, N> = Record<C, MetaBed6<N>>;
pub type Bed12<C, N, Ts, Te, R, T, Si, St> = Record<C, MetaBed12<N, Ts, Te, R, T, Si, St>>;
pub type BedGraph<C> = Record<C, MetaBedGraph>;
pub type Gtf<C, N> = Record<C, MetaGtf<N>>;
pub type MetaInterval<C, N> = Record<C, MetaMetaInterval<N>>;
pub type StrandedBed3<C> = Record<C, MetaStrandedBed3>;
