use cmder::{Command, Program};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_empty_cmd(c: &mut Criterion) {
    c.bench_function("build empty cmd", |b| {
        b.iter(|| {
            Command::new("empty");
        })
    });
}

pub fn build_empty_subcmd(c: &mut Criterion) {
    c.bench_function("build empty subcmd", |b| {
        b.iter(|| {
            Program::new().subcommand("empty");
        })
    });
}

pub fn parse_empty(c: &mut Criterion) {
    c.bench_function("parse empty", |b| {
        b.iter(|| {
            Program::new().parse_from(vec![""]);
        })
    });
}

criterion_group!(benches, build_empty_cmd, build_empty_subcmd, parse_empty);
criterion_main!(benches);
