use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_solver::solution::Solver;
use aoc_solver::y2022::{MAX_DAYS, Y2022};

criterion_group!(benches, benchmark_all, benchmark_individual,);
criterion_main!(benches);

fn benchmark_all(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| Y2022::run_all());
    });
}

fn benchmark_individual(c: &mut Criterion) {
    for day in 1..=MAX_DAYS {
        c.bench_function(format!("day-{day}").as_str(), |b| {
            b.iter(|| Y2022::run_solution(black_box(day), black_box(None)));
        });
    }
}
