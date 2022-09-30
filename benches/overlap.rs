use bedrs::{types::{Interval, IntervalMeta, GenomicInterval}, traits::Overlap};
use criterion::Criterion;

pub fn overlap_base(c: &mut Criterion) {
    let a = Interval::new(10, 100);
    let b = Interval::new(50, 150);
    c.bench_function("overlap-base", |bench| {
        bench.iter(|| a.overlaps(&b))
    });
}

pub fn overlap_meta(c: &mut Criterion) {
    let a = IntervalMeta::new(10, 100, Some(0));
    let b = IntervalMeta::new(50, 150, Some(0));
    c.bench_function("overlap-meta", |bench| {
        bench.iter(|| a.overlaps(&b))
    });
}

pub fn overlap_cross(c: &mut Criterion) {
    let a = Interval::new(10, 100);
    let b = IntervalMeta::new(50, 150, Some(0));
    c.bench_function("overlap-meta", |bench| {
        bench.iter(|| a.overlaps(&b))
    });
}

pub fn overlap_genomic(c: &mut Criterion) {
    let x = GenomicInterval::new(1, 10, 100);
    let y = GenomicInterval::new(1, 50, 150);
    let z = GenomicInterval::new(2, 50, 150);
    c.bench_function("overlap-genomic-true", |bench| {
        bench.iter(|| x.overlaps(&y))
    });
    c.bench_function("overlap-genomic-false", |bench| {
        bench.iter(|| x.overlaps(&z))
    });
}
