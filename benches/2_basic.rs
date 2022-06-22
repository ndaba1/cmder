use cmder::{CmderFlag, CmderOption, Command};
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

fn build_w_flag_builder(c: &mut Criterion) {
    c.bench_function("build_w_flag_builder", |b| {
        b.iter(|| {
            Command::new("flags")
                .add_flag(CmderFlag::new("extra").help("Something extra").short('x'))
                .add_flag(CmderFlag::new("verbose").help("Verbosity").short('v'));
        })
    });
}

fn build_with_options(c: &mut Criterion) {
    c.bench_function("build_with_options", |b| {
        b.iter(|| {
            Command::new("options")
                .option("-n --name [name]", "Optional name")
                .required_option("-f --file-path <path>", "File path");
        })
    });
}

fn build_w_opt_builder(c: &mut Criterion) {
    c.bench_function("build_w_opt_builder", |b| {
        b.iter(|| {
            Command::new("options")
                .add_option(
                    CmderOption::new("name")
                        .help("optional name")
                        .short('n')
                        .argument("[name]"),
                )
                .add_option(
                    CmderOption::new("file-path")
                        .short('f')
                        .argument("<path>")
                        .required(true),
                );
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
    build_w_flag_builder,
    build_with_options,
    build_w_opt_builder,
    build_with_partial_args
);
criterion_main!(benches);
