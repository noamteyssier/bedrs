use bedrs::{types::StrandMethod, BaseInterval, IntervalContainer};
use criterion::Criterion;

const N: i32 = 10000;
const SIZE: i32 = 100;

pub fn lower_bound(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| BaseInterval::new(x, y))
        .collect();
    let query = BaseInterval::new(20, 30);
    let set = IntervalContainer::new(records);
    c.bench_function("bound_original", |bench| {
        bench.iter(|| set.lower_bound_unchecked(&query))
    });
}

pub fn chr_bound_upstream(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| BaseInterval::new(x, y))
        .collect();
    let query = BaseInterval::new(20, 30);
    let set = IntervalContainer::new(records);
    c.bench_function("bound_upstream", |bench| {
        bench.iter(|| set.bound_upstream_unchecked(&query, StrandMethod::Ignore))
    });
}

pub fn chr_bound_downstream(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| BaseInterval::new(x, y))
        .collect();
    let query = BaseInterval::new(20, 30);
    let set = IntervalContainer::new(records);
    c.bench_function("bound_downstream", |bench| {
        bench.iter(|| set.bound_downstream_unchecked(&query, StrandMethod::Ignore))
    });
}
