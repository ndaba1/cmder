//! A simple, lighweight crate to parse command line arguments. Inspired by its javascript equivalent, commander.js.
//!
//! This crate is relatively similar in syntax to the said library and is easy to get started with. It presents a builder interface to work with and can easily be extended to suit your needs.
//! The crate only offers a builder interface, no derive features, if you're looking such features or something more powerful, you should probably check out `clap`.
//!
//! Constructs used within the crate include:
//! - Command, which is exactly what it sounds like
//! - Program which is a command marked as the entrypoint
//! - Flags and Options(flags that take arguments)
//! - Themes that control the color choices used in the program
//! - Patterns which control how output is formatted on the terminal
//!
//! The following is a full-fleged example on crate usage:
//! ```
//! use cmder::{Program, Event, Setting, Pattern, PredefinedThemes};
//!
//! let mut program = Program::new();
//!
//! program
//!     .author("vndaba")
//!     .description("An example CLI")
//!     .version("0.1.0")
//!     .bin_name("example");
//!
//! // Subcommands
//! program
//!     .subcommand("demo")
//!     .argument("<value>", "Some required value")
//!     .alias("d")
//!     .option("-f", "Some flag")
//!     .option("-n --name <value>", "Some option")
//!     .description("A demo subcommand")
//!     .action(|matches|{dbg!(matches);});
//!
//! // ...
//!
//! // Event listeners
//! program.before_all(|cfg| {
//!     let p_ref = cfg.get_program();
//!     println!("This program was authored by: {}", p_ref.get_author())
//! });
//!
//! // ...
//!
//! // Program settings
//! program.set(Setting::ShowHelpOnAllErrors(true));
//! program.set(Setting::ChoosePredefinedTheme(PredefinedThemes::Colorful));
//! program.set(Setting::SetProgramPattern(Pattern::Standard));
//! program.set(Setting::OverrideAllDefaultListeners(true));
//!
//! program.parse();
//! ```

/// The parser modules contains all functionality for parsing arguments . It contains some submodules all involved in parsing arguments and flags.
mod parse;

/// A module housing all the core functionality of the library such as events, errors, settings and other core functionality
mod core;

/// A module to house some utilities used by the crate itself.
mod utils;

/// The UI module houses the formatter module that is used to print to stdout and the themes module used to construct and define new themes.
mod ui;

pub use crate::core::{Command, Event, EventEmitter, Program, ProgramSettings, Setting};
pub use parse::ParserMatches;
pub use termcolor::Color;
pub use ui::{CustomPattern, Designation, Formatter, Pattern, PredefinedThemes, Theme};
