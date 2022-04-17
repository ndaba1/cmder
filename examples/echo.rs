use cmder::Program;

fn main() {
    let mut program = Program::new();

    program
        .author("vndaba")
        .bin_name("echo")
        .description("A simple echo example");

    program
        .argument("<TEXT...>", "The text to echo")
        .option("-n --newline", "Whether to add a newline at the end")
        .action(|values, options| {
            // We can unwrap the value because we know it's there, an error would have been thrown if it wasn't
            let text = values.get("TEXT").unwrap().clone();

            if options.contains_key("newline") {
                println!("{}", text);
            } else {
                print!("{}", text);
            }
        });

    program.parse();
}
