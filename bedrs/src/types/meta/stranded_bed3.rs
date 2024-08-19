use super::{MetaBed12, MetaBed3, MetaBed4, MetaBed6, MetaBedGraph, RecordMetadata};
use crate::prelude::{MetaBounds, Strand, ValueBounds};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaStrandedBed3 {
    #[getset(get = "pub", set = "pub")]
    strand: Strand,
}
impl RecordMetadata for MetaStrandedBed3 {
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
    }
    fn update_strand(&mut self, strand: Option<Strand>) {
        if let Some(s) = strand {
            self.strand = s;
        }
    }
}

impl From<MetaBed3> for MetaStrandedBed3 {
    fn from(_: MetaBed3) -> Self {
        Self::new(Strand::Unknown)
    }
}
impl<N: MetaBounds> From<MetaBed4<N>> for MetaStrandedBed3 {
    fn from(_: MetaBed4<N>) -> Self {
        Self::new(Strand::Unknown)
    }
}
impl<N: MetaBounds> From<MetaBed6<N>> for MetaStrandedBed3 {
    fn from(t: MetaBed6<N>) -> Self {
        Self::new(*t.strand())
    }
}
impl From<MetaBedGraph> for MetaStrandedBed3 {
    fn from(_: MetaBedGraph) -> Self {
        Self::new(Strand::Unknown)
    }
}
impl<N, Ts, Te, R, T, Si, St> From<MetaBed12<N, Ts, Te, R, T, Si, St>> for MetaStrandedBed3
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(t: MetaBed12<N, Ts, Te, R, T, Si, St>) -> Self {
        Self::new(t.strand().unwrap_or_default())
    }
}
