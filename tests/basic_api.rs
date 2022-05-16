use cmder::{
    self, ParserMatches, {Command, Program},
};

fn create_default_program() -> Command<'static> {
    let mut program = Program::new();

    program
        .author("vndaba")
        .version("0.1.0")
        .description("A test CLI App")
        .bin_name("simple");

    program
        .subcommand("test")
        .argument("<app-name>", "Pass the name of the app to test")
        .alias("t")
        .description("A test subcommand")
        .option("-a --all", "Run all the configured tests")
        .option("-q --quiet", "Don't show tests output")
        .action(basic_cmd_cb);

    program
}

#[test]
fn test_basic_api() {
    let mut program = create_default_program();
    program.parse_from(vec!["simple", "t", "app", "--all", "--quiet"]);
}

fn basic_cmd_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(m.contains_flag("--quiet"));
    assert!(m.contains_flag("--all"));
    assert!(cmd.is_some());

    assert_eq!(m.get_arg("<app-name>").unwrap(), "app");
    assert_eq!(cmd.unwrap().get_name(), "test");
}
