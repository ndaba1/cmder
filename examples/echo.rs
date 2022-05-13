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
        .action(cmd::echo_cmd_cb);

    program.parse();
}

mod cmd {
    use cmder::ParserMatches;

    pub fn echo_cmd_cb(m: ParserMatches) {
        let text = m.get_arg("<TEXT...>").unwrap();

        if m.contains_flag("-n") {
            println!("{text}")
        } else {
            print!("{text}")
        }
    }
}
