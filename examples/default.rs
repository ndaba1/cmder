use cmder::Program;

fn main() {
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

    program.parse();
}
