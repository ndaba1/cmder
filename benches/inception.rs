use cmder::Program;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn five_level_inception(c: &mut Criterion) {
    c.bench_function("five_level_inception", |b| {
        b.iter(|| {
            Program::new()
                .subcommand("first")
                .description("The first level of inception")
                .subcommand("second")
                .description("The second inception level")
                .subcommand("third")
                .description("Third inception level")
                .subcommand("fourth")
                .description("The fourth level of inception")
                .subcommand("fifth")
                .description("The fifth level of inception");
        });
    });
}

criterion_group!(benches, five_level_inception);
criterion_main!(benches);
