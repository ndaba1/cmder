use cmder::{core::new_program::Program, ParserMatches};

#[test]
fn test_complex_api() {
    let mut program = Program::new();

    program
        .author("vndaba")
        .version("0.1.0")
        .description("A test for subcommands")
        .bin_name("complex");

    let img_cmd = program.subcommand("image");

    img_cmd
        .subcommand("build")
        .argument("<path>", "The path to the dockerfile")
        .alias("b")
        .description("Build a docker image from provided context")
        .option("-q --quiet", "Supress output when building")
        .action(|_m| {});

    img_cmd
        .subcommand("prune")
        .argument("[image-name]", "The name of the image to prune")
        .alias("p")
        .option("-a --all", "Remove all unused images")
        .option("-p --port <port-number>", "Prune containers on given ports")
        .description("Remove the provided image or all unused images")
        .action(prune_cmd_cb);

    img_cmd
        .alias("i")
        .description("A subcommand housing image functionality");

    program.parse_from(vec![
        "complex",
        "i",
        "prune",
        "image-one",
        "-a",
        "-p",
        "8080",
        "-p",
        "5053",
    ]);
}

fn prune_cmd_cb(m: ParserMatches) {
    let cmd = m.get_matched_cmd();

    assert!(cmd.is_some());

    let cmd = cmd.unwrap();
    assert!(cmd.get_parent().is_some());
    assert!(cmd.get_parent().unwrap().get_parent().is_some());
    assert!(m.contains_flag("-a"));

    assert_eq!(cmd.get_name(), "prune");
    assert_eq!(m.get_arg("[image-name]"), Some("image-one".to_string()));
    assert_eq!(m.get_instances_of("<port-number>"), vec!["8080", "5053"])
}
