use cmder::{Command, ParserMatches, Program};

fn create_default_program(cb: fn(ParserMatches)) -> Command<'static> {
    let mut program = Program::new();

    program
        .author("vndaba")
        .version("0.1.0")
        .description("A test for subcommands")
        .bin_name("complex");

    let img_cmd = program.subcommand("image");

    img_cmd
        .subcommand("prune")
        .argument("[image-name]", "The name of the image to prune")
        .alias("p")
        .option("-a --all", "Remove all unused images")
        .option("-p --port <port-number>", "Prune containers on given ports")
        .description("Remove the provided image or all unused images")
        .action(cb);

    img_cmd
        .alias("i")
        .description("A subcommand housing image functionality");

    program
}

#[test]
fn test_full_args() {
    let mut program = create_default_program(first_cb);
    program.parse_from(vec![
        "complex", "i", "prune", "cont-one", "-a", "-p=8080", "-p=5053", "--", "ng", "-pre",
    ]);
}

#[test]
fn test_options_syntax() {
    let mut program = create_default_program(first_cb);
    program.parse_from(vec![
        "complex", "i", "prune", "cont-one", "-a", "-p", "8080", "-p", "5053", "--", "ng", "-pre",
    ]);
}

fn first_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(cmd.is_some());

    let cmd = cmd.unwrap();
    assert!(cmd.get_parent().is_some());
    assert!(cmd.get_parent().unwrap().get_parent().is_some());
    assert!(m.contains_flag("-a"));

    assert_eq!(cmd.get_name(), "prune");
    assert_eq!(m.get_arg("[image-name]"), Some("cont-one".to_string()));
    assert_eq!(m.get_instances_of("<port-number>"), vec!["8080", "5053"]);
    assert_eq!(m.get_positional_args(), vec!["ng", "-pre"]);
}

#[test]
fn test_optional_args() {
    let mut program = create_default_program(second_cb);
    program.parse_from(vec!["complex", "i", "prune", "-a"]);
}

fn second_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(cmd.is_some());

    let cmd = cmd.unwrap();
    assert!(m.contains_flag("-a"));
    assert_eq!(cmd.get_name(), "prune");
    assert_eq!(m.get_arg("[image-name]"), None);
}
