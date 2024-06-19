use bedrs::{bed3, traits::Overlap, types::BaseInterval};
use criterion::Criterion;

pub fn overlap_base(c: &mut Criterion) {
    let a = BaseInterval::new(10, 100);
    let b = BaseInterval::new(50, 150);
    c.bench_function("overlap-base", |bench| bench.iter(|| a.overlaps(&b)));
}

pub fn overlap_genomic(c: &mut Criterion) {
    let x = bed3![1, 10, 100];
    let y = bed3![1, 50, 150];
    let z = bed3![2, 50, 150];
    c.bench_function("overlap-genomic-true", |bench| {
        bench.iter(|| x.overlaps(&y))
    });
    c.bench_function("overlap-genomic-false", |bench| {
        bench.iter(|| x.overlaps(&z))
    });
}

pub fn overlap_named(c: &mut Criterion) {
    let x = bed3!["chr1", 10, 100];
    let y = bed3!["chr1", 50, 150];
    let z = bed3!["chr2", 50, 150];
    c.bench_function("overlap-named-true", |bench| bench.iter(|| x.overlaps(&y)));
    c.bench_function("overlap-named-false", |bench| bench.iter(|| x.overlaps(&z)));
}
