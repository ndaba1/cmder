use cmder::Program;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_standard_with_args(c: &mut Criterion) {
    c.bench_function("build_standard_with_args", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program
                .bin_name("std")
                .author("vndaba")
                .version("0.1.0")
                .description("std");

            program
                .argument("<value>", "some value")
                .argument("[opt]", "optional value")
                .option("-o --opt", "some option");
        })
    });
}

criterion_group!(benches, build_standard_with_args);
criterion_main!(benches);
