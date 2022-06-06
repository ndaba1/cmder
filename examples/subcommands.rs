#![allow(unused)]

use cmder::{
    Color, Event, ParserMatches, Pattern, PredefinedTheme, Theme, {Program, Setting},
};
/// This is an example of a program that uses commands which have subcommands themselves. The example used here emulates the docker cli. i.e
/// docker container ls, docker image ls, docker image build
fn main() {
    let mut program = Program::new();

    program
        .bin_name("docker")
        .author("vndaba")
        .description("An example of a program with subcommands");

    // Bind the newly created subcmd to a variable
    let mut img_cmd = program.subcommand("image");

    img_cmd
        .description("A command housing all the subcommands for image functionality")
        .alias("i");

    // You can then chain the subcommand method to add subcmds to the subcmd.
    img_cmd
        .subcommand("ls")
        .alias("l")
        .option("-l --long", "Use the long listing method")
        .description("List all the docker images available")
        .action(cmd::img_ls_cb);

    img_cmd
        .subcommand("build")
        .argument("<path>", "The path to the build context")
        .alias("b")
        .option("-q --quiet", "Supress output when building")
        .description("Build a docker image from provided context")
        .action(cmd::img_build_cb);

    img_cmd
        .subcommand("prune")
        .argument("<image-name>", "The name of the image to prune")
        .alias("p")
        .option("-a --all", "Remove all unused images")
        .description("Remove the provided image or all unused images")
        .action(cmd::img_prune_cb);

    // The docker container example command
    let mut cont_cmd = program.subcommand("container");

    cont_cmd
        .alias("cont")
        .description("A command housing all subcommands for containers");

    cont_cmd
        .subcommand("ls")
        .alias("l")
        .option("-l --long", "Use the long listing method")
        .option("-a --all", "List even stopped conatiners")
        .description("List all the docker containers available")
        .action(cmd::cont_ls_cb);

    cont_cmd
        .subcommand("start")
        .alias("s")
        .option(
            "-p --port <port-number>",
            "Add port mappings from container to host",
        )
        .description("Start running a given container")
        .option("-i --interactive", "Start container in interactive shell")
        .option("-d --detached", "Start container in detached state")
        .action(cmd::cont_start_cb);

    program.set(Setting::ShowCommandAliases, true);

    program.parse();
}

mod cmd {
    use cmder::ParserMatches;

    pub fn img_build_cb(_m: ParserMatches) {
        // Command logic goes here
        println!("Image build command called");
    }

    pub fn img_prune_cb(_m: ParserMatches) {
        // Command logic goes here
        println!("Image prune command called");
    }

    pub fn img_ls_cb(_m: ParserMatches) {
        // Command logic goes here
        println!("Image list command called");
    }

    pub fn cont_start_cb(_m: ParserMatches) {
        // Command logic goes here
        println!("Container start command called");
    }

    pub fn cont_ls_cb(_m: ParserMatches) {
        // Command logic goes here
        println!("Container list command called");
    }
}
