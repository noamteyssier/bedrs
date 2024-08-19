use crate::prelude::{ChromBounds, Features, GenericIntervalExt, RecordMetadata, Strand};
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
    pub features: Features<C>,
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
        self.features.start()
    }
    fn last(&self) -> i32 {
        self.features.end()
    }
    fn metadata(&self) -> &M {
        &self.metadata
    }
    fn len(&self) -> i32 {
        self.last() - self.first()
    }
}

impl<C, M> GenericInterval<M> for &Record<C, M>
where
    C: ChromBounds,
    M: RecordMetadata,
{
    fn first(&self) -> i32 {
        self.features.start()
    }
    fn last(&self) -> i32 {
        self.features.end()
    }
    fn metadata(&self) -> &M {
        &self.metadata
    }
    fn len(&self) -> i32 {
        self.last() - self.first()
    }
}

impl<C, M> GenericInterval<M> for &mut Record<C, M>
where
    C: ChromBounds,
    M: RecordMetadata,
{
    fn first(&self) -> i32 {
        self.features.start()
    }
    fn last(&self) -> i32 {
        self.features.end()
    }
    fn metadata(&self) -> &M {
        &self.metadata
    }
    fn len(&self) -> i32 {
        self.last() - self.first()
    }
}

impl<C, T> GenericIntervalExt<C, T> for Record<C, T>
where
    C: ChromBounds,
    T: RecordMetadata,
{
    fn chr(&self) -> &C {
        self.features.chr()
    }
    fn start(&self) -> i32 {
        self.features.start()
    }
    fn end(&self) -> i32 {
        self.features.end()
    }
    fn strand(&self) -> Option<Strand> {
        self.metadata.strand()
    }
    fn update_chr(&mut self, val: &C) {
        self.features.update_chr(val);
    }
    fn update_start(&mut self, val: &i32) {
        self.features.update_start(val);
    }
    fn update_end(&mut self, val: &i32) {
        self.features.update_end(val);
    }
    fn update_strand(&mut self, val: Option<Strand>) {
        self.metadata.update_strand(val);
    }
    fn from<Iv, V>(other: &Iv) -> Self
    where
        Iv: GenericIntervalExt<C, V>,
        V: Clone + From<T> + Into<T>,
    {
        let features = Features::new(other.chr().clone(), other.start(), other.end());
        let metadata = other.metadata().clone().into();
        Self { features, metadata }
    }
    fn empty() -> Self {
        Self::default()
    }
}

// ===========
// Conversions
// ===========

impl<C, M1, M2> From<&Record<C, M1>> for Record<C, M2>
where
    C: ChromBounds,
    M1: RecordMetadata,
    M2: RecordMetadata + From<M1>,
{
    fn from(rec: &Record<C, M1>) -> Self {
        Record {
            features: rec.features.clone(),
            metadata: rec.metadata.clone().into(),
        }
    }
}
