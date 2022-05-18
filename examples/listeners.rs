use cmder::{Event, Program};

fn main() {
    let mut program = Program::new();

    program
        .bin_name("listeners")
        .version("0.1.0")
        .author("vndaba")
        .description("A simple example with event listeners");

    use Event::*;

    // Reacts to a specific event being emitted. If the program is not set to override the default listeners, the user-defined listeners get invoked after the default ones.
    program.on(OutputVersion, |cfg| {
        let program_ref = cfg.get_program();
        println!("Currently on version: {}", program_ref.get_version());
    });

    // The callback defined here gets invoked before all events, event before printing out help information
    program.before_all(|cfg| {
        let p_ref = cfg.get_program();
        println!("This program was authored by: {}", p_ref.get_author())
    });

    // Others include:
    program.after_all(|_cfg| println!("This will get printed after all events"));

    program.after_help(|_cfg| println!("This will only get printed after printing help"));

    program.before_help(|_cfg| println!("Gets printed before help"));

    program.parse();
}
