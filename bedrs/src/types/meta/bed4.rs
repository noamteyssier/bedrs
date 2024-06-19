use super::{MetaBed12, MetaBed3, MetaBed6, RecordMetadata};
use crate::{
    traits::{MetaBounds, ValueBounds},
    Strand,
};
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

impl<N: MetaBounds> From<MetaBed6<N>> for MetaBed4<N> {
    fn from(value: MetaBed6<N>) -> Self {
        Self::new(value.name().clone())
    }
}

impl<N, Ts, Te, R, T, Si, St> From<MetaBed12<N, Ts, Te, R, T, Si, St>> for MetaBed4<N>
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
        Self::new(t.name().clone())
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn meta_convert_bed3_into_bed4() {
        let bed3 = MetaBed3::default();
        let bed4: MetaBed4<i32> = MetaBed4::from(bed3);
        assert_eq!(bed4, MetaBed4::default());
    }

    #[test]
    fn meta_convert_bed6_into_bed4() {
        let bed6 = MetaBed6::new(10, 20.into(), Strand::Forward);
        let bed4 = MetaBed4::from(bed6);
        assert_eq!(bed4, MetaBed4::new(10));
    }

    #[test]
    fn meta_convert_bed12_into_bed4() {
        let bed12 = MetaBed12::new(10, 20.into(), Strand::default(), 40, 50, 60, 70, 80, 90);
        let bed4 = MetaBed4::from(bed12);
        assert_eq!(bed4, MetaBed4::new(10));
    }
}
