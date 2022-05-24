#![allow(unused)]
use std::{env, fmt::Debug, path::PathBuf, rc::Rc};

use crate::{
    core::errors::CmderError,
    parse::{matches::ParserMatches, parser::Parser, Argument},
    ui::formatter::FormatGenerator,
    utils::{self, HelpWriter},
    Event, Pattern, PredefinedThemes, Theme,
};

use super::events::EventListener;
use super::{
    super::parse::{CmderFlag, CmderOption},
    events::{EventConfig, EventEmitter},
    settings::{ProgramSettings, Setting},
};

type Callback = fn(ParserMatches) -> ();

/// Similar to the Command struct except commands created via the `Program::new()` method are marked as the root command and also contain the version flag automatically.
/// Exists due to maintain some familiarity with earlier versions of the crate
pub struct Program {}

impl Program {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Command<'static> {
        Command {
            flags: vec![
                CmderFlag::generate("-v", "--version", "Print out version information"),
                CmderFlag::generate("-h", "--help", "Print out help information"),
            ],
            is_root: true,
            emitter: Some(EventEmitter::default()),
            ..Command::new("")
        }
    }
}

/// The gist of the crate. Create instances of the program struct to chain to them all available methods. Event the program created is itself a command.
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
    flags: Vec<CmderFlag<'p>>,
    options: Vec<CmderOption<'p>>,
    description: Option<&'p str>,
    more_info: Option<&'p str>,
    usage_str: Option<&'p str>,
    settings: ProgramSettings,
    emitter: Option<EventEmitter>,
    subcommands: Vec<Command<'p>>,
    callback: Option<Callback>, // (cb_function, index_of_execution)
    parent: Option<Rc<Command<'p>>>,
}

impl<'d> Debug for Command<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "
            name: {},
            alias: {},
            args: {:#?},
            flags: {:#?},
            options: {:#?},
            subcmds: {:#?},
            ",
            self.name,
            self.alias.unwrap_or(""),
            self.arguments,
            self.flags,
            self.options,
            self.subcommands,
        ))
    }
}

impl<'p> Command<'p> {
    /// Simply creates a new instance of a command with the help flag added to it
    pub fn new(name: &'p str) -> Self {
        Self {
            name: name.to_string(),
            alias: None,
            arguments: vec![],
            description: None,
            flags: vec![CmderFlag::generate(
                "-h",
                "--help",
                "Print out help information",
            )],
            options: vec![],
            subcommands: vec![],
            callback: None,
            parent: None,
            more_info: None,
            version: None,
            author: None,
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            emitter: None,
            settings: ProgramSettings::default(),
            is_root: false,
            usage_str: None,
        }
    }

    // Root command options

    /// Sets the author of the program
    pub fn author(&mut self, author: &'p str) -> &mut Self {
        self.author = Some(author);
        self
    }

    /// Simply sets the version of the program
    pub fn version(&mut self, val: &'p str) -> &mut Self {
        self.version = Some(val);
        self
    }

    /// Sets the command name but only for the root command(program)
    pub fn bin_name(&mut self, val: &'p str) -> &mut Self {
        if self.is_root {
            self.name = val.into();
        }
        self
    }

    // Getters

    /// Returns the author of the program or empty value if none is set
    pub fn get_author(&self) -> &str {
        self.author.unwrap_or("")
    }

    /// Returns the provided version of the program or empty string slice
    pub fn get_version(&self) -> &str {
        self.version.unwrap_or("")
    }

    /// Returns configured theme of the program
    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns configured program pattern
    pub fn get_pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Getter for the command name
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// A getter for the command alias or empty value if none is found
    pub fn get_alias(&self) -> &str {
        self.alias.unwrap_or("")
    }

    /// Returns a reference to a vector containing all the flags of a given command
    pub fn get_flags(&self) -> &Vec<CmderFlag> {
        &self.flags
    }

    /// Returns the command description or empty string slice
    pub fn get_description(&self) -> &str {
        self.description.unwrap_or("")
    }

    /// Returns a ref to a vector containing all configured command options
    pub fn get_options(&self) -> &Vec<CmderOption> {
        &self.options
    }

    /// Returns borrowed vectot with command arguments
    pub fn get_arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    /// Returns the vector of subcommands of a command
    pub fn get_subcommands(&self) -> &Vec<Self> {
        &self.subcommands
    }

    /// Returns the parent of a given command if any
    pub fn get_parent(&self) -> Option<&Rc<Self>> {
        self.parent.as_ref()
    }

    /// Returns the more info value of a command
    pub fn get_cmd_info(&self) -> &str {
        self.more_info.unwrap_or("")
    }

    /// Returns the usage string of a command
    pub fn get_usage_str(&self) -> String {
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

    /// A utility method used to check if a subcommand is contained within a command and returns a reference to said subcommand if found
    pub fn find_subcommand(&self, val: &str) -> Option<&Command<'_>> {
        self.subcommands
            .iter()
            .find(|c| c.get_name() == val || c.get_alias() == val)
    }

    fn _set_bin_name(&mut self, val: &str) {
        if self.name.is_empty() {
            let p_buff = PathBuf::from(val);

            if let Some(name) = p_buff.file_name() {
                self.name = name.to_str().unwrap().into();
            };
        }
    }

    // Core functionality
    fn _add_args(&mut self, args: &[&str]) {
        for p in args.iter() {
            let temp = Argument::generate(p, None);
            if !self.arguments.contains(&temp) {
                self.arguments.push(temp);
            }
        }
    }

    fn _add_parent(&mut self, parent: Rc<Self>) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    #[deprecated(note = "Subcmds now built automatically")]
    pub fn build(&mut self) {}

    /// Sets the alias of a given command
    pub fn alias(&mut self, val: &'p str) -> &mut Self {
        self.alias = Some(val);
        self
    }

    /// Sets the description or help string of a command
    pub fn description(&mut self, val: &'p str) -> &mut Self {
        self.description = Some(val);
        self
    }

    /// Adds a new subcommand to an instance of a command
    pub fn subcommand(&mut self, name: &'p str) -> &mut Self {
        let parent = Rc::new(self.to_owned());

        self.subcommands.push(Self::new(name));
        self.subcommands.last_mut().unwrap()._add_parent(parent)
    }

    /// Used to register a new argument, receives the name of the argument and its help string
    pub fn argument(&mut self, val: &str, help: &str) -> &mut Self {
        let arg = Argument::generate(val, Some(help.to_string()));

        if !self.arguments.contains(&arg) {
            self.arguments.push(arg);
        }

        self
    }

    /// A method used to configure the function to be invoked when the command it is chained to is matched
    pub fn action(&mut self, cb: Callback) -> &mut Self {
        self.callback = Some(cb);
        self
    }

    fn _generate_option(&mut self, values: Vec<&'p str>, help: &'p str, r: bool) {
        let mut short = "";
        let mut long = "";
        let mut args = vec![];

        for v in &values {
            if v.starts_with("--") {
                long = v;
            } else if v.starts_with('-') {
                short = v;
            } else {
                args.push(*v);
            }
        }

        let option = CmderOption::generate(short, long, help, &args[..]).is_required(r);
        if !self.options.contains(&option) {
            self.options.push(option)
        }
    }

    /// A method to add more information to be printed with the help information of a command
    pub fn info(&mut self, val: &'p str) -> &mut Self {
        self.more_info = Some(val);
        self
    }

    /// Similar to the .option() method but it is instead used to register options that are required
    pub fn required_option(&mut self, val: &'p str, help: &'p str) -> &mut Self {
        let values: Vec<_> = val.split_whitespace().collect();
        self._generate_option(values, help, true);

        self
    }

    /// A method for adding flags thats more flexible than the default `.option()` method
    /// ```
    /// use cmder::{Command, CmderFlag};
    ///
    /// Command::new("test").add_flag(
    ///   CmderFlag::new("version")
    ///     .help("Version flag")
    ///     .short("-v")
    ///     .long("--version"),
    /// );
    /// ```
    pub fn add_flag(&mut self, flag: CmderFlag<'p>) -> &mut Self {
        self.flags.push(flag);
        self
    }

    /// This method is similar to the `add_flag` method but it applies to options as shown
    /// ```
    /// use cmder::{CmderOption, Command};
    ///
    /// Command::new("test").add_option(
    ///   CmderOption::new("port")
    ///     .help("The port option")
    ///     .short("-p")
    ///     .long("--port")
    ///     .is_required(true)
    ///     .argument("<port-number>"),
    /// );
    ///
    /// ```
    pub fn add_option(&mut self, opt: CmderOption<'p>) -> &mut Self {
        self.options.push(opt);
        self
    }

    /// Registers a new option or flag depending on the values passed along with the help string for the flag or option
    pub fn option(&mut self, val: &'p str, help: &'p str) -> &mut Self {
        let values: Vec<_> = val.split_whitespace().collect();

        let mut short = "";
        let mut long = "";
        let mut args = vec![];

        for v in &values {
            if v.starts_with("--") {
                long = v;
            } else if v.starts_with('-') {
                short = v;
            } else {
                args.push(*v);
            }
        }

        if args.is_empty() {
            let flag = CmderFlag::generate(short, long, help);
            if !self.flags.contains(&flag) {
                self.flags.push(flag)
            };
        } else {
            self._generate_option(values, help, false);
        }

        self
    }

    // Settings

    /// A method used to register a new listener to the program. It takes in a closure that will be invoked when the given event occurs
    pub fn on(&mut self, event: Event, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(event, cb, 0)
        }
    }

    /// Used to emit events and thus trigger the callbacks
    pub(crate) fn emit(&self, cfg: EventConfig) {
        if let Some(emitter) = &self.emitter {
            emitter.emit(cfg);
        }
    }

    /// A global method used to configure all settings of the program. This settings are defined in the `Setting` enum
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
    fn _handle_root_flags(&self, matches: &ParserMatches) {
        let cmd = matches.get_matched_cmd().unwrap();
        let program = matches.get_program();

        let cfg = EventConfig::new(program);
        if matches.contains_flag("-h") {
            self.emit(cfg.set_matched_cmd(cmd).set_event(Event::OutputHelp));
        } else if matches.contains_flag("-v") && cmd.is_root {
            self.emit(
                cfg.arg_c(1_usize)
                    .args(vec![program.get_version().to_string()])
                    .set_event(Event::OutputVersion),
            );
        }
    }

    fn __parse(&'p mut self, args: Vec<String>) {
        self._set_bin_name(&args[0]);

        // TODO: Rewrite this functionality
        self.__init(); // performance dip here

        let mut parser = Parser::new(self);

        match parser.parse(args[1..].to_vec()) {
            Ok(matches) => {
                self._handle_root_flags(&matches);

                if let Some(cmd) = matches.get_matched_cmd() {
                    if let Some(cb) = cmd.callback {
                        (cb)(matches);
                    }
                }
            }
            Err(e) => {
                // FIXME: No clones
                // TODO: Impl into eventcfg from cmdererror
                let clone = self.clone();
                let shared_cfg = EventConfig::new(&clone).error_str(e.clone().into());

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
        if !self.subcommands.is_empty() && self.settings.auto_include_help_subcommand {
            // Add help subcommand
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
        }

        // Means that it is the root_cmd(program)
        if let Some(emitter) = &mut self.emitter {
            let settings = &self.settings;

            use Event::*;

            emitter.on(
                OutputHelp,
                |cfg| cfg.get_matched_cmd().unwrap().output_help(),
                -4,
            );

            // Register default listeners
            if !settings.override_all_default_listeners {
                // Default behavior for errors is to print the error message
                if !settings.ignore_all_errors {
                    emitter.on_all_errors(
                        |cfg| {
                            let error = cfg.get_error_str();

                            // TODO: Improve default error handling
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
                let _output_help_ = |cfg: EventConfig| cfg.get_matched_cmd().unwrap().output_help();

                // Output help on all error events
                emitter.insert_before_all(_output_help_);
            }

            // TODO: remove this
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

    /// Builds the command and parses the args passed to it automatically
    pub fn parse(&'p mut self) {
        let args = env::args().collect::<Vec<_>>();
        self.__parse(args);
    }

    /// Builds the command and parses from the vector of string slices passed to it
    pub fn parse_from(&'p mut self, list: Vec<&str>) {
        let args = list.iter().map(|a| a.to_string()).collect::<Vec<_>>();
        self.__parse(args);
    }

    // Others

    /// Prints out help information for a command
    pub fn output_help(&self) {
        HelpWriter::write(self, self.get_theme(), self.get_pattern());
    }

    /// Method used to register a listener before all events
    pub fn before_all(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.insert_before_all(cb)
        }
    }

    /// Register a listener after all other listeners
    pub fn after_all(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.insert_after_all(cb)
        }
    }

    /// Register a listener only before help is printed out
    pub fn before_help(&mut self, cb: EventListener) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(Event::OutputHelp, cb, -4)
        }
    }

    /// Register a listener to be invoked after help is printed out
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
    fn generate(&self, ptrn: Pattern) -> (String, String) {
        match &ptrn {
            Pattern::Custom(ptrn) => {
                let base = &ptrn.sub_cmds_fmter;

                let mut leading = base.replace("{{name}}", self.get_name());
                let mut floating = String::from("");

                if let Some(alias) = self.alias {
                    leading = leading.replace("{{alias}}", alias)
                } else {
                    leading = leading.replace("{{alias}}", "")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prog_creation() {
        let mut program = Program::new();

        assert!(program.is_root);
        assert!(program.emitter.is_some());
        assert!(program.get_flags().len() == 2);
        assert!(program.get_parent().is_none());
        assert!(program.get_name().is_empty());
        assert!(program.get_version().is_empty());
        assert!(program.get_subcommands().is_empty());

        program
            .author("vndaba")
            .bin_name("test1")
            .version("0.1.0")
            .argument("<dummy>", "Some dummy value");

        assert_eq!(program.get_author(), "vndaba");
        assert_eq!(program.get_name(), "test1");
        assert_eq!(program.get_version(), "0.1.0");
        assert_eq!(
            program.get_arguments(),
            &vec![Argument::generate(
                "<dummy>",
                Some("Some dummy value".into())
            )]
        )
    }

    #[test]
    fn test_cmd_creation() {
        let cmd = Command::new("test2");

        assert!(!cmd.is_root);
        assert!(cmd.emitter.is_none());
        assert!(cmd.parent.is_none());
        assert_eq!(cmd.get_name(), "test2");
        assert_eq!(
            cmd.get_flags(),
            &vec![CmderFlag::generate(
                "-h",
                "--help",
                "Print out help information"
            )]
        );
    }
}
