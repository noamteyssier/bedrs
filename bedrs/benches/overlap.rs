use bedrs::{
    traits::Overlap,
    types::{BaseInterval, Bed3},
};
use criterion::Criterion;

pub fn overlap_base(c: &mut Criterion) {
    let a = BaseInterval::new(10, 100);
    let b = BaseInterval::new(50, 150);
    c.bench_function("overlap-base", |bench| bench.iter(|| a.overlaps(&b)));
}

pub fn overlap_genomic(c: &mut Criterion) {
    let x = Bed3::new(1, 10, 100);
    let y = Bed3::new(1, 50, 150);
    let z = Bed3::new(2, 50, 150);
    c.bench_function("overlap-genomic-true", |bench| {
        bench.iter(|| x.overlaps(&y))
    });
    c.bench_function("overlap-genomic-false", |bench| {
        bench.iter(|| x.overlaps(&z))
    });
}

pub fn overlap_named(c: &mut Criterion) {
    let x = Bed3::new("chr1", 10, 100);
    let y = Bed3::new("chr1", 50, 150);
    let z = Bed3::new("chr2", 50, 150);
    c.bench_function("overlap-named-true", |bench| bench.iter(|| x.overlaps(&y)));
    c.bench_function("overlap-named-false", |bench| bench.iter(|| x.overlaps(&z)));
}
