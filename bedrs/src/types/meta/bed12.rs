use super::{MetaBed3, MetaBed4, MetaBed6, RecordMetadata};
use crate::{
    traits::{MetaBounds, ValueBounds},
    Score, Strand,
};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(clippy::too_many_arguments)]
#[derive(Debug, Default, Clone, Copy, Getters, Setters, new, PartialEq)]
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
    #[getset(get = "pub", set = "pub")]
    score: Score,
    strand: Strand,
    #[getset(get = "pub", set = "pub")]
    thick_start: Ts,
    #[getset(get = "pub", set = "pub")]
    thick_end: Te,
    #[getset(get = "pub", set = "pub")]
    item_rgb: R,
    #[getset(get = "pub", set = "pub")]
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

impl<N, Ts, Te, R, T, Si, St> From<MetaBed3> for MetaBed12<N, Ts, Te, R, T, Si, St>
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(_meta: MetaBed3) -> Self {
        Self::default()
    }
}

impl<N, Ts, Te, R, T, Si, St> From<MetaBed4<N>> for MetaBed12<N, Ts, Te, R, T, Si, St>
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(meta: MetaBed4<N>) -> Self {
        Self {
            name: meta.name().clone(),
            score: Score::default(),
            strand: Strand::default(),
            thick_start: Ts::default(),
            thick_end: Te::default(),
            item_rgb: R::default(),
            block_count: T::default(),
            block_sizes: Si::default(),
            block_starts: St::default(),
        }
    }
}

impl<N, Ts, Te, R, T, Si, St> From<MetaBed6<N>> for MetaBed12<N, Ts, Te, R, T, Si, St>
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(meta: MetaBed6<N>) -> Self {
        Self {
            name: meta.name().clone(),
            score: *meta.score(),
            strand: *meta.strand(),
            thick_start: Ts::default(),
            thick_end: Te::default(),
            item_rgb: R::default(),
            block_count: T::default(),
            block_sizes: Si::default(),
            block_starts: St::default(),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn meta_convert_bed3_into_bed12() {
        let bed3 = MetaBed3::default();
        let bed12: MetaBed12<i32, i32, i32, i32, i32, i32, i32> = bed3.into();
        assert_eq!(bed12, MetaBed12::default());
    }

    #[test]
    fn meta_convert_bed4_into_bed12() {
        let bed4 = MetaBed4::new("name");
        let bed12: MetaBed12<&str, i32, i32, i32, i32, i32, i32> = bed4.into();
        assert_eq!(
            bed12,
            MetaBed12::new("name", None.into(), Strand::Unknown, 0, 0, 0, 0, 0, 0)
        );
    }

    #[test]
    fn meta_convert_bed6_into_bed12() {
        let bed4 = MetaBed6::new("name", 100.into(), Strand::Forward);
        let bed12: MetaBed12<&str, i32, i32, i32, i32, i32, i32> = bed4.into();
        assert_eq!(
            bed12,
            MetaBed12::new("name", 100.into(), Strand::Forward, 0, 0, 0, 0, 0, 0)
        );
    }
}
