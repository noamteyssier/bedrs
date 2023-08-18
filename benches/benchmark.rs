mod bound;
mod find;
mod merge;
mod overlap;
use bound::lower_bound;
use criterion::{criterion_group, criterion_main};
use find::{
    find_base, find_genomic, find_iter_base, find_iter_genomic, find_iter_sort_base,
    find_iter_sort_genomic,
};
use merge::{merge_base, merge_genomic, merge_unchecked_base, merge_unchecked_genomic};
use overlap::{overlap_base, overlap_genomic, overlap_named};

criterion_group!(bound, lower_bound);

criterion_group!(
    find,
    find_base,
    find_genomic,
    find_iter_base,
    find_iter_genomic,
    find_iter_sort_base,
    find_iter_sort_genomic
);
criterion_group!(
    merge,
    merge_base,
    merge_genomic,
    merge_unchecked_base,
    merge_unchecked_genomic
);
criterion_group!(overlap, overlap_base, overlap_genomic, overlap_named,);
criterion_main!(bound, find, merge, overlap);
