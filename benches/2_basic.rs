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

criterion_group!(benches, build_with_args, build_with_flags);
criterion_main!(benches);
