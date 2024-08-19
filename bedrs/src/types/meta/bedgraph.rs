use super::{MetaBed3, RecordMetadata};
use crate::types::{Score, Strand};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBedGraph {
    #[getset(get = "pub", set = "pub")]
    score: Score,
}
impl RecordMetadata for MetaBedGraph {
    fn strand(&self) -> Option<Strand> {
        None
    }
}

impl From<MetaBed3> for MetaBedGraph {
    fn from(_bed3: MetaBed3) -> Self {
        Self::default()
    }
}
