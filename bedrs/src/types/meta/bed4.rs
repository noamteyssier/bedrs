use super::RecordMetadata;
use crate::{traits::MetaBounds, Strand};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
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
