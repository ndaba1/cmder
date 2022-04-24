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

pub fn build_new(c: &mut Criterion) {
    use cmder::core::new_program::Program;
    c.bench_function("build new", |b| {
        b.iter(|| {
            let mut program = Program::new();

            program
                .author("vndaba")
                .version("0.1.0");
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

criterion_group!(benches, build_old, build_new);
criterion_main!(benches);
