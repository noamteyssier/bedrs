use super::RecordMetadata;
use crate::{traits::MetaBounds, Frame, Score, Strand};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaGtf<N: MetaBounds> {
    #[getset(get = "pub", set = "pub")]
    source: N,
    #[getset(get = "pub", set = "pub")]
    feature: N,
    #[getset(get = "pub", set = "pub")]
    score: Score,
    strand: Strand,
    #[getset(get = "pub", set = "pub")]
    frame: Frame,
    #[getset(get = "pub", set = "pub")]
    attributes: N,
}
impl<N: MetaBounds> RecordMetadata for MetaGtf<N> {
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
    }
    fn update_strand(&mut self, strand: Option<Strand>) {
        if let Some(s) = strand {
            self.strand = s;
        }
    }
}
