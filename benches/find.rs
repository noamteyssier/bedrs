use bedrs::{
    traits::{Container, Find},
    types::{GenomicInterval, GenomicIntervalSet, Interval, IntervalSet},
};
use criterion::Criterion;

const N: usize = 10000;
const SIZE: usize = 100;

pub fn find_base(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let query = Interval::new(20, 30);
    let set = IntervalSet::new(records);
    c.bench_function("find-base", |bench| bench.iter(|| set.find(&query)));
}

pub fn find_iter_base(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let query = Interval::new(20, 30);
    let set = IntervalSet::new(records);
    c.bench_function("find-iter-base", |bench| {
        bench.iter(|| set.find_iter(&query).count())
    });
}

pub fn find_iter_sort_base(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let query = Interval::new(20, 30);
    let mut set = IntervalSet::new(records);
    set.sort();
    c.bench_function("find-iter-sort-base", |bench| {
        bench.iter(|| set.find_iter_sorted_unchecked(&query).count())
    });
}

pub fn find_genomic(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE, x % 5))
        .map(|(x, y, z)| GenomicInterval::new(z, x, y))
        .collect();
    let query = GenomicInterval::new(2, 20, 30);
    let set = GenomicIntervalSet::new(records);
    c.bench_function("find-genomic", |bench| bench.iter(|| set.find(&query)));
}

pub fn find_iter_genomic(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE, x % 5))
        .map(|(x, y, z)| GenomicInterval::new(z, x, y))
        .collect();
    let query = GenomicInterval::new(2, 20, 30);
    let set = GenomicIntervalSet::new(records);
    c.bench_function("find-iter-genomic", |bench| {
        bench.iter(|| set.find_iter(&query).count())
    });
}

pub fn find_iter_sort_genomic(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE, x % 5))
        .map(|(x, y, z)| GenomicInterval::new(z, x, y))
        .collect();
    let query = GenomicInterval::new(2, 20, 30);
    let mut set = GenomicIntervalSet::new(records);
    set.sort();
    c.bench_function("find-iter-sort-genomic", |bench| {
        bench.iter(|| set.find_iter_sorted_unchecked(&query).count())
    });
}
