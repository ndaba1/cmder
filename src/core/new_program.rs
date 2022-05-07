#![allow(dead_code)]
#![allow(unused)]

use std::{collections::HashMap, env, fmt::Debug};

use crate::{
    core::errors::CmderError,
    parse::{
        matches::{FlagsMatches, ParserMatches},
        parser::NewParser,
        resolve_flag, Argument, Flag,
    },
    utils::{self, suggest_cmd},
    Event, Pattern, PredefinedThemes, Theme,
};

use super::{
    super::parse::flags::{NewFlag, NewOption},
    events::{EventConfig, NewEventEmitter},
    settings::{NewProgramSettings, Setting},
};
use super::{events::NewListener, ProgramSettings};

type Callback = fn(ParserMatches) -> ();

pub struct Program {}

impl Program {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Command<'static> {
        Command {
            flags: vec![
                NewFlag::new("-h", "--help", "Print out help information"),
                NewFlag::new("-v", "--version", "Print out version information"),
            ],
            metadata: Some(CmdMetadata::default()),
            ..Command::new("")
        }
    }
}

#[derive(Clone)]
pub struct Command<'p> {
    name: String,
    alias: Option<&'p str>,
    arguments: Vec<Argument>,
    flags: Vec<NewFlag<'p>>,
    options: Vec<NewOption<'p>>,
    description: &'p str,
    parent: Option<Box<Command<'p>>>,
    subcommands: Vec<Command<'p>>,
    callback: Callback,
    metadata: Option<CmdMetadata<'p>>,
    cmd_path: Vec<String>,
    more_info: &'p str,
}

#[derive(Clone, Debug)]
pub struct CmdMetadata<'a> {
    version: &'a str,
    author: &'a str,
    theme: Theme,
    pattern: Pattern,
    emitter: NewEventEmitter,
    settings: NewProgramSettings,
}

impl<'c> CmdMetadata<'c> {
    fn new() -> Self {
        Self {
            version: "0.1.0",
            author: "Rustacean",
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            emitter: NewEventEmitter::default(),
            settings: NewProgramSettings::default(),
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
            "name: {},
             alias: {},
             args: {:#?},
             flags: {:#?},
             options: {:#?},
             cmd_path: {:#?},
             sub_cmds: {:#?}",
            self.name,
            self.alias.unwrap_or(""),
            self.arguments,
            self.flags,
            self.options,
            self.cmd_path,
            self.subcommands
        ))
    }
}

impl<'p> Command<'p> {
    pub(crate) fn new(name: &'p str) -> Self {
        Self {
            name: name.to_string(),
            alias: None,
            arguments: vec![],
            description: "",
            flags: vec![],
            options: vec![],
            subcommands: vec![],
            callback: |_m| {},
            metadata: None,
            parent: None,
            cmd_path: vec![name.to_string()],
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
            self.name = val.to_string()
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
        self.name.as_str()
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

    pub fn get_parent(&self) -> Option<&Box<Self>> {
        self.parent.as_ref()
    }

    pub fn find_subcommand(&self, val: &str) -> Option<&Command<'_>> {
        self.subcommands
            .iter()
            .find(|c| c.get_name() == val || c.get_alias() == Some(val))
    }

    fn _get_target_name(&self, val: &str) -> String {
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

    // fn _add_parent(mut self, parent_cmd: &'p Self) -> Self {
    //     self.parent = Some(parent_cmd);
    //     self
    // }

    pub fn build(&mut self, cmd_vec: &mut Vec<Self>) {
        // TODO: Find a way to achieve this without using the build method
        self.__init();
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
        self.callback = cb;
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

    pub fn set(&mut self, setting: Setting) {
        if let Some(meta) = &mut self.metadata {
            use Setting::*;
            match setting {
                EnableCommandSuggestion(enable) => {
                    meta.settings.enable_command_suggestions = enable
                }
                HideCommandAliases(hide) => meta.settings.hide_command_aliases = hide,
                SeparateOptionsAndFlags(separate) => {
                    meta.settings.separate_options_and_flags = separate
                }
                ShowHelpOnError(show) => meta.settings.show_help_on_error = show,
                ChoosePredefinedTheme(theme) => match theme {
                    PredefinedThemes::Plain => meta.theme = Theme::plain(),
                    PredefinedThemes::Colorful => meta.theme = Theme::colorful(),
                },
                DefineCustomTheme(theme) => meta.theme = theme,
                SetProgramPattern(pattern) => meta.pattern = pattern,
            }
        }
    }

    // Parser
    fn _handle_flags(&mut self, matches: &ParserMatches) {
        let program = matches.get_program();

        if let Some(_f) = matches.get_flag("-h") {
            let cfg = EventConfig::default().program(program.clone());

            self.output_help();
            self.emit(cfg);
            std::process::exit(0);
        } else if let Some(_f) = matches.get_flag("-v") {
            let version = program.get_version();

            let cfg = EventConfig::default()
                .arg_c(1_usize)
                .args(vec![version.to_string()])
                .set_event(Event::OutputVersion)
                .program(program.clone());

            self.emit(cfg);
            self.output_version();
            std::process::exit(0);
        }
    }

    fn __parse(&'p mut self, args: &[&str]) {
        self.__init();

        match NewParser::parse(self, args, None) {
            Ok(matches) => {
                if let Some(sub_cmd) = matches.get_matched_cmd() {
                    (sub_cmd.callback)(matches);
                } else {
                    (self.callback)(matches);
                }
            }
            Err(e) => {
                eprintln!("{e}");
                let shared_cfg = EventConfig::default().program(self.clone());

                use CmderError::*;
                let event_cfg = match e {
                    MissingArgument(args) => shared_cfg
                        .arg_c(args.len())
                        .args(args)
                        .exit_code(5)
                        .set_event(Event::MissingArgument),
                    OptionMissingArgument(args) => shared_cfg
                        .arg_c(args.len())
                        .args(args)
                        .exit_code(10)
                        .set_event(Event::OptionMissingArgument),
                    UnknownCommand(cmd) => shared_cfg
                        .arg_c(1)
                        .args(vec![cmd.to_string()])
                        .exit_code(15)
                        .set_event(Event::UnknownCommand),
                    UnknownOption(opt) => shared_cfg
                        .arg_c(1)
                        .args(vec![opt.to_string()])
                        .exit_code(20)
                        .set_event(Event::UnknownOption),
                };

                self.emit(event_cfg.clone());
                std::process::exit(event_cfg.get_exit_code() as i32);
            }
        }
    }

    fn __init(&mut self) {
        let parent = self.clone();

        for cmd in &mut self.subcommands {
            // Set the cmd_path
            let mut temp = self.cmd_path.clone();
            temp.extend_from_slice(&cmd.cmd_path[..]);
            cmd.cmd_path = temp;

            // Set the parent
            cmd.parent = Some(Box::new(parent.clone()));
        }

        // Means that it is the root_cmd(program)
        if let Some(meta) = self.metadata.clone() {
            let settings = &meta.settings;

            // Register help listeners
            if settings.show_help_on_error {
                let _output_help_ = |cfg: EventConfig| {
                    let prog = cfg.get_program();

                    if let Some(cmd) = cfg.get_matched_cmd() {
                        cmd.output_help()
                    } else {
                        prog.output_help()
                    }
                };

                use Event::*;
                // Output help on all error events
                self.on(MissingArgument, _output_help_);
                self.on(OptionMissingArgument, _output_help_);
                self.on(UnknownCommand, _output_help_);
                self.on(UnknownOption, _output_help_);
            }

            // Register listener for unknown commands
            if settings.enable_command_suggestions {
                self.on(Event::UnknownCommand, |cfg| {
                    // Suggest command
                    let prog = cfg.get_program();
                    let cmd = &cfg.get_args()[0];

                    if let Some(_ans) = suggest_cmd(&cmd, prog.get_subcommands()) {
                        // output command suggestion
                    }
                })
            }
        }
    }

    pub fn parse(&'p mut self) {
        let raw_args = env::args().collect::<Vec<_>>();
        let args = raw_args.iter().map(|a| a.as_str()).collect::<Vec<_>>();

        self.name = self._get_target_name(args[0]);
        self.cmd_path = vec![self.name.clone()];

        self.__parse(&args[1..]);
    }

    pub fn parse_from(&'p mut self, list: Vec<&str>) {
        self.__parse(&list[..]);
    }

    pub fn get_matches(&'p mut self) {}

    pub fn get_matches_from(
        &'p mut self,
        list: &'p [&'p str],
    ) -> Result<ParserMatches<'p>, CmderError<'p>> {
        NewParser::parse(self, list, None)
    }

    // Others
    fn suggest_cmd(&self, cmd: &str) -> Option<String> {
        utils::suggest_cmd(cmd, self.get_subcommands())
    }
    pub fn output_help(&self) {}

    pub fn output_version(&self) {}

    pub fn before_all(&self) {}

    pub fn before_help(&self) {}

    pub fn after_all(&self) {}

    pub fn after_help(&self) {}
}
