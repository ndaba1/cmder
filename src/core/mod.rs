/// The events module contains the EventEmitter functionality and an enum containing all the possible events that can be emitted. It also contains associative methods to `emit` and `listen` to events in the Event enum. When a new instance of a program is created, the program contains an event_emitter instance.
mod events;

/// The settings houses the functionality used to configure the settings of the program which determine the default behavior of the program.
mod settings;

mod program;

mod errors;

pub use errors::{CmderError, CmderResult};
pub use events::{Event, EventConfig, EventEmitter};
pub use program::{Command, Program};
pub use settings::Setting;
