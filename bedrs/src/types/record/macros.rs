#[macro_export]
macro_rules! impl_coordinates_for_bed_structs {
    ($struct_name:ident, <$($gen:ident: $bound:ident),*>) => {
        impl<'a, $($gen),*> Coordinates<C, T> for &'a $struct_name<$($gen),*>
        where
            $($gen: $bound),*
        {
            fn empty() -> Self {
                unreachable!("Cannot create an empty immutable reference")
            }

            fn start(&self) -> T {
                self.start
            }

            fn end(&self) -> T {
                self.end
            }

            fn chr(&self) -> &C {
                &self.chr
            }

            #[allow(unused)]
            fn update_start(&mut self, _val: &T) {
                unreachable!("Cannot update an immutable reference")
            }

            #[allow(unused)]
            fn update_end(&mut self, _val: &T) {
                unreachable!("Cannot update an immutable reference")
            }

            #[allow(unused)]
            fn update_chr(&mut self, _val: &C) {
                unreachable!("Cannot update an immutable reference")
            }

            fn from<Iv>(_other: &Iv) -> Self {
                unimplemented!("Cannot create a new reference from a reference")
            }
        }

        impl<'a, $($gen),*> Coordinates<C, T> for &'a mut $struct_name<$($gen),*>
        where
            $($gen: $bound),*
        {
            fn empty() -> Self {
                unreachable!("Cannot create an empty mutable reference")
            }

            fn start(&self) -> T {
                self.start
            }

            fn end(&self) -> T {
                self.end
            }

            fn chr(&self) -> &C {
                &self.chr
            }

            fn update_start(&mut self, val: &T) {
                self.start = *val;
            }

            fn update_end(&mut self, val: &T) {
                self.end = *val;
            }

            fn update_chr(&mut self, val: &C) {
                self.chr = val.clone();
            }

            fn from<Iv>(_other: &Iv) -> Self {
                unimplemented!("Cannot create a new reference from a mutable reference")
            }
        }
    };
}

// use crate::{
//     traits::{ChromBounds, Coordinates, MetaBounds, ValueBounds},
//     types::{Bed12, Bed3, Bed4, Bed6, BedGraph, Gtf, MetaInterval, StrandedBed3},
// };

// Usage example for Bed3 and Bed4
// impl_coordinates_for_bed_structs!(Bed3, <C: ChromBounds, T: ValueBounds>);
// impl_coordinates_for_bed_structs!(Bed4, <C: ChromBounds, T: ValueBounds, M: MetaBounds>);
// impl_coordinates_for_bed_structs!(Bed6, <C: ChromBounds, T: ValueBounds, M: MetaBounds>);
// impl_coordinates_for_bed_structs!(Bed12, <C: ChromBounds, T: ValueBounds, N: MetaBounds, Ts: ValueBounds, Te: ValueBounds, R: MetaBounds, Si: MetaBounds, St: MetaBounds>);
// impl_coordinates_for_bed_structs!(BedGraph, <C: ChromBounds, T: ValueBounds>);
// impl_coordinates_for_bed_structs!(Gtf, <C: ChromBounds, T: ValueBounds, N: MetaBounds>);
// impl_coordinates_for_bed_structs!(MetaInterval, <C: ChromBounds, T: ValueBounds, M: MetaBounds>);
// impl_coordinates_for_bed_structs!(StrandedBed3, <C: ChromBounds, T: ValueBounds>);
