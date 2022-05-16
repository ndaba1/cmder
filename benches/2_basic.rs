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

criterion_group!(benches, build_with_args);
criterion_main!(benches);
