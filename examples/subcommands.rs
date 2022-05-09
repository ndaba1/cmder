#![allow(unused)]

use cmder::{
    construct_theme,
    core::{new_program::Program, Setting},
    Color, Event, ParserMatches, Pattern, PredefinedThemes, Theme,
};
/// This is an example of a program that uses commands which have subcommands themselves. The example used here emulates the docker cli. i.e
/// docker container ls, docker image ls, docker image build
fn main() {
    let mut program = Program::new();

    program
        .bin_name("docker")
        .author("vndaba")
        .description("An example of a program with subcommands");

    // The docker image example command
    let mut img_cmd = program.subcommand("image");

    // You can then chain the subcommand method then invoke the construct method as the final method.
    img_cmd
        .subcommand("ls")
        .alias("l")
        .option("-l --long", "Use the long listing method")
        .description("List all the docker images available");

    img_cmd
        .subcommand("build")
        .alias("b")
        .option("-q --quiet", "Supress output when building")
        .description("Build a docker image from provided context");

    img_cmd
        .subcommand("prune")
        .alias("p")
        .option("-a --all", "Remove all unused images")
        .description("Remove the provided image or all unused images");

    // The build method should always be invoked after all the subcommands have been constructed
    img_cmd
        .description("A command housing all the subcommands for image functionality")
        .alias("i");

    // The docker container example command
    let mut cont_cmd = program.subcommand("container");

    cont_cmd
        .subcommand("ls")
        .alias("l")
        .option("-l --long", "Use the long listing method")
        .option("-a --all", "List even stopped conatiners")
        .description("List all the docker containers available");

    cont_cmd
        .alias("cont")
        .description("A command housing all subcommands for containers");

    program
        .subcommand("tree")
        .argument("<SUB-COMMAND>", "The subcommand to print out the tree for")
        .description("A subcommand used for printing out a tree view of the command tree")
        .action(|m| {
            let cmd = m.get_matched_cmd().unwrap();
            let val = m.get_arg("<SUB-COMMAND>").unwrap();
            let parent = cmd.get_parent().unwrap();

            if let Some(cmd) = parent.find_subcommand(&val) {
                cmd.display_commands_tree();
            }
        });

    program.parse_from(vec!["docker", "tree", "image"]);
}
