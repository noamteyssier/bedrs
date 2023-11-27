use bedrs::{Bound, Container, Interval, IntervalContainer};
use criterion::Criterion;

const N: usize = 10000;
const SIZE: usize = 100;

pub fn lower_bound(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let query = Interval::new(20, 30);
    let set = IntervalContainer::new(records);
    c.bench_function("bound_original", |bench| {
        bench.iter(|| set.lower_bound_unchecked(&query))
    });
}

pub fn chr_bound_upstream(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let query = Interval::new(20, 30);
    let set = IntervalContainer::new(records);
    c.bench_function("bound_upstream", |bench| {
        bench.iter(|| set.chr_bound_upstream_unchecked(&query))
    });
}

pub fn chr_bound_downstream(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| Interval::new(x, y))
        .collect();
    let query = Interval::new(20, 30);
    let set = IntervalContainer::new(records);
    c.bench_function("bound_downstream", |bench| {
        bench.iter(|| set.chr_bound_downstream_unchecked(&query))
    });
}
