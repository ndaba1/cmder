use cmder::{Command, Program};
use criterion::{criterion_group, criterion_main, Criterion};

fn build_empty_cmd(c: &mut Criterion) {
    c.bench_function("build_empty_cmd", |b| {
        b.iter(|| {
            Command::new("empty");
        })
    });
}

fn build_empty_subcmd(c: &mut Criterion) {
    c.bench_function("build_empty_subcmd", |b| {
        b.iter(|| {
            Program::new().subcommand("empty");
        })
    });
}

fn parse_empty(c: &mut Criterion) {
    c.bench_function("parse_empty", |b| {
        b.iter(|| {
            Program::new().parse_from(vec![""]);
        })
    });
}

criterion_group!(benches, build_empty_cmd, build_empty_subcmd, parse_empty);
criterion_main!(benches);
