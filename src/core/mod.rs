/// The program module houses the the program struct and all its associated methods for manipulating and adjusting the program settings to customize the look and feel of your cli. It contains multiple `getters` and `setters` that can be used to get any desired values in the program.
///
/// The program module contains all functionality for parsing arguments at the program level. It resolves the target command then passes the cleaned argument to the cmd.parse() method.
mod program;

/// The events module contains the EventEmitter functionality and an enum containing all the possible events that can be emitted. It also contains associative methods to `emit` and `listen` to events in the Event enum. When a new instance of a program is created, the program contains an event_emitter instance.
mod events;

/// The settings houses the functionality used to configure the settings of the program which determine the default behavior of the program.
mod settings;

pub mod new_program;

pub use events::{Event, EventEmitter};
pub use program::Program;
pub use settings::ProgramSettings;
