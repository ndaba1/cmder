#![allow(dead_code)]
#![allow(unused)]

use std::{collections::HashMap, fmt::Debug};

use crate::{
    parse::{
        matches::{FlagsMatches, ParserMatches},
        parser::NewParser,
        resolve_flag, Argument, Flag,
    },
    Event, Pattern, Theme,
};

use super::{
    super::parse::flags::{NewFlag, NewOption},
    events::{EventConfig, NewEventEmitter},
};
use super::{events::NewListener, ProgramSettings};

type Callback = fn(ParserMatches) -> ();

pub struct Program {}

impl Program {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Command<'static> {
        Command {
            name: "",
            alias: None,
            arguments: vec![],
            flags: vec![
                NewFlag::new("-h", "--help", "Print out help information"),
                NewFlag::new("-v", "--version", "Print out version information"),
            ],
            options: vec![],
            description: "",
            subcommands: vec![],
            callback: None,
            metadata: Some(CmdMetadata::default()),
            parent: None,
            cmd_path: "",
            more_info: "",
        }
    }
}

#[derive(Clone)]
pub struct Command<'p> {
    name: &'p str,
    alias: Option<&'p str>,
    arguments: Vec<Argument>,
    flags: Vec<NewFlag<'p>>,
    options: Vec<NewOption<'p>>,
    description: &'p str,
    parent: Option<&'p Command<'p>>,
    subcommands: Vec<Command<'p>>,
    callback: Option<Callback>,
    metadata: Option<CmdMetadata<'p>>,
    cmd_path: &'p str,
    more_info: &'p str,
}

#[derive(Clone, Debug)]
pub struct CmdMetadata<'a> {
    version: &'a str,
    author: &'a str,
    theme: Theme,
    pattern: Pattern,
    emitter: NewEventEmitter,
    settings: ProgramSettings,
}

impl<'c> CmdMetadata<'c> {
    fn new() -> Self {
        Self {
            version: "0.1.0",
            author: "Rustacean",
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            emitter: NewEventEmitter::default(),
            settings: ProgramSettings::default(),
        }
    }
}

impl<'d> Default for CmdMetadata<'d> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'d> Debug for Command<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "
                {},
                {},
                {:#?},
                {:#?},
                {:#?},
                {:#?},
            ",
            self.name, self.description, self.arguments, self.flags, self.options, self.subcommands
        ))
    }
}

impl<'p> Command<'p> {
    pub(crate) fn new(name: &'p str) -> Self {
        Self {
            name,
            alias: None,
            arguments: vec![],
            description: "",
            flags: vec![],
            options: vec![],
            subcommands: vec![],
            callback: None,
            metadata: None,
            parent: None,
            cmd_path: "",
            more_info: "",
        }
    }

    // Root command options
    pub fn author(&mut self, author: &'p str) -> &mut Self {
        if let Some(meta) = &mut self.metadata {
            meta.author = author
        }

        self
    }

    pub fn version(&mut self, val: &'p str) -> &mut Self {
        if let Some(meta) = &mut self.metadata {
            meta.version = val
        }

        self
    }

    pub fn bin_name(&mut self, val: &'p str) -> &mut Self {
        if let Some(_meta) = &mut self.metadata {
            self.name = val
        }

        self
    }

    // Getters
    pub fn get_author(&self) -> &str {
        if let Some(meta) = &self.metadata {
            meta.author
        } else {
            ""
        }
    }

    pub fn get_version(&self) -> &str {
        if let Some(meta) = &self.metadata {
            meta.version
        } else {
            "0.1.0"
        }
    }

    pub fn get_theme(&self) -> Theme {
        if let Some(meta) = &self.metadata {
            // FIX: No clones
            meta.theme.clone()
        } else {
            Theme::default()
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_alias(&self) -> Option<&str> {
        self.alias
    }

    pub fn get_flags(&self) -> &Vec<NewFlag> {
        &self.flags
    }

    pub fn get_options(&self) -> &Vec<NewOption> {
        &self.options
    }

    pub fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn get_subcommands(&self) -> &Vec<Self> {
        &self.subcommands
    }

    pub fn s(&mut self) -> &mut Vec<Self> {
        &mut self.subcommands
    }

    pub fn get_parent(&self) -> Option<&Self> {
        self.parent
    }

    pub fn find_subcommand(&self, val: &str) -> Option<&Command<'_>> {
        self.subcommands
            .iter()
            .find(|c| c.get_name() == val || c.get_alias() == Some(val))
    }

    fn _get_target_name(&self, val: String) -> String {
        if self.name.is_empty() {
            if cfg!(windows) {
                let path_buff: Vec<&str> = val.split('\\').collect();
                let target = path_buff.last().unwrap();
                target.replace(".exe", "")
            } else {
                let path_buff: Vec<&str> = val.split('/').collect();
                let target = path_buff.last().unwrap();
                target.to_string()
            }
        } else {
            self.name.to_string()
        }
    }

    // Core functionality
    fn _add_args(&mut self, args: &[&str]) {
        for p in args.iter() {
            let temp = Argument::new(p, None);
            if !self.arguments.contains(&temp) {
                self.arguments.push(temp);
            }
        }
    }

    fn _add_sub_cmd(&mut self, sub_cmd: Self) {
        self.subcommands.push(sub_cmd);
    }

    fn _add_parent(mut self, parent_cmd: &'p Self) -> Self {
        self.parent = Some(parent_cmd);
        self
    }

    pub fn build(&self, cmd_vec: &mut Vec<Self>) {
        // TODO: Find a way to achieve this without using the build method
        // FIXME: No clones
        cmd_vec.push(self.clone());
    }

    pub fn alias(&mut self, val: &'p str) -> &mut Self {
        self.alias = Some(val);
        self
    }

    pub fn description(&mut self, val: &'p str) -> &mut Self {
        self.description = val;
        self
    }

    pub fn subcommand(&self, name: &'p str) -> Self {
        Self::new(name)
    }

    pub fn argument(&mut self, val: &str, desc: &str) -> &mut Self {
        let arg = Argument::new(val, Some(desc.to_string()));

        if !self.arguments.contains(&arg) {
            self.arguments.push(arg);
        }

        self
    }

    pub fn action(&mut self, cb: Callback) -> &mut Self {
        self.callback = Some(cb);
        self
    }

    pub fn option(&mut self, val: &'p str, desc: &'p str) -> &mut Self {
        let values: Vec<_> = val.split_whitespace().collect();

        match values.len() {
            2 => {
                let flag = NewFlag::new(values[0], values[1], desc);
                if !self.flags.contains(&flag) {
                    self.flags.push(flag)
                }
            }
            val if val > 2 => {
                let option = NewOption::new(values[0], values[1], desc, &values[2..]);
                if !self.options.contains(&option) {
                    self.options.push(option)
                }
            }
            _ => {}
        }

        self
    }

    // Settings
    pub fn on(&mut self, event: Event, cb: NewListener) {
        if let Some(meta) = &mut self.metadata {
            meta.emitter.on(event, cb);
        }
    }

    pub fn emit(&mut self, cfg: EventConfig) {
        if let Some(meta) = &mut self.metadata {
            meta.emitter.emit(cfg);
        }
    }

    // Parser
    fn _is_subcommand(&self) -> bool {
        self.parent.is_some()
            && self
                .parent
                .unwrap()
                .find_subcommand(self.get_name())
                .is_some()
    }

    pub fn parse(&'p mut self) {
        let raw_args: Vec<_> = std::env::args().collect();
        let mut cleaned_args = vec![];

        for a in &raw_args {
            cleaned_args.push(a.as_str());
        }

        // self.name = self._get_target_name(raw_args[0]).as_str();

        match NewParser::parse(self, &cleaned_args[1..], None) {
            Ok(res) => {
                dbg!(res);
            }
            _ => {}
        }
    }

    pub fn parse_from(&mut self, list: Vec<&str>) {}

    pub fn get_matches(&mut self) {}

    pub fn get_matches_from(&mut self, list: Vec<&str>) {}

    // Others
    pub fn output_help(&self) {}

    pub fn before_all(&self) {}

    pub fn before_help(&self) {}

    pub fn after_all(&self) {}

    pub fn after_help(&self) {}
}
