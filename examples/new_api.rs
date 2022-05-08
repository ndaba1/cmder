#![allow(unused)]

use cmder::{
    construct_theme,
    core::{new_program::Program, Setting},
    Color, Event, ParserMatches, Pattern, PredefinedThemes, Theme,
};

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
        .build();

    program
        .subcommand("image")
        .argument("<image-name>", "Enter the name of the image to start")
        .alias("i")
        .description("A command to start up images")
        .option("-p --port <port-number>", "The port to start the image on")
        .option("-a -all", "Perform the action for all images")
        .action(image_cmd_cb)
        .build();

    let mut temp_cmd = program.subcommand("temp");

    temp_cmd
        .subcommand("hello")
        .description("Some desc")
        .build();

    temp_cmd
        .alias("tmp")
        .description("A command with subcmds")
        .build();

    // program.set(Setting::EnableCommandSuggestion(false));
    program.set(Setting::ShowHelpOnError(true));
    program.set(Setting::ChoosePredefinedTheme(PredefinedThemes::Colorful));
    program.set(Setting::SetProgramPattern(Pattern::Legacy));
    // program.set(Setting::OverrideAllDefaultListeners(true));

    use Color::*;
    program.set(Setting::DefineCustomTheme(construct_theme!((
        Green, Magenta, Blue, Red, White
    ))));

    // program.before_all(|_cfg| {
    //     println!("Before all");
    // });

    // program.on(Event::UnknownCommand, |_cfg| {
    //     println!("Unknown command");
    // });

    // program.parse();
    // program.parse_from(vec!["image", "ls", "--", "-xc", "-pv"]);
    // program.parse_from(vec!["-v"]);
    dbg!(program);
}

fn image_cmd_cb(matches: ParserMatches) {
    dbg!(matches);
}
