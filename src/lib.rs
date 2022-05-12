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

//! Themes can also be customized by defining your own color palette to be used when printing out information.

/// The parser modules contains all functionality for parsing arguments at the command level. It contains some submodules all involved in parsing arguments and flags.
mod parse;

/// A module housing all the core functionality of the library such as events, the program module itself, settings and other core functionality
mod core;

/// A module to house some utilities used by the crate itself.
mod utils;

/// The UI module houses the formatter module that is used to print to stdout and the themes module used to construct and define new themes.
mod ui;

pub use crate::core::{Command, Event, EventEmitter, Program, ProgramSettings, Setting};
pub use parse::ParserMatches;
pub use termcolor::Color;
pub use ui::{CustomPattern, Designation, Formatter, Pattern, PredefinedThemes, Theme};
