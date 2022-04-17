use cmder::Program;

/// This is an example of a program that uses commands which have subcommands themselves. The example used here emulates the docker cli. i.e
/// docker container ls, docker image ls, docker image build
fn main() {
    let mut program = Program::new();

    program
        .bin_name("docker")
        .author("vndaba")
        .description("An example of a program with subcommands");

    let mut img_cmd = program.command("image");

    img_cmd
        .subcommand("ls")
        .alias("l")
        .option("-l --long", "Use the long listing method")
        .description("List all the docker images available")
        .action(|v, o| {
            dbg!(v, o);
        })
        .construct(&mut img_cmd);

    img_cmd
        .subcommand("build <path>")
        .alias("b")
        .option("-q --quiet", "Supress output when building")
        .description("Build a docker image from provided context")
        .action(|v, o| {
            dbg!(v, o);
        })
        .construct(&mut img_cmd);

    img_cmd
        .subcommand("prune [image-name]")
        .alias("p")
        .option("-a --all", "Remove all unused images")
        .description("Remove the provided image or all unused images")
        .action(|v, o| {
            dbg!(v, o);
        })
        .construct(&mut img_cmd);

    // The build method should always be invoked after all the subcommands have been constructed
    img_cmd
        .description("A command housing all the subcommands for image functionality")
        .alias("i")
        .build(&mut program);

    let mut cont_cmd = program.command("container");

    cont_cmd
        .subcommand("ls")
        .alias("l")
        .option("-l --long", "Use the long listing method")
        .option("-a --all", "List even stopped conatiners")
        .description("List all the docker containers available")
        .action(|v, o| {
            dbg!(v, o);
        })
        .construct(&mut cont_cmd);

    cont_cmd
        .alias("cont")
        .description("A command housing all subcommands for containers")
        .build(&mut program);

    program.parse();
}
