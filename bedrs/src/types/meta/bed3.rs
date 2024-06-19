use super::{MetaBed4, RecordMetadata};
use crate::{traits::MetaBounds, Strand};
use derive_new::new;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, new, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBed3;
impl RecordMetadata for MetaBed3 {
    fn strand(&self) -> Option<Strand> {
        None
    }
    fn update_strand(&mut self, _strand: Option<Strand>) {}
}

impl<N: MetaBounds> From<MetaBed4<N>> for MetaBed3 {
    fn from(_t: MetaBed4<N>) -> Self {
        Self
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn convert_to_bed4() {
        let bed3 = MetaBed3;
        let bed4: MetaBed4<i32> = MetaBed4::from(bed3);
        assert_eq!(bed4, MetaBed4::default());
    }
}
