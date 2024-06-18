use super::RecordMetadata;
use crate::Strand;
use derive_new::new;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBed3;
impl RecordMetadata for MetaBed3 {
    fn strand(&self) -> Option<Strand> {
        None
    }
    fn update_strand(&mut self, _strand: Option<Strand>) {}
}
