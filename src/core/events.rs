#![allow(unused)]
use std::{collections::HashMap, fmt::Debug};

use super::program::Command;

/// The event config struct defines the structure of the data passed to a listener to a particular event. Whenever an event occurs, its config is generated depending on the context. All its members are private but has numerous getters to access the fields data.
#[derive(Clone, Debug)]
pub struct EventConfig<'e> {
    pub(crate) args: Vec<String>,
    pub(crate) arg_count: usize,
    pub(crate) error_string: String,
    pub(crate) exit_code: usize,
    pub(crate) event_type: Event,
    pub(crate) matched_cmd: Option<&'e Command<'e>>,
    pub(crate) additional_info: &'e str,
    pub(crate) program_ref: &'e Command<'e>,
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
        self.program_ref
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

    pub fn program(mut self, p_ref: &'a Command<'a>) -> Self {
        self.program_ref = p_ref;
        self
    }
}

impl<'a> EventConfig<'a> {
    pub fn new(cmd: &'a Command<'a>) -> Self {
        Self {
            additional_info: "",
            error_string: "".into(),
            arg_count: 0,
            args: vec![],
            event_type: Event::OutputHelp,
            exit_code: 0,
            matched_cmd: None,
            program_ref: cmd,
        }
    }
}

pub type EventCallback = fn(EventConfig) -> ();

#[derive(Clone)]
pub struct EventListener {
    pub cb: EventCallback,
    pub index: isize,
}

/// The event emitter struct simply contains a `listeners` field which is a vector containing a tuple with the structure: (`EventListener`, `index_of_execution`).
#[derive(Clone)]
pub struct EventEmitter {
    listeners: HashMap<Event, Vec<EventListener>>,
    events_to_override: Vec<Event>,
}

impl Debug for EventEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#?}", self.listeners.keys()))
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Event {
    MissingRequiredArgument,
    OptionMissingArgument,
    OutputCommandHelp,
    OutputHelp,
    OutputVersion,
    UnknownCommand,
    UnknownOption,
    UnresolvedArgument,
}

fn get_events_slice() -> Vec<Event> {
    use Event::*;
    vec![
        MissingRequiredArgument,
        OutputHelp,
        OutputVersion,
        UnknownCommand,
        UnknownOption,
        UnresolvedArgument,
    ]
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            events_to_override: vec![],
        }
    }

    pub fn on(&mut self, event: Event, cb: EventCallback, pstn: i32) {
        let new = EventListener {
            cb,
            index: pstn as isize,
        };
        match self.listeners.get_mut(&event) {
            Some(lstnrs) => lstnrs.push(new),
            None => {
                self.listeners.insert(event, vec![new]);
            }
        };
    }

    pub fn override_event(&mut self, event: Event) {
        self.events_to_override.push(event)
    }

    pub fn emit(&self, cfg: EventConfig) {
        let event = cfg.get_event();

        if let Some(lstnrs) = self.listeners.get(&event) {
            let mut lstnrs = lstnrs.clone();

            lstnrs.sort_by(|a, b| a.index.cmp(&b.index));

            for (lstnr) in lstnrs {
                (lstnr.cb)(cfg.clone());
            }

            std::process::exit(cfg.get_exit_code() as i32);
        }
    }

    pub(crate) fn insert_before_all(&mut self, cb: EventCallback) {
        for event in get_events_slice() {
            self.on(event, cb, -5)
        }
    }

    pub(crate) fn insert_after_all(&mut self, cb: EventCallback) {
        for event in get_events_slice() {
            self.on(event, cb, 5)
        }
    }

    pub(crate) fn on_all(&mut self, cb: EventCallback, pstn: i32) {
        for event in get_events_slice() {
            self.on(event, cb, pstn)
        }
    }

    pub(crate) fn on_errors(&mut self, cb: EventCallback, pstn: i32) {
        use Event::*;
        for event in get_events_slice() {
            // Ignore events that aren't errors
            if event == OutputHelp || event == OutputVersion {
                continue;
            } else {
                self.on(event, cb, pstn)
            }
        }
    }

    pub(crate) fn rm_default_lstnr(&mut self, event: Event, val: i32) {
        if let Some(lstnrs) = self.listeners.get_mut(&event) {
            for (idx, lstnr) in lstnrs.clone().iter().enumerate() {
                if lstnr.index == val as isize {
                    lstnrs.remove(idx);
                }
            }
        }
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}
