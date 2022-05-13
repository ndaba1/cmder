use cmder::{
    self, ParserMatches, {Command, Program},
};

fn create_default_program(cb: fn(ParserMatches)) -> Command<'static> {
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
        .action(cb);

    program
}

#[test]
fn test_basic_api() {
    let mut program = create_default_program(basic_cmd_cb);
    program.parse_from(vec!["simple", "t", "app", "--all", "--quiet", "--help"]);
}

#[test]
fn test_cmd_metadata() {
    let mut program = create_default_program(cmd_metadata_cb);
    program.parse_from(vec!["simple", "test", "app"]);
}

fn basic_cmd_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(m.contains_flag("--quiet"));
    assert!(m.contains_flag("--all"));
    assert!(m.contains_flag("--help"));
    assert!(cmd.is_some());

    assert_eq!(m.get_arg("<app-name>").unwrap(), "app");
    assert_eq!(cmd.unwrap().get_name(), "test");
}

fn cmd_metadata_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(cmd.is_some());

    let cmd = cmd.unwrap();
    assert!(cmd.get_parent().is_some(), "No parent found");

    assert_eq!(cmd.get_name(), "test");
    assert_eq!(cmd.get_alias(), "t");
    assert_eq!(cmd.get_arguments().len(), 1);
    assert_eq!(cmd.get_flags().len(), 3); // the two created flags plus the help flag
    assert_eq!(cmd.get_description(), "A test subcommand")
}
