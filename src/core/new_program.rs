#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{
    parser::{matches::ParserMatches, resolve_flag, Argument, Flag},
    EventEmitter, Pattern, Theme,
};

use super::ProgramSettings;

pub struct Program {}

impl Program {
    pub fn new() -> Command<'static> {
        Command {
            name: "",
            alias: None,
            arguments: vec![],
            flags: vec![],
            description: "",
            subcommands: Box::new(vec![]),
            callback: None,
            metadata: Some(CmdMetadata::default()),
            parent: None,
        }
    }
}

#[derive(Clone)]
pub struct Command<'p> {
    name: &'p str,
    alias: Option<&'p str>,
    arguments: Vec<Argument>,
    flags: Vec<Flag>,
    description: &'p str,
    parent: Option<&'p Command<'p>>,
    subcommands: Box<Vec<&'p Command<'p>>>,
    callback: Option<fn() -> ()>,
    metadata: Option<CmdMetadata<'p>>,
}

#[derive(Clone)]
pub struct CmdMetadata<'a> {
    version: &'a str,
    author: &'a str,
    theme: Theme,
    pattern: Pattern,
    emitter: EventEmitter,
    settings: ProgramSettings,
}

impl<'c> CmdMetadata<'c> {
    fn new() -> Self {
        Self {
            version: "0.1.0",
            author: "Rustacean",
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            emitter: EventEmitter::default(),
            settings: ProgramSettings::default(),
        }
    }
}

impl<'d> Default for CmdMetadata<'d> {
    fn default() -> Self {
        Self::new()
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
            subcommands: Box::new(vec![]),
            callback: None,
            metadata: None,
            parent: None,
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
            ""
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

    pub fn get_flags(&self) -> &Vec<Flag> {
        &self.flags
    }

    pub fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn get_subcommands(&self) -> &Vec<&Self> {
        &self.subcommands
    }

    pub fn get_parent(&self) -> Option<&Self> {
        self.parent
    }

    pub fn find_subcommand(&self, val: &str) -> Option<&&Command<'_>> {
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

    fn _add_sub_cmd(&mut self, sub_cmd: &'p Self) {
        self.subcommands.push(sub_cmd);
    }

    fn _add_parent(&mut self, parent_cmd: &'p Self) -> &'p Self {
        self.parent = Some(parent_cmd);
        parent_cmd
    }

    pub fn build(&'p mut self, parent_cmd: &'p mut Self) -> &Self {
        // TODO: Find a way to achieve this without using the build method
        // self._add_parent(parent_cmd)._add_sub_cmd(self);
        parent_cmd._add_sub_cmd(self);
        self
    }

    pub fn alias(&mut self, val: &'p str) -> &mut Self {
        self.alias = Some(val);
        self
    }

    pub fn description(&mut self, val: &'p str) -> &mut Self {
        self.description = val;
        self
    }

    pub fn subcommand(&mut self, name: &'p str) -> Self {
        Self::new(name)
    }

    pub fn argument(&mut self, val: &str, desc: &str) -> &mut Self {
        let arg = Argument::new(val, Some(desc.to_string()));

        if !self.arguments.contains(&arg) {
            self.arguments.push(arg);
        }

        self
    }

    pub fn option(&mut self, val: &str, desc: &str) -> &mut Self {
        let flag = Flag::new(val, desc);

        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }

        self
    }

    // Settings

    // Parser
    fn _is_subcommand(&self) -> bool {
        self.parent.is_some()
            && self
                .parent
                .unwrap()
                .find_subcommand(self.get_name())
                .is_some()
    }

    fn _parse(
        &'p self,
        raw_args: &[&str],
        root_cfg: Option<ParserMatches<'p>>,
    ) -> ParserMatches<'p> {
        if raw_args.is_empty() {
            // handle empty args
        }

        let mut config = if root_cfg.is_some() {
            root_cfg.unwrap()
        } else {
            ParserMatches::new(raw_args.len())
        };

        // ["image", "ls", "-p", "80"]
        for (idx, arg) in raw_args.iter().enumerate() {
            if let Some(flag) = resolve_flag(self.get_flags(), arg) {
                // handle flags input
            } else if let Some(sub_cmd) = self.find_subcommand(arg) {
                // it is a subcommand/command
                return sub_cmd._parse(&raw_args[1..], Some(config));
            } else {
                // it is either an argument or unknown
            }
        }

        config
    }

    pub fn parse(&self) {}

    pub fn parse_from(&self, list: Vec<&str>) {}

    pub fn get_matches(&self) {}

    pub fn get_matches_from(&self, list: Vec<&str>) {}

    // Others
    pub fn output_help(&self) {}
}