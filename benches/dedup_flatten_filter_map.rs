use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pushgen::{SliceGenerator, GeneratorExt};
use itertools::Itertools;

static mut ITER_RESULT: i32 = 0;
static mut GENERATOR_RESULT: i32 = 0;

fn run_iterator(data: &Vec<Vec<i32>>) {
    let mut result = 0i32;
    data.iter()
        .dedup()
        .flatten()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| result = result.wrapping_add(x));
    unsafe { ITER_RESULT = result };
    black_box(result);
}

fn run_generator(data: &Vec<Vec<i32>>) {
    let mut result = 0i32;
    SliceGenerator::new(data.as_slice())
        .dedup()
        .flatten(|x| SliceGenerator::new(x.as_slice()))
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 3)
        .for_each(|x| result = result.wrapping_add(x));
    unsafe { GENERATOR_RESULT = result };
    black_box(result);
}

pub fn make_data() -> Vec<Vec<i32>> {
    let mut data = Vec::new();
    data.reserve(100_000);
    for x in 0..100_000/4 {
        data.push(x);
        data.push(x);
        data.push(x);
        data.push(x);
    }

    let mut retval = Vec::new();
    for _x in 0..10 {
        retval.push(data.clone());
    }
    retval
}

pub fn benchmarks(c: &mut Criterion) {
    let data = make_data();
    c.bench_function("iterator_dedup_flatten_filter_map", |b| {
        b.iter(|| run_iterator(black_box(&data)))
    });
    c.bench_function("generator_dedup_flatten_filter_map", |b| {
        b.iter(|| run_generator(black_box(&data)))
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);