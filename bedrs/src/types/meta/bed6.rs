use super::RecordMetadata;
use crate::{traits::MetaBounds, Score, Strand};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBed6<N: MetaBounds> {
    #[getset(get = "pub", set = "pub")]
    name: N,
    #[getset(get_copy = "pub", set = "pub")]
    score: Score,
    strand: Strand,
}
impl<N: MetaBounds> RecordMetadata for MetaBed6<N> {
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
    }
    fn update_strand(&mut self, strand: Option<Strand>) {
        if let Some(s) = strand {
            self.strand = s;
        }
    }
}
