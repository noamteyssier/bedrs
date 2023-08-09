mod find;
mod merge;
mod overlap;
use criterion::{criterion_group, criterion_main, Criterion};
use find::{
    find_base, find_genomic, find_iter_base, find_iter_genomic, find_iter_sort_base,
    find_iter_sort_genomic,
};
use merge::{merge_base, merge_genomic, merge_unchecked_base, merge_unchecked_genomic};
use overlap::{overlap_base, overlap_cross, overlap_genomic, overlap_meta};

mod perf;

criterion_group! {
    name = find;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = find_base, find_genomic, find_iter_base, find_iter_genomic, find_iter_sort_base, find_iter_sort_genomic
}
criterion_group!(
    merge,
    merge_base,
    merge_genomic,
    merge_unchecked_base,
    merge_unchecked_genomic
);
criterion_group!(
    overlap,
    overlap_base,
    overlap_meta,
    overlap_cross,
    overlap_genomic
);
// criterion_main!(find, merge, overlap);
criterion_main!(find);
