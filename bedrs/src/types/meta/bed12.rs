use super::RecordMetadata;
use crate::{
    traits::{MetaBounds, ValueBounds},
    Score, Strand,
};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(clippy::too_many_arguments)]
#[derive(Debug, Default, Clone, Copy, Getters, Setters, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MetaBed12<N, Ts, Te, R, T, Si, St>
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    #[getset(get = "pub", set = "pub")]
    name: N,
    #[getset(get_copy = "pub", set = "pub")]
    score: Score,
    strand: Strand,
    #[getset(get_copy = "pub", set = "pub")]
    thick_start: Ts,
    #[getset(get_copy = "pub", set = "pub")]
    thick_end: Te,
    #[getset(get = "pub", set = "pub")]
    item_rgb: R,
    #[getset(get_copy = "pub", set = "pub")]
    block_count: T,
    #[getset(get = "pub", set = "pub")]
    block_sizes: Si,
    #[getset(get = "pub", set = "pub")]
    block_starts: St,
}
impl<N, Ts, Te, R, T, Si, St> RecordMetadata for MetaBed12<N, Ts, Te, R, T, Si, St>
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
    }
    fn update_strand(&mut self, strand: Option<Strand>) {
        if let Some(s) = strand {
            self.strand = s;
        }
    }
}
