use cmder::{self, core::new_program::Program, ParserMatches};

#[test]
fn test_new_api() {
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
        .action(test_cmd)
        .build();

    program.parse_from(vec!["test", "app", "--all", "--quiet", "--help"]);
}

fn test_cmd(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(cmd.is_some());
    assert_eq!(cmd.unwrap().get_name(), "test");
    assert_eq!(m.get_arg("<app-name>").unwrap(), "app");
}
