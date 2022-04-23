use cmder::Program;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn build_empty(c: &mut Criterion) {
    c.bench_function("build empty", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program.command("empty").build(&mut program);
        })
    });
}

pub fn build_one(c: &mut Criterion) {
    c.bench_function("build one", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program
                .author("vndaba")
                .description("A simple demo cli")
                .bin_name("demo");

            program
                .command("greet <name>")
                .alias("g")
                .description("Simply greets the provided name")
                .option("-d --default", "Override the provided name")
                .option("-c --custom <GREETING...>", "Pass a custom greeting to use")
                .action(|values, options| {
                    let mut name = values.get("name").unwrap().as_str();

                    let greeting = if options.contains_key("GREETING") {
                        options.get("GREETING").unwrap().to_owned()
                    } else {
                        String::from("Ahoy!")
                    };

                    if options.contains_key("default") {
                        name = "Kemosabe";
                    }

                    println!("{} {}", greeting, name);
                })
                .build(&mut program);
        })
    });
}

criterion_group!(benches, build_empty, build_one);
criterion_main!(benches);
