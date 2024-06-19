use super::{Bed3, Bed4, Coordinate};
use crate::{
    traits::{ChromBounds, MetaBounds},
    types::meta::RecordMetadata,
    Coordinates,
};
use coitrees::GenericInterval;
use derive_new::new;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Record<C, M>
where
    C: ChromBounds,
    M: RecordMetadata,
{
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub coordinates: Coordinate<C>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub metadata: M,
}

/// Implements the `GenericInterval` trait for `Record` types.
/// To make use of the `COItrees` interval tree, we need to implement this trait
impl<C, M> GenericInterval<M> for Record<C, M>
where
    C: ChromBounds,
    M: RecordMetadata,
{
    fn first(&self) -> i32 {
        self.coordinates.start()
    }
    fn last(&self) -> i32 {
        self.coordinates.end()
    }
    fn metadata(&self) -> &M {
        &self.metadata
    }
}

impl<C, M> Coordinates<C> for Record<C, M>
where
    C: ChromBounds,
    M: RecordMetadata,
{
    fn chr(&self) -> &C {
        self.coordinates.chr()
    }
    fn start(&self) -> i32 {
        self.coordinates.start()
    }
    fn end(&self) -> i32 {
        self.coordinates.end()
    }
    fn strand(&self) -> Option<crate::Strand> {
        self.metadata.strand()
    }
    fn update_chr(&mut self, val: &C) {
        self.coordinates.update_chr(val);
    }
    fn update_start(&mut self, val: &i32) {
        self.coordinates.update_start(val);
    }
    fn update_end(&mut self, val: &i32) {
        self.coordinates.update_end(val);
    }
    fn update_strand(&mut self, strand: Option<crate::Strand>) {
        self.metadata.update_strand(strand);
    }
    fn from<Iv: Coordinates<C>>(iv: &Iv) -> Self {
        let mut new = Self::default();
        new.update_chr(iv.chr());
        new.update_start(&iv.start());
        new.update_end(&iv.end());
        new.update_strand(iv.strand());
        new
    }
    fn empty() -> Self {
        Self::default()
    }
}

// ===========
// Conversions
// ===========

impl<C, N> From<Bed4<C, N>> for Bed3<C>
where
    C: ChromBounds,
    N: MetaBounds,
{
    fn from(record: Bed4<C, N>) -> Self {
        Bed3::new(record.coordinates.clone(), record.metadata.into())
    }
}