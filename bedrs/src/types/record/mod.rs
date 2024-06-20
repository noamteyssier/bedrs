mod base_interval;
mod features;
mod record;

pub use base_interval::BaseInterval;
pub use features::Features;
pub use record::Record;

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
