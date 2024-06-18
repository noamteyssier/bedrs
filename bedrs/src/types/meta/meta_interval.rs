use super::RecordMetadata;
use crate::traits::MetaBounds;
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaMetaInterval<N: MetaBounds> {
    #[getset(get = "pub", set = "pub")]
    meta: N,
}
impl<N: MetaBounds> RecordMetadata for MetaMetaInterval<N> {
    fn strand(&self) -> Option<crate::Strand> {
        None
    }
}
