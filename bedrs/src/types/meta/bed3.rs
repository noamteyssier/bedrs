use super::{MetaBed12, MetaBed4, MetaBed6, RecordMetadata};
use crate::{
    traits::{MetaBounds, ValueBounds},
    Strand,
};

/// Metadata for a BED3 record is empty
pub type MetaBed3 = Option<()>;
impl RecordMetadata for MetaBed3 {
    fn strand(&self) -> Option<Strand> {
        None
    }
    fn update_strand(&mut self, _strand: Option<Strand>) {}
}

impl<N: MetaBounds> From<MetaBed4<N>> for MetaBed3 {
    fn from(_t: MetaBed4<N>) -> Self {
        None
    }
}

impl<N: MetaBounds> From<MetaBed6<N>> for MetaBed3 {
    fn from(_t: MetaBed6<N>) -> Self {
        None
    }
}

impl<N, Ts, Te, R, T, Si, St> From<MetaBed12<N, Ts, Te, R, T, Si, St>> for MetaBed3
where
    N: MetaBounds,
    Ts: ValueBounds,
    Te: ValueBounds,
    R: MetaBounds,
    T: ValueBounds,
    Si: MetaBounds,
    St: MetaBounds,
{
    fn from(_t: MetaBed12<N, Ts, Te, R, T, Si, St>) -> Self {
        None
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn meta_convert_bed4_into_bed3() {
        let bed4 = MetaBed4::new(10);
        let bed3 = MetaBed3::from(bed4);
        assert!(bed3.is_none());
    }

    #[test]
    fn meta_convert_bed6_into_bed3() {
        let bed6 = MetaBed6::new(10, 20.into(), Strand::Forward);
        let bed3 = MetaBed3::from(bed6);
        assert!(bed3.is_none());
    }

    #[test]
    fn meta_convert_bed12_into_bed3() {
        let bed12 = MetaBed12::new(10, 20.into(), Strand::default(), 40, 50, 60, 70, 80, 90);
        let bed3 = MetaBed3::from(bed12);
        assert!(bed3.is_none());
    }
}
