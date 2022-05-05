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
        .build(program.s());

    program
        .subcommand("image")
        .argument("<image-name>", "Enter the name of the image to start")
        .alias("i")
        .description("A command to start up images")
        .option("-p --port <port-number>", "The port to start the image on")
        .option("-a -all", "Perform the action for all images")
        .build(program.s());

    let mut temp_cmd = program.subcommand("temp");

    temp_cmd
        .subcommand("hello")
        .description("Some desc")
        .build(temp_cmd.s());

    temp_cmd
        .alias("tmp")
        .description("A command with subcmds")
        .build(program.s());

    // program.parse();
    dbg!(program);
}
