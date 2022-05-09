use cmder::core::new_program::Program;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_empty(c: &mut Criterion) {
    c.bench_function("build empty", |b| {
        b.iter(|| {
            Program::new().subcommand("empty");
        })
    });
}

pub fn parse_empty(c: &mut Criterion) {
    c.bench_function("parse empty", |b| {
        b.iter(|| {
            Program::new().parse_from(vec![]);
        })
    });
}

criterion_group!(benches, build_empty, parse_empty);
criterion_main!(benches);
