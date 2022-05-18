use cmder::Command;
use criterion::{criterion_group, criterion_main, Criterion};

fn build_with_args(c: &mut Criterion) {
    c.bench_function("build_with_args", |b| {
        b.iter(|| {
            Command::new("yargs")
                .argument("<path>", "Some path")
                .argument("[value]", "Some value");
        })
    });
}

fn build_with_flags(c: &mut Criterion) {
    c.bench_function("build_with_flags", |b| {
        b.iter(|| {
            Command::new("flags")
                .option("-x --extra", "Something extra")
                .option("-v --verbose", "Verbosity");
        })
    });
}

fn build_with_options(c: &mut Criterion) {
    c.bench_function("build_with_options", |b| {
        b.iter(|| {
            Command::new("options")
                .option("-n --name [name]", "Optional name")
                .option("-f --file-path <path>", "File path");
        })
    });
}

fn build_with_partial_args(c: &mut Criterion) {
    c.bench_function("build_with_partial_args", |b| {
        b.iter(|| {
            Command::new("partial")
                .option("--example <path>", "Path to example")
                .option("-V", "Partial flag");
        })
    });
}

criterion_group!(
    benches,
    build_with_args,
    build_with_flags,
    build_with_options,
    build_with_partial_args
);
criterion_main!(benches);
