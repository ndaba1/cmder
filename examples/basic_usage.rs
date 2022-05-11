use cmder::{ParserMatches, Program};

fn main() {
    let mut program = Program::new();

    program
        .author("vndaba")
        .description("A simple demo cli")
        .bin_name("demo");

    program
        .subcommand("greet")
        .argument("<name>", "Pass the name to say hello to")
        .alias("g")
        .description("Simply greets the provided name")
        .option("-c --custom <GREETING...>", "Pass a custom greeting to use")
        .action(cmd::greet_cb);

    program.parse();
}

mod cmd {
    use super::*;

    pub fn greet_cb(m: ParserMatches) {
        // we can safely unwrap the value since it it required and an error would have been thrown if no value was provided
        let name = m.get_arg("<name>").unwrap();

        let mut greeting = "Ahoy!".to_owned();
        // Check if any args for <GREETING> exist
        if let Some(custom_grtng) = m.get_option_arg("<GREETING...>") {
            greeting = custom_grtng;
        }

        println!("{greeting} {name}");
    }
}
