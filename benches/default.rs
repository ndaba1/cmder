use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_old(c: &mut Criterion) {
    use cmder::Program;
    c.bench_function("build old", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program
                .author("vndaba")
                .bin_name("bench")
                .description("Benchmarks")
                .version("0.1.0");

            program
                .command("empty")
                .option("-x --extra", "Something extra")
                .option("-x --extra", "Something extra")
                .build(&mut program);
        })
    });
}

pub fn build_new(c: &mut Criterion) {
    use cmder::core::new_program::Program;
    c.bench_function("build new", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program
                .author("vndaba")
                .bin_name("bench")
                .description("Benchmarks")
                .version("0.1.0");

            program
                .subcommand("empty")
                .option("-x --extra", "Something extra")
                .option("-x --extra", "Something extra")
                .build(program.s());
        })
    });
}

criterion_group!(benches, build_old, build_new);
criterion_main!(benches);
