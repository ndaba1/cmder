#![allow(dead_code)]
#![allow(unused)]

use std::{cell::RefCell, collections::HashMap, env, fmt::Debug, rc::Rc};

use crate::{
    core::errors::CmderError,
    parse::{
        matches::{FlagsMatches, ParserMatches},
        parser::Parser,
        resolve_flag, Argument, Flag,
    },
    ui::formatter::{CustomPattern, FormatGenerator},
    utils::{self, suggest_cmd, HelpWriter},
    Event, Pattern, PredefinedThemes, Theme,
};

use super::events::EventListener;
use super::{
    super::parse::flags::{NewFlag, NewOption},
    errors::CmderResult,
    events::{EventConfig, EventEmitter},
    settings::{ProgramSettings, Setting},
};

type Callback = fn(ParserMatches) -> ();

pub struct Program {}

impl Program {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Command<'static> {
        Command {
            flags: vec![
                NewFlag::new("-v", "--version", "Print out version information"),
                NewFlag::new("-h", "--help", "Print out help information"),
            ],
            is_root: true,
            ..Command::new("")
        }
    }
}

#[derive(Clone)]
pub struct Command<'p> {
    name: String,
    theme: Theme,
    is_root: bool,
    pattern: Pattern,
    alias: Option<&'p str>,
    author: Option<&'p str>,
    version: Option<&'p str>,
    arguments: Vec<Argument>,
    flags: Vec<NewFlag<'p>>,
    options: Vec<NewOption<'p>>,
    description: Option<&'p str>,
    more_info: Option<&'p str>,
    usage_str: Option<&'p str>,
    settings: ProgramSettings,
    emitter: Option<EventEmitter>,
    subcommands: Vec<Command<'p>>,
    callbacks: Vec<(Callback, i32)>, // (cb_function, index_of_execution)
    parent: Option<Rc<Command<'p>>>,
}

impl<'d> Debug for Command<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "name: {},",
            self.name,
            // self.alias.unwrap_or(""),
            // self.arguments,
            // self.flags,
            // self.options,
            // self.cmd_path,
            // self.subcommands
        ))
    }
}

impl<'p> Command<'p> {
    pub fn new(name: &'p str) -> Self {
        Self {
            name: name.to_string(),
            alias: None,
            arguments: vec![],
            description: None,
            flags: vec![NewFlag::new("-h", "--help", "Print out help information")],
            options: vec![],
            subcommands: vec![],
            callbacks: vec![],
            parent: None,
            more_info: None,
            version: None,
            author: None,
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            emitter: Some(EventEmitter::default()),
            settings: ProgramSettings::default(),
            is_root: false,
            usage_str: None,
        }
    }

    // Root command options
    pub fn author(&mut self, author: &'p str) -> &mut Self {
        self.author = Some(author);
        self
    }

    pub fn version(&mut self, val: &'p str) -> &mut Self {
        self.version = Some(val);
        self
    }

    pub fn bin_name(&mut self, val: &'p str) -> &mut Self {
        if self.is_root {
            self.name = val.into();
        }
        self
    }

    // Getters
    pub fn get_author(&self) -> &str {
        self.author.unwrap_or("")
    }

    pub fn get_version(&self) -> &str {
        self.version.unwrap_or("")
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    pub fn get_pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_alias(&self) -> &str {
        self.alias.unwrap_or("")
    }

    pub fn get_flags(&self) -> &Vec<NewFlag> {
        &self.flags
    }

    pub fn get_description(&self) -> &str {
        self.description.unwrap_or("")
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

    pub fn get_parent(&self) -> Option<&Rc<Self>> {
        self.parent.as_ref()
    }

    pub(crate) fn get_usage_str(&self) -> String {
        let mut parent = self.get_parent();
        let mut usage = vec![self.get_name()];
        let mut usage_str = String::new();

        while parent.is_some() {
            usage.push(parent.unwrap().get_name());
            parent = parent.unwrap().get_parent();
        }

        usage.reverse();

        for v in &usage {
            usage_str.push_str(v);
            usage_str.push(' ');
        }

        usage_str.trim().into()
    }

    pub fn find_subcommand(&self, val: &str) -> Option<&Command<'_>> {
        self.subcommands
            .iter()
            .find(|c| c.get_name() == val || c.get_alias() == val)
    }

    fn _get_callbacks(&self) -> &Vec<(Callback, i32)> {
        &self.callbacks
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

    fn _add_parent(&mut self, parent: Rc<Self>) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    #[deprecated(note = "Subcmds now built automatically")]
    pub fn build(&mut self) {}

    pub fn alias(&mut self, val: &'p str) -> &mut Self {
        self.alias = Some(val);
        self
    }

    pub fn description(&mut self, val: &'p str) -> &mut Self {
        self.description = Some(val);
        self
    }

    pub fn subcommand(&mut self, name: &'p str) -> &mut Self {
        let parent = Rc::new(self.to_owned());

        self.subcommands.push(Self::new(name));
        self.subcommands.last_mut().unwrap()._add_parent(parent)
    }

    pub fn argument(&mut self, val: &str, help: &str) -> &mut Self {
        let arg = Argument::new(val, Some(help.to_string()));

        if !self.arguments.contains(&arg) {
            self.arguments.push(arg);
        }

        self
    }

    pub fn action(&mut self, cb: Callback) -> &mut Self {
        self.callbacks.push((cb, 0));
        self
    }

    pub fn option(&mut self, val: &'p str, help: &'p str) -> &mut Self {
        let values: Vec<_> = val.split_whitespace().collect();

        match values.len() {
            2 => {
                let flag = NewFlag::new(values[0], values[1], help);
                if !self.flags.contains(&flag) {
                    self.flags.push(flag)
                }
            }
            val if val > 2 => {
                let option = NewOption::new(values[0], values[1], help, &values[2..]);
                if !self.options.contains(&option) {
                    self.options.push(option)
                }
            }
            _ => {}
        }

        self
    }

    // Settings
    pub fn on(&mut self, event: Event, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(event, cb, 0)
        }
    }

    pub fn emit(&mut self, cfg: EventConfig) {
        if let Some(emitter) = &mut self.emitter {
            emitter.emit(cfg);
        }
    }

    pub fn set(&mut self, setting: Setting) {
        let s = &mut self.settings;

        use Setting::*;
        match setting {
            ChoosePredefinedTheme(theme) => match theme {
                PredefinedThemes::Plain => self.theme = Theme::plain(),
                PredefinedThemes::Colorful => self.theme = Theme::colorful(),
            },
            EnableCommandSuggestion(enable) => s.enable_command_suggestions = enable,
            HideCommandAliases(hide) => s.hide_command_aliases = hide,
            SeparateOptionsAndFlags(separate) => s.separate_options_and_flags = separate,
            ShowHelpOnAllErrors(show) => s.show_help_on_all_errors = show,
            ShowHelpOnEmptyArgs(show) => s.show_help_on_empty_args = show,
            DefineCustomTheme(theme) => self.theme = theme,
            SetProgramPattern(pattern) => self.pattern = pattern,
            OverrideAllDefaultListeners(val) => s.override_all_default_listeners = val,
            OverrideSpecificEventListener(event) => s.events_to_override.push(event),
            AutoIncludeHelpSubcommand(val) => s.auto_include_help_subcommand = val,
            EnableTreeViewSubcommand(val) => s.enable_tree_view_subcommand = val,
            IgnoreAllErrors(val) => s.ignore_all_errors = val,
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
            std::process::exit(0);
        }
    }

    fn __parse(&'p mut self, args: Vec<String>) {
        if args.is_empty() {
            // handle empty args
            return;
        }

        // TODO: Change get target name to account for non path-buffer values
        self.name = self._get_target_name(&args[0]);

        self.__init(); // performance dip here

        let mut parser = Parser::new(self);

        match parser.parse(args[1..].to_vec()) {
            Ok(matches) => {
                let exec_callbacks = |cmd: &Command| {
                    // FIXME: No clones
                    let mut cbs = cmd._get_callbacks().clone();

                    // Sort by index
                    cbs.sort_by(|a, b| a.1.cmp(&b.1));

                    // Execute callbacks
                    for cb in cbs {
                        (cb.0)(matches.clone());
                    }
                };

                if let Some(sub_cmd) = matches.get_matched_cmd() {
                    exec_callbacks(sub_cmd);
                } else {
                    exec_callbacks(self);
                }
            }
            Err(e) => {
                let shared_cfg = EventConfig::default()
                    .program(self.clone())
                    .error_str(e.clone().into());

                use CmderError::*;
                let event_cfg = match e {
                    MissingRequiredArgument(args) => shared_cfg
                        .arg_c(args.len())
                        .args(args)
                        .exit_code(5)
                        .set_event(Event::MissingRequiredArgument),
                    OptionMissingArgument(args) => shared_cfg
                        .arg_c(args.len())
                        .args(args)
                        .exit_code(10)
                        .set_event(Event::OptionMissingArgument),
                    UnknownCommand(cmd) => shared_cfg
                        .arg_c(1)
                        .args(vec![cmd])
                        .exit_code(15)
                        .set_event(Event::UnknownCommand),
                    UnknownOption(opt) => shared_cfg
                        .arg_c(1)
                        .args(vec![opt])
                        .exit_code(20)
                        .set_event(Event::UnknownOption),
                    UnresolvedArgument(vals) => shared_cfg
                        .arg_c(vals.len())
                        .args(vals)
                        .exit_code(25)
                        .set_event(Event::UnresolvedArgument),
                };

                self.emit(event_cfg);
            }
        }
    }

    fn __init(&mut self) {
        if !self.subcommands.is_empty() {
            // Add help subcommand
            // TODO: Check settings for help command
            self.subcommand("help")
                .argument("<SUB-COMMAND>", "The subcommand to print out help info for")
                .description("A subcommand used for printing out help")
                .action(|m| {
                    let cmd = m.get_matched_cmd().unwrap();
                    let val = m.get_arg("<SUB-COMMAND>").unwrap();
                    let parent = cmd.get_parent().unwrap();

                    if let Some(cmd) = parent.find_subcommand(&val) {
                        cmd.output_help();
                    }
                });
            self.subcommand("tree")
                .description("A subcommand used for printing out a tree view of the command tree")
                .action(|m| {
                    let cmd = m.get_matched_cmd().unwrap();

                    cmd.display_commands_tree();
                });
        }

        // Means that it is the root_cmd(program)
        if let Some(emitter) = &mut self.emitter {
            let settings = &self.settings;

            use Event::*;
            // Register default listeners
            if !settings.override_all_default_listeners {
                // Default behavior for errors is to print the error message
                if !settings.ignore_all_errors {
                    emitter.on_all_errors(
                        |cfg| {
                            let error = cfg.get_error_str();

                            if !error.is_empty() {
                                eprintln!("Error: {error}");
                            }
                        },
                        -4,
                    );
                }

                // Register default output version listener
                emitter.on(
                    OutputVersion,
                    |cfg| {
                        let p = cfg.get_program();

                        println!("{}, v{}", p.get_name(), p.get_version());
                        println!("{}", p.get_author());
                        println!("{}", p.get_description());
                    },
                    -4,
                );

                // Remove default listeners if behavior set to override
                for event in &settings.events_to_override {
                    emitter.rm_lstnr_idx(*event, -4)
                }
            }

            // Register help listeners
            if settings.show_help_on_all_errors && !settings.ignore_all_errors {
                let _output_help_ = |cfg: EventConfig| {
                    let prog = cfg.get_program();

                    if let Some(cmd) = cfg.get_matched_cmd() {
                        cmd.output_help()
                    } else {
                        prog.output_help()
                    }
                };

                // Output help on all error events
                emitter.insert_before_all(_output_help_);
            }

            // Register listener for unknown commands
            if settings.enable_command_suggestions {
                // Remove default listener to register new default one
                emitter.rm_lstnr_idx(UnknownCommand, -4);

                emitter.on(
                    UnknownCommand,
                    |cfg| {
                        println!("Error: {}\n", cfg.get_error_str());

                        // Suggest command
                        let prog = cfg.get_program();
                        let cmd = &cfg.get_args()[0];

                        if let Some(ans) = utils::suggest_cmd(cmd, prog.get_subcommands()) {
                            // output command suggestion
                            println!("       Did you mean: `{ans}` ?\n")
                        }
                    },
                    -1,
                )
            }
        }
    }

    pub fn parse(&'p mut self) {
        let args = env::args().collect::<Vec<_>>();
        self.__parse(args);
    }

    pub fn parse_from(&'p mut self, list: Vec<&str>) {
        let args = list.iter().map(|a| a.to_string()).collect::<Vec<_>>();
        self.__parse(args);
    }

    // Others
    pub fn output_help(&self) {
        HelpWriter::write(self, self.get_theme().clone(), Pattern::Legacy);
    }

    pub fn before_all(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.insert_before_all(cb)
        }
    }

    pub fn after_all(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.insert_after_all(cb)
        }
    }

    pub fn before_help(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(Event::OutputHelp, cb, -1)
        }
    }

    pub fn after_help(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(Event::OutputHelp, cb, 1)
        }
    }

    // Debug utilities
    pub fn display_commands_tree(&self) {
        let mut commands = self.get_subcommands();
        let mut empty = String::new();

        let mut parent = self.get_parent();

        while parent.is_some() {
            empty.push('\t');
            empty.push('|');

            parent = parent.unwrap().get_parent();
        }

        println!("{}-> {}", &empty, self.get_name());

        for cmd in commands.iter() {
            cmd.display_commands_tree();
        }
    }

    pub fn init_dbg(&mut self) {
        self.__init();
    }
}

impl<'f> FormatGenerator for Command<'f> {
    fn generate(&self, ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        use crate::ui::formatter::Pattern;
        match &ptrn {
            Pattern::Custom(ptrn) => {
                let base = &ptrn.sub_cmds_fmter;

                let mut leading = base.replace("{{name}}", self.get_name());
                let mut floating = String::from("");

                if let Some(alias) = self.alias {
                    leading = leading.replace("{{alias}}", alias)
                }

                if base.contains("{{args}}") && !self.get_arguments().is_empty() {
                    let mut value = String::new();

                    for a in self.get_arguments() {
                        value.push_str(&(a.literal));
                        value.push(' ');
                    }

                    leading = leading.replace("{{args}}", value.trim());
                }

                if base.contains("{{description}}") {
                    leading = leading.replace("{{description}}", self.get_author());
                } else {
                    floating = self.get_description().into()
                }

                (leading, floating)
            }
            _ => (self.get_name().into(), self.get_description().into()),
        }
    }
}
