use cmder::{ParserMatches, Program};

#[test]
fn test_args_api() {
    let mut program = Program::new();

    program
        .bin_name("yargs")
        .argument("<TEXT...>", "The text to print out")
        .description("A test for args parsing")
        .option("-n --newline", "Whether to print a newline after the text")
        .action(args_cmd_cb);

    program.parse_from(vec!["yargs", "hello", "world", "--newline"]);
}

fn args_cmd_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(cmd.is_some());

    let cmd = cmd.unwrap();
    assert!(cmd.get_name() == "yargs");
    assert!(cmd.get_arguments().len() == 1);
    assert!(cmd.get_flags().len() == 3);
    assert!(m.contains_flag("--newline"));
    assert!(m.get_arg("<TEXT...>").unwrap() == "hello world");
}
