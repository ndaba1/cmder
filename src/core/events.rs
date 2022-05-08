#![allow(unused)]
use std::{collections::HashMap, fmt::Debug};

use super::{new_program::Command, Program};

#[derive(Clone, Debug)]
pub struct EventConfig<'e> {
    args: Vec<String>,
    arg_count: usize,
    error_string: String,
    exit_code: usize,
    event_type: Event,
    matched_cmd: Option<&'e Command<'e>>,
    additional_info: &'e str,
    program_ref: Command<'e>,
}

impl<'a> EventConfig<'a> {
    // Getters
    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn get_event(&self) -> Event {
        self.event_type
    }

    pub fn get_program(&self) -> &Command<'a> {
        &self.program_ref
    }

    pub fn get_exit_code(&self) -> usize {
        self.exit_code
    }

    pub fn get_error_str(&self) -> &str {
        self.error_string.as_str()
    }

    pub fn get_matched_cmd(&self) -> Option<&Command<'a>> {
        self.matched_cmd
    }

    // Setters
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn arg_c(mut self, count: usize) -> Self {
        self.arg_count = count;
        self
    }

    pub fn exit_code(mut self, code: usize) -> Self {
        self.exit_code = code;
        self
    }

    pub fn error_str(mut self, val: String) -> Self {
        self.error_string = val;
        self
    }

    pub fn set_event(mut self, event: Event) -> Self {
        self.event_type = event;
        self
    }

    pub fn set_matched_cmd(mut self, cmd: &'a Command<'a>) -> Self {
        self.matched_cmd = Some(cmd);
        self
    }

    pub fn info(mut self, info: &'a str) -> Self {
        self.additional_info = info;
        self
    }

    pub fn program(mut self, p_ref: Command<'a>) -> Self {
        self.program_ref = p_ref;
        self
    }
}

impl<'d> Default for EventConfig<'d> {
    fn default() -> Self {
        Self {
            additional_info: "",
            error_string: "".into(),
            arg_count: 0,
            args: vec![],
            event_type: Event::OutputHelp,
            exit_code: 0,
            matched_cmd: None,
            program_ref: Command::new("none"),
        }
    }
}

pub type NewListener = fn(EventConfig) -> ();

#[derive(Clone)]
pub struct NewEventEmitter {
    listeners: HashMap<Event, Vec<(NewListener, i32)>>,
}

impl Debug for NewEventEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self.listeners.keys()))
    }
}

impl NewEventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn on(&mut self, event: Event, cb: NewListener, pstn: i32) {
        match self.listeners.get(&event) {
            Some(lstnrs) => {
                let mut temp = vec![];

                for l in lstnrs {
                    temp.push((l.0, l.1));
                }
                temp.push((cb, pstn));

                self.listeners.insert(event, temp);
            }
            None => {
                self.listeners.insert(event, vec![(cb, pstn)]);
            }
        };
    }

    pub fn emit(&self, cfg: EventConfig) {
        let event = cfg.get_event();

        if let Some(lstnrs) = self.listeners.get(&event) {
            let mut lstnrs = lstnrs.clone();

            lstnrs.sort_by(|a, b| a.1.cmp(&b.1));

            for (cb, idx) in lstnrs.iter() {
                cb(cfg.clone());
            }

            std::process::exit(cfg.get_exit_code() as i32);
        }
    }

    pub(crate) fn insert_before_all(&mut self, cb: NewListener) {
        match self.listeners.len() {
            0 => self.on_all(cb, -5),
            _ => {
                for lstnr in self.listeners.clone() {
                    self.on(lstnr.0, cb, -5); // Insert before all listeners
                }
            }
        }
    }

    pub(crate) fn insert_after_all(&mut self, cb: NewListener) {
        match self.listeners.len() {
            0 => self.on_all(cb, 5),
            _ => {
                for lstnr in self.listeners.clone() {
                    self.on(lstnr.0, cb, 5); // Insert after all listeners
                }
            }
        }
    }

    pub(crate) fn on_all(&mut self, cb: NewListener, pstn: i32) {
        use Event::*;

        self.on(OutputHelp, cb, pstn);
        self.on(OutputVersion, cb, pstn);
        self.on_all_errors(cb, pstn)
    }

    pub(crate) fn on_all_errors(&mut self, cb: NewListener, pstn: i32) {
        use Event::*;

        self.on(MissingArgument, cb, pstn);
        self.on(OptionMissingArgument, cb, pstn);
        self.on(UnknownCommand, cb, pstn);
        self.on(UnknownOption, cb, pstn);
    }

    pub(crate) fn rm_lstnr_idx(&mut self, event: Event, val: i32) {
        if let Some(lstnrs) = self.listeners.get_mut(&event) {
            for (idx, lstnr) in lstnrs.clone().iter().enumerate() {
                if lstnr.1 == val {
                    lstnrs.remove(idx);
                }
            }
        }
    }
}

impl Default for NewEventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple type to be used to pass callbacks to the .action() method on a command.
type Listener = fn(&Program, String) -> ();

#[derive(Clone)]
/// The EventEmitter strPartialEq, Eq, Hash, uct holds all functionality for emitting and receiving all events occurring in the program.
/// It contains only a single field being the listeners themselves.
pub struct EventEmitter {
    /// The listeners field is simply a hashmap with keys containing Event variants and the values containing a vector of listeners. Whenever an event is emitted, The listeners hashmap is queried for any callbacks for the said event and if any are found, then they are executed sequentially.
    listeners: HashMap<Event, Vec<Listener>>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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

    UnresolvedArgument,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    /// Receives an event and the actual listener to be set then matches on the listener and adds to the listener to its desired vector.
    pub fn on(&mut self, event: Event, callback: Listener) {
        self._add_listener(event, callback);
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

    /// This method retrives the vector of existing callbacks if any and pushes the new listener to the vector
    fn _add_listener(&mut self, event: Event, callback: fn(&Program, String) -> ()) {
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
