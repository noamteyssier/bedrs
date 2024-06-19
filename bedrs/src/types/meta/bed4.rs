use super::{MetaBed3, RecordMetadata};
use crate::{traits::MetaBounds, Strand};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBed4<N: MetaBounds> {
    #[getset(get = "pub", set = "pub")]
    name: N,
}
impl<N: MetaBounds> RecordMetadata for MetaBed4<N> {
    fn strand(&self) -> Option<Strand> {
        None
    }
}

impl<N: MetaBounds> From<MetaBed3> for MetaBed4<N> {
    fn from(_t: MetaBed3) -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn convert_to_bed3() {
        let bed4 = MetaBed4::new(10);
        let bed3 = MetaBed3::from(bed4);
        assert_eq!(bed3, MetaBed3);
    }
}
