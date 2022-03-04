use std::collections::HashMap;

use super::Program;

/// A simple type to be used to pass callbacks to the .action() method on a command.
type Listener = fn(&Program, String) -> ();

/// The EventEmitter struct holds all functionality for emitting and receiving all events occurring in the program.
/// It contains only a single field being the listeners themselves.
pub struct EventEmitter {
    /// The listeners field is simply a hashmap with keys containing Event variants and the values containing a vector of listeners. Whenever an event is emitted, The listeners hashmap is queried for any callbacks for the said event and if any are found, then they are executed sequentially.
    listeners: HashMap<Event, Vec<Listener>>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Event {
    /// This event gets triggered when a required argument is missing from the args passed to the cli. The string value passed to this listener contains two values, the name of the matched command, and the name of the missing argument, comma separated.
    /// The callbacks set override the default behavior
    MissingArgument,

    /// This is similar to the `MissingArgument` argument variant except it occurs when the missing argument is for an option. The string value passed is the name of the missing argument.
    ///The callbacks set override the default behavior
    OptionMissingArgument,

    /// This gets triggered when the method output_command_help is called on a command. The value that gets passed to it is the name of the command that the method has been invoked for.
    OutputCommandHelp,

    /// This gets called anytime the output_help method is called on the program. The value passed here is an empty string and the callbacks do not override the default behavior
    OutputHelp,

    /// This event occurs when the output_version method gets called on the program instance. The value passed to it is the version of the program. It also overrides the default behavior
    OutputVersion,

    /// An event that occurs when the passed command could not be matched to any of the existing commands in the program. The value passed to it is the name of the unrecognized command.
    /// The callbacks set override the default behavior
    UnknownCommand,

    /// This occurs when an unknown flag or option is passed to the program, the value passed to the callback being the unknown option and also overrides the default program behavior.
    UnknownOption,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Receives an event and the actual listener to be set then matches on the listener and adds to the listener to its desired vector.
    pub fn on(&mut self, event: Event, callback: Listener) {
        use Event::*;

        match event {
            MissingArgument => {
                self.add_listener(MissingArgument, callback);
            }
            OptionMissingArgument => {
                self.add_listener(OptionMissingArgument, callback);
            }
            UnknownCommand => {
                self.add_listener(UnknownCommand, callback);
            }
            UnknownOption => {
                self.add_listener(UnknownOption, callback);
            }
            OutputHelp => {
                self.add_listener(OutputHelp, callback);
            }
            OutputCommandHelp => {
                self.add_listener(OutputCommandHelp, callback);
            }
            OutputVersion => {
                self.add_listener(OutputVersion, callback);
            }
        }
    }

    /// This method is called when events in the program occur. It simply checks for any listeners and then executes them in a sequential manner.
    pub fn emit(&self, program: &Program, event: Event, param: String) {
        if self.listeners.contains_key(&event) {
            let callbacks = self.listeners.get(&event).unwrap();
            for cb in callbacks {
                cb(program, param.clone())
            }
            std::process::exit(1)
        }
    }

    // This method retrives the vector of existing callbacks if any and pushes the new listener to the vector
    fn add_listener(&mut self, event: Event, callback: fn(&Program, String) -> ()) {
        let existing = self.listeners.get(&event);

        match existing {
            Some(values) => {
                let mut new_cbs = vec![];
                for cb in values.clone() {
                    new_cbs.push(cb)
                }
                new_cbs.push(callback);
                self.listeners.insert(event, new_cbs);
            }
            None => {
                self.listeners.insert(event, vec![callback]);
            }
        }
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}
