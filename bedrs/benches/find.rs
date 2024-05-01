use bedrs::types::{BaseInterval, Bed3, IntervalContainer, Query};
use criterion::Criterion;

const N: usize = 10000;
const SIZE: usize = 100;

pub fn find_base(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| BaseInterval::new(x, y))
        .collect();
    let query = BaseInterval::new(20, 30);
    let set = IntervalContainer::from_sorted(records).unwrap();
    let method = Query::default();
    c.bench_function("find-base", |bench| {
        bench.iter(|| set.query(&query, method).unwrap())
    });
}

pub fn find_iter_base(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| BaseInterval::new(x, y))
        .collect();
    let query = BaseInterval::new(20, 30);
    let set = IntervalContainer::from_sorted(records).unwrap();
    let method = Query::default();
    c.bench_function("find-iter-base", |bench| {
        bench.iter(|| set.query_iter(&query, method).unwrap().count())
    });
}

pub fn find_iter_sort_base(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE))
        .map(|(x, y)| BaseInterval::new(x, y))
        .collect();
    let query = BaseInterval::new(20, 30);
    let set = IntervalContainer::from_sorted(records).unwrap();
    let method = Query::default();
    c.bench_function("find-iter-sort-base", |bench| {
        bench.iter(|| set.query_iter(&query, method).unwrap().count())
    });
}

pub fn find_genomic(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE, x % 5))
        .map(|(x, y, z)| Bed3::new(z, x, y))
        .collect();
    let query = Bed3::new(2, 20, 30);
    let set = IntervalContainer::from_sorted_unchecked(records);
    let method = Query::default();
    c.bench_function("find-genomic", |bench| {
        bench.iter(|| set.query(&query, method).unwrap())
    });
}

pub fn find_iter_genomic(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE, x % 5))
        .map(|(x, y, z)| Bed3::new(z, x, y))
        .collect();
    let query = Bed3::new(2, 20, 30);
    let set = IntervalContainer::from_sorted_unchecked(records);
    let method = Query::default();
    c.bench_function("find-iter-genomic", |bench| {
        bench.iter(|| set.query_iter(&query, method).unwrap().count())
    });
}

pub fn find_iter_sort_genomic(c: &mut Criterion) {
    let records = (0..N)
        .map(|x| (x, x + SIZE, x % 5))
        .map(|(x, y, z)| Bed3::new(z, x, y))
        .collect();
    let query = Bed3::new(2, 20, 30);
    let set = IntervalContainer::from_sorted_unchecked(records);
    let method = Query::default();
    c.bench_function("find-iter-sort-genomic", |bench| {
        bench.iter(|| set.query_iter(&query, method).unwrap().count())
    });
}
