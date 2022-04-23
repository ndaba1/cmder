use cmder::core::new_program::Program;

fn main() {
    let mut program = Program::new();

    program
        .author("vndaba")
        .version("0.1.0")
        .description("A test CLI App")
        .bin_name("echo");

    program
        .subcommand("test")
        .argument("<app-name>", "Pass the name of the app to test")
        .alias("t")
        .description("A test subcommand")
        .option("-a --all", "Run all the configured tests")
        .option("-q --quiet", "Don't show tests output")
        .build(&mut program);
}
