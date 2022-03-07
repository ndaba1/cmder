//! A simple, lighweight crate to parse command line arguments. Inspired by its javascript equivalent, commander.js.
//!
//! This crate is relatively similar in syntax to the said library and is easy to get started with. It presents a builder interface to work with and can easily be extended to suit your needs.
//! The crate only offers a builder interface, no derive features, if you're looking such features or something more powerful, you should probably check out `clap`.
//!
//! There are three main constructs you need to be aware of when using this crate. `Program`, `Event` and `Cmd`. A program is what you actually create. An instance of your, well program, It contains a few fields, some for metadata such as `version`, `author` and `about` and the most important field, the `cmds` field which is a vector of `Cmds`
//! .
//! Cmds on the other hand are exactly what they sound like. They contain all the functionality for a given command.
//! Events are a construct that can be used to extend the program. As will be demonstrated below.
//!
//! There are other constructs such as `Themes` and `Patterns` that can also be ussed to extend and customize the default program behavior.
//!
//! The following is a full-fleged example of how the crate can be used:
//!
//!
//! ```should_panic
//!
//!
//! use commander_rs::{Program, Event, Pattern, PredefinedThemes};
//!
//! let mut program = Program::new();
//!
//! program.version("0.20").description("Some awesome cli");
//!
//! program
//!     .add_cmd()
//!     .command("test <app-name> [optional-val]")
//!     .alias("-t")
//!     .describe("A test command")
//!     .option("-a --all", "Test everything in the app")
//!     .option(
//!         "-p --priority-level <priority-value>",
//!         "Set the priority level when testing",
//!     )
//!     .action(|vals, config| {
//!         dbg!(vals);
//!         dbg!(config);
//!     })
//!     .build(&mut program);
//!
//! program
//!     .add_cmd()
//!     .command("new <app-name>")
//!     .alias("n")
//!     .describe("A command for creating new projects.")
//!     .option("-g --git", " Whether to initialized the project with git.")
//!     .option("-s  --skip ", " Skips installing the dependencies")
//!      .action(|vals, config| {
//!          dbg!(vals);
//!          dbg!(config);
//!      })
//!      .build(&mut program);
//!
//!
//!  program.on(Event::MissingArgument, |p, v| {
//!      let msg = format!("You are missing a required argument: {}", v);
//!      p.output_help(&msg);
//!  });
//!
//!
//!  program.on(Event::OutputHelp, |_p, _v| {
//!      println!("The help listener acts differently, help still gets printed out")
//!  });
//!
//!
//!  program.on(Event::OutputVersion, |p, v| {
//!      println!("You are using version {} of my program", v);
//!      println!("This program was authored by: {}", p.author);
//!  });
//!
//!
//!  program.on(Event::OutputVersion, |_p, _v| {
//!      println!("A single event can have multiple callbacks, this will get invoked after the one above it.");
//!  });
//!
//!
//!  program.on(Event::UnknownCommand, |_p, v| {
//!      println!("Your command was not recognized {}", v);
//!  });
//!
//!  program.set_pattern(Pattern::Standard);
//!
//!  program.set_theme(PredefinedThemes::Plain);
//!
//!  program.parse();
//! ```
//! The program.parse() method should be the very last thing to call. It doesn't take any args. If any custom event listeners are called after the parse method, they will be ignored.
//!
//! Assuming we have a binary crate called `bolt` with the above code in its main function and we run the following command:
//! ```bash
//! bolt test appOne -a -p 1
//! ```
//!
//! When the program gets executed, the closure in the action method gets invoked and we get the following output:
//!
//! ```bash
//! [src\bin\bolt.rs:19] vals = {  
//!     "app_name": "appOne",       
//! }
//! [src\bin\bolt.rs:20] config = {
//!    "priority_value": "1",      
//!    "all": "true",
//! }
//! ```
//!
//! Any parameters starting with angle brackets < > are marked as required and one with [ ] are not. Hence, in the above snippets, we did not include the optional-val and the code worked well without throwing any errors. However, we can include the optional val as shown below and the code will still work.
//!
//! ```bash
//! bolt test appOne optionalName -a -p 1
//! ```
//!
//! And we get the following output:
//!
//! ```bash
//! [src\bin\bolt.rs:19] vals = {  
//!     "app_name": "appOne",       
//!     "optional_val": "optionalName",
//! }
//! [src\bin\bolt.rs:20] config = {
//!     "all": "true",
//!     "priority_value": "1",      
//! }
//! ```
//!
//! There are a few important things to note:
//! - For any flags that receive a parameter, the name of the param will be returned in the hashmap containing flags metadata, while flags that dont have any input will simply have the value true.
//! - The names of the values and the flag params get transformed into a rust-friendly manner(snake_cased)
//! - Required parameters are enclosed in angle brackets while optional ones in square brackets
//!
//! The crate can also be easily extended, to override or modify the default behavior. This is done by the use of `Event Listeners` which are simply closures of the type Listener where:
//! `type Listener = fn(&Program, String) -> ();`
//!
//! As shown, these listeners take in a ref to the program and a string which is a different value depending on the event being referred to.
//! These listeners are set by invoking the .on() method on the instance of the program.
//!
//! For instance, when the -v flag is passed to the program, the program simply prints the version and exits. This can easily be modified in the following way:
//!
//! ```
//! use commander_rs::{Event, Program};
//!
//! let mut program = Program::new();
//!
//! //...
//!
//! program.on(Event::OutputVersion, |p, v| {
//!     println!("You are using version {} of my program", v);
//!     println!("This program was authored by: {}", p.author);
//! });
//!
//! //...
//!
//! ```
//!
//! In the above event, the string that gets passed to the callback closure is the actual version of the program. Do note that when you set a custom listener, the default behavior is overriden and you have to perform all the desired actions. This is true for all events apart from the `Event::OutputHelp`. In this special case, the help informartion first gets printed out, then your callbacks are invoked.
//!
//! All available events are found in the `Event` enum in the `program.rs` module.
//!
//! Here are some more examples of using event listeners:
//!
//! ```
//! use commander_rs::{Event, Program};
//!
//! let mut program = Program::new();
//!
//! //...
//!
//! program.on(Event::MissingArgument, |p, v| {
//!     // the value returned contains the name of the resolved command, and the name of the missing argument
//!     let params: Vec<_> = v.split(',').collect();
//!
//!     // the missing argument name is the second value
//!     let msg = format!("You are missing a required argument: {}", params[1]);
//!
//!     // you can use the `get_cmd` method to get a ref to the command and invoke the `output_command_help`
//!     // this is equivalent to the default behavior
//!     let cmd = p.get_cmd(&params[0]).unwrap();
//!     cmd.output_command_help(p, &msg);
//! });
//!
//! //...
//!
//!
//! program.on(Event::OutputHelp, |_p, _v| {
//!     // Here v is an empty string since there's nothing to pass
//!     // The help listener acts differently, your callbacks are invoked after the help info is printed out
//!     println!("This line gets printed out after all the help information")
//! });
//!
//!
//! //...
//!
//! program.on(Event::UnknownCommand, |_p, v| {
//!     // The value of v is the unrecognized command
//!
//!     // If the command is unrecognized, you can redirect the command elsewhere
//!     let args: Vec<String> = std::env::args().collect();
//!     custom_handler(&args);
//!
//!     // Alternatively, you can define a custom function to suggest commands:
//!     suggest_commands(&v);
//!
//!     // Or you could simply print out a warning, as is the default behavior:
//!     println!("Unknown Command: {}", v);
//!
//! });
//!
//! fn custom_handler(args: &Vec<String>) {
//!     // custom handling
//! }
//!
//! fn suggest_commands(_cmd: &String) {
//!     // suggest commands logic
//! }
//!
//! //...
//!
//! ```
//!
//! Apart from customizing the default behavior of the program, you can also customize the look and feel by using Patterns and Themes.
//!
//! Patterns refer to how your program presents and outputs help information. The default pattern is the `Legacy` pattern which is how most CLI's are presented
//!
//! The legacy pattern looks as shown below:
//!
//! ```bash
//! Some awesome prog
//!
//! USAGE:
//!    bolt <COMMAND> [options]
//!
//! OPTIONS:
//!    -h, --help           Output help for the program
//!    -v, --version        Output the version info for the program
//!
//! COMMANDS:
//!    test | -t            A test command
//!    new | n              A command for creating new projects.
//! ```
//!
//! The `Standard` pattern appears as shown below:
//!
//! ```bash
//! Some awesome prog
//!
//! USAGE:
//!    bolt <COMMAND> [options]
//!
//! OPTIONS:
//!    -h, --help
//!    Output help for the program
//!
//!    -v, --version
//!    Output the version info for the program
//!
//! COMMANDS:
//!    test | -t, <app-name> [optional-val]   
//!    A test command
//!
//!    new | n, <app-name>
//!    A command for creating new projects.
//! ```
//!
//! Themes can also be customized by defining your own color palette to be used when printing out information.

/// The parser modules contains all functionality for parsing arguments at the command level. It contains some submodules all involved in parsing arguments and flags.
pub mod parser;

/// The program module houses the the program struct and all its associated methods for manipulating and adjusting the program settings to customize the look and feel of your cli. It contains multiple `getters` and `setters` that can be used to get any desired values in the program.
///
/// The program module contains all functionality for parsing arguments at the program level. It resolves the target command then passes the cleaned argument to the cmd.parse() method.
pub mod program;

/// The events module contains the EventEmitter functionality and an enum containing all the possible events that can be emitted. It also contains associative methods to `emit` and `listen` to events in the Event enum. When a new instance of a program is created, the program contains an event_emitter instance.
pub mod events;

/// A module to house some utilities used by the crate itself.
pub mod utils;

pub mod ui;

pub use events::{Event, EventEmitter};
pub use program::Program;
pub use termcolor::Color;
pub use ui::{Designation, Formatter, FormatterRules, Pattern, PredefinedThemes, Theme};
