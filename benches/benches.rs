
use std::{hint::black_box, path::Path};
use advent24::{day01, day02, day03, utils::get_problem_input};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let day01_input = get_problem_input(Path::new("input/day01.txt")).unwrap();
    let day02_input = get_problem_input(Path::new("input/day02.txt")).unwrap();
    let day03_input = get_problem_input(Path::new("input/day03.txt")).unwrap();

    c.bench_function("day01", |b| b.iter(|| day01::solve(&day01_input)));
    c.bench_function("day02", |b| b.iter(|| day02::solve(&day02_input)));
    c.bench_function("day03", |b| b.iter(|| day03::solve(&day03_input)));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);