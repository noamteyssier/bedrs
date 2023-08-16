use bedrs::{
    traits::Overlap,
    types::{GenomicInterval, Interval, IntervalMeta},
    NamedInterval,
};
use criterion::Criterion;
use tinystr::{TinyStr16, TinyStr4, TinyStr8};

pub fn overlap_base(c: &mut Criterion) {
    let a = Interval::new(10, 100);
    let b = Interval::new(50, 150);
    c.bench_function("overlap-base", |bench| bench.iter(|| a.overlaps(&b)));
}

pub fn overlap_meta(c: &mut Criterion) {
    let a = IntervalMeta::new(10, 100, Some(0));
    let b = IntervalMeta::new(50, 150, Some(0));
    c.bench_function("overlap-meta", |bench| bench.iter(|| a.overlaps(&b)));
}

pub fn overlap_cross(c: &mut Criterion) {
    let a = Interval::new(10, 100);
    let b = IntervalMeta::new(50, 150, Some(0));
    c.bench_function("overlap-meta", |bench| bench.iter(|| a.overlaps(&b)));
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

pub fn overlap_named(c: &mut Criterion) {
    let x = NamedInterval::new("chr1", 10, 100);
    let y = NamedInterval::new("chr1", 50, 150);
    let z = NamedInterval::new("chr2", 50, 150);
    c.bench_function("overlap-named-true", |bench| bench.iter(|| x.overlaps(&y)));
    c.bench_function("overlap-named-false", |bench| bench.iter(|| x.overlaps(&z)));
}

#[derive(Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
struct TinyStrWrapper(TinyStr8);
impl From<&str> for TinyStrWrapper {
    fn from(s: &str) -> Self {
        Self(TinyStr8::from_str(s).unwrap())
    }
}
impl Default for TinyStrWrapper {
    fn default() -> Self {
        Self(TinyStr8::from_str("").unwrap())
    }
}

pub fn overlap_named_tinystr(c: &mut Criterion) {
    let chr_a = TinyStrWrapper::from("chr1");
    let chr_b = TinyStrWrapper::from("chr2");

    let x = NamedInterval::new(chr_a.clone(), 10, 100);
    let y = NamedInterval::new(chr_a.clone(), 50, 150);
    let z = NamedInterval::new(chr_b.clone(), 50, 150);
    c.bench_function("overlap-named-true-tinystr", |bench| {
        bench.iter(|| x.overlaps(&y))
    });
    c.bench_function("overlap-named-false-tinystr", |bench| {
        bench.iter(|| x.overlaps(&z))
    });
}
