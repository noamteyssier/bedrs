use super::{MetaBed12, MetaBed3, MetaBed4, RecordMetadata};
use crate::{
    traits::{MetaBounds, ValueBounds},
    Score, Strand,
};
use derive_new::new;
use getset::{Getters, Setters};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Getters, Setters, new, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(get = "pub", set = "pub")]
pub struct MetaBed6<N: MetaBounds> {
    name: N,
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

impl<N: MetaBounds> From<MetaBed3> for MetaBed6<N> {
    fn from(_t: MetaBed3) -> Self {
        Self::default()
    }
}

impl<N: MetaBounds> From<MetaBed4<N>> for MetaBed6<N> {
    fn from(t: MetaBed4<N>) -> Self {
        Self::new(t.name().clone(), Score::default(), Strand::default())
    }
}

impl<N, Ts, Te, R, T, Si, St> From<MetaBed12<N, Ts, Te, R, T, Si, St>> for MetaBed6<N>
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(t: MetaBed12<N, Ts, Te, R, T, Si, St>) -> Self {
        Self::new(t.name().clone(), *t.score(), t.strand().unwrap_or_default())
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn meta_convert_bed3_into_bed6() {
        let bed3 = MetaBed3::default();
        let bed6: MetaBed6<i32> = MetaBed6::from(bed3);
        assert_eq!(bed6, MetaBed6::default());
    }

    #[test]
    fn meta_convert_bed4_into_bed6() {
        let bed4 = MetaBed4::new(10);
        let bed6 = MetaBed6::from(bed4);
        assert_eq!(bed6, MetaBed6::new(10, Score::default(), Strand::default()));
    }

    #[test]
    fn meta_convert_bed12_into_bed6() {
        let bed12 = MetaBed12::new(10, 20.into(), Strand::default(), 40, 50, 60, 70, 80, 90);
        let bed6 = MetaBed6::from(bed12);
        assert_eq!(bed6, MetaBed6::new(10, 20.into(), Strand::default()));
    }
}
