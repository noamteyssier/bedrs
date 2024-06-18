use crate::traits::ChromBounds;
use derive_new::new;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, new)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coordinate<C: ChromBounds> {
    chr: C,
    start: i32,
    end: i32,
}
impl<C: ChromBounds> Coordinate<C> {
    pub fn chr(&self) -> &C {
        &self.chr
    }
    pub fn start(&self) -> i32 {
        self.start
    }
    pub fn end(&self) -> i32 {
        self.end
    }
    pub fn update_chr(&mut self, val: &C) {
        self.chr = val.clone();
    }
    pub fn update_start(&mut self, val: &i32) {
        self.start = *val;
    }
    pub fn update_end(&mut self, val: &i32) {
        self.end = *val;
    }
}
