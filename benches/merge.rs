use bedrs::{
    traits::{Container, Merge},
    types::{GenomicInterval, GenomicIntervalSet, Interval, IntervalSet},
};
use criterion::Criterion;

pub fn merge_base(c: &mut Criterion) {
    let records = (0..100)
        .map(|x| (x, x + 50))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let set = IntervalSet::new(records);
    c.bench_function("merge-base", |bench| bench.iter(|| set.merge()));
}

pub fn merge_genomic(c: &mut Criterion) {
    let records = (0..100)
        .map(|x| (x, x + 50, x % 5))
        .map(|(x, y, z)| GenomicInterval::new(z, x, y))
        .collect();
    let mut set = GenomicIntervalSet::new(records);
    set.sort();
    c.bench_function("merge-genomic", |bench| bench.iter(|| set.merge()));
}
