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
    let mut set = IntervalSet::new(records);
    set.sort();
    c.bench_function("merge-base", |bench| bench.iter(|| set.merge().unwrap()));
}

pub fn merge_genomic(c: &mut Criterion) {
    let records = (0..100)
        .map(|x| (x, x + 50, x % 5))
        .map(|(x, y, z)| GenomicInterval::new(z, x, y))
        .collect();
    let mut set = GenomicIntervalSet::new(records);
    set.sort();
    c.bench_function("merge-genomic", |bench| bench.iter(|| set.merge().unwrap()));
}

pub fn merge_unchecked_base(c: &mut Criterion) {
    let records = (0..100)
        .map(|x| (x, x + 50))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let mut set = IntervalSet::new(records);
    set.sort();
    c.bench_function("merge-unchecked-base", |bench| {
        bench.iter(|| set.merge_unchecked())
    });
}

pub fn merge_unchecked_genomic(c: &mut Criterion) {
    let records = (0..100)
        .map(|x| (x, x + 50, x % 5))
        .map(|(x, y, z)| GenomicInterval::new(z, x, y))
        .collect();
    let mut set = GenomicIntervalSet::new(records);
    set.sort();
    c.bench_function("merge-unchecked-genomic", |bench| {
        bench.iter(|| set.merge_unchecked())
    });
}
