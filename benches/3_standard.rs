use cmder::Program;
use criterion::{criterion_group, criterion_main, Criterion};

fn build_std_with_args(c: &mut Criterion) {
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

fn build_std_with_subcmds(c: &mut Criterion) {
    c.bench_function("build_standard_with_subcmds", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program
                .bin_name("std")
                .author("vndaba")
                .version("0.1.0")
                .description("std");

            program
                .subcommand("one")
                .description("first cmd")
                .alias("first")
                .argument("<val>", "Some value");

            program
                .subcommand("two")
                .description("second cmd")
                .alias("second")
                .argument("<val>", "Some value");
        })
    });
}

criterion_group!(benches, build_std_with_args, build_std_with_subcmds);
criterion_main!(benches);
