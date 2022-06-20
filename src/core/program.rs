#![allow(unused)]
use std::{env, fmt::Debug, path::PathBuf, rc::Rc};

use crate::{
    core::errors::CmderError,
    parse::{
        flags::new_flag,
        matches::ParserMatches,
        options::new_option,
        parser::{NewParser, Parser},
        Argument,
    },
    ui::{formatter::FormatGenerator, themes::get_predefined_theme},
    utils::{self, HelpWriter},
    Event, Pattern, PredefinedTheme, Theme,
};

use super::events::{EventCallback, EventListener};
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
                CmderFlag::new("version")
                    .short('V')
                    .help("Print out version information"),
                CmderFlag::new("help")
                    .short('h')
                    .help("Print out help information"),
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
    flags: Vec<CmderFlag>,
    options: Vec<CmderOption>,
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
            flags: vec![CmderFlag::new("help")
                .short('h')
                .help("Print out help information")],
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

    /************************************* Getters ********************************************/

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

    /********************************** Command Metadata methods **********************************/

    /// A simple method for setting the program author. Typically invoked on the root cmd
    pub fn author(&mut self, author: &'p str) -> &mut Self {
        self.author = Some(author);
        self
    }

    /// This method simply sets the version of the program.
    pub fn version(&mut self, val: &'p str) -> &mut Self {
        self.version = Some(val);
        self
    }

    /// A method to override the name of the root command(the Program). This method doesn't change the actual binary name, only the value displayed to users when printing help
    pub fn bin_name(&mut self, val: &'p str) -> &mut Self {
        if self.is_root {
            self.name = val.into();
        }
        self
    }

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

    /// A method used to register a new argument, accepts the name of the argument and its help string. Arguments enclosed in `< >` are marked as required while those in `[ ]` are optional. Defaults to optional if no enclosing provided.
    ///
    /// ```
    /// use cmder::{Command};
    ///
    /// Command::new("basic")
    ///     .argument("<name>", "Some basic name value");
    /// ```
    pub fn argument(&mut self, val: &str, help: &str) -> &mut Self {
        self.add_argument(Argument::new(val).help(help));
        self
    }

    /// A method for adding an argument to a command normally when using the builder method to create the argument.
    ///
    /// ```
    /// use cmder::{Command, Argument};
    ///
    /// Command::new("basic")
    ///     .add_argument(
    ///         Argument::new("language")
    ///             .required(true)
    ///             .help("The language to use")
    ///             .variadic(false)
    ///             .valid_values(vec!["ENG", "SPA", "FRE"])
    ///     );
    /// ```
    pub fn add_argument(&mut self, arg: Argument) -> &mut Self {
        if !self.arguments.contains(&arg) {
            self.arguments.push(arg);
        }
        self
    }

    /// A method used to configure the function to be invoked when the command it is chained to is matched
    ///
    /// ```
    /// use cmder::{Program};
    ///
    /// let mut program = Program::new();
    ///
    /// program
    ///     .subcommand("basic")
    ///     .description("A basic subcmd")
    ///     .action(|_matches|{
    ///         println!("Basic subcmd matched!!")
    ///     });
    ///
    ///
    /// ```
    pub fn action(&mut self, cb: Callback) -> &mut Self {
        self.callback = Some(cb);
        self
    }

    /// A method for adding a new subcommand to a command instance. It returns the newly created subcommand for further manipulation
    /// ```
    /// use cmder::{Program};
    ///
    /// let mut program = Program::new();
    ///
    /// program
    ///     .subcommand("subcmd")
    ///     .description("A simple subcmd");
    ///
    /// ```
    pub fn subcommand(&mut self, name: &'p str) -> &mut Self {
        let parent = Rc::new(self.to_owned());

        self.subcommands.push(Self::new(name));
        self.subcommands.last_mut().unwrap()._add_parent(parent)
    }

    /// A method to add more information to be printed with the help information of a command
    pub fn info(&mut self, val: &'p str) -> &mut Self {
        self.more_info = Some(val);
        self
    }

    /// A method for adding flags thats more flexible than the default `.option()` method
    /// ```
    /// use cmder::{Command, CmderFlag};
    ///
    /// Command::new("test").add_flag(
    ///   CmderFlag::new("version")
    ///     .help("Version flag")
    ///     .short('-v')
    /// );
    /// ```
    pub fn add_flag(&mut self, flag: CmderFlag) -> &mut Self {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
        self
    }

    /// A simpler method for adding a flag to a command when you do not need to manipulate the structure of a flag. An example is shown below:
    /// ```
    /// use cmder::{Command, CmderFlag};
    ///
    /// Command::new("test")
    ///     .flag("-v --verbose", "show verbose output")
    ///     .flag("-x --extra", "some extra flag");
    ///
    /// ```
    pub fn flag(&mut self, val: &'p str, help: &'static str) -> &mut Self {
        self.add_flag(new_flag(val, help));
        self
    }

    /// A method for adding an option to a command suitable when using the option builder interface which provides more methods for option manipulation
    /// ```
    /// use cmder::{CmderOption, Command};
    ///
    /// Command::new("test").add_option(
    ///   CmderOption::new("port")
    ///     .help("The port option")
    ///     .short('p')
    ///     .required(true)
    ///     .argument("<port-number>"),
    /// );
    ///
    /// ```
    pub fn add_option(&mut self, opt: CmderOption) -> &mut Self {
        if !self.options.contains(&opt) {
            self.options.push(opt);
        }
        self
    }

    /// Adds a new option to a command. Accepts the option syntax value and the help string
    ///
    /// ```
    /// use cmder::{Command};
    ///
    /// Command::new("empty")
    ///     .option("-p --port <port-no>", "The port to use")
    ///     .option("-c --count <number>", "Some count value");
    ///
    /// ```
    pub fn option(&mut self, val: &'p str, help: &'static str) -> &mut Self {
        self.add_option(new_option(val, help, false));
        self
    }

    /// A method for adding an option that is marked as required without necessarily using the builder interface to generate the option
    /// ```
    /// use cmder::{Command};
    ///
    /// Command::new("empty")
    ///     .required_option("-p --port <port-no>", "The port to use");
    ///
    /// ```
    pub fn required_option(&mut self, val: &'p str, help: &'static str) -> &mut Self {
        self.add_option(new_option(val, help, true));
        self
    }

    /********************************* Utility Methods ***********************************/

    /// A utility method used to try and find a subcommand within a command.
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

    fn _add_parent(&mut self, parent: Rc<Self>) -> &mut Self {
        self.parent = Some(parent);
        self
    }

    /********************************* Event Emitter funcs ***********************************/

    /// A method used to register a new listener to the program. It takes in a closure that will be invoked when the given event occurs
    ///
    /// ```
    /// use cmder::{Program, Event};
    ///
    /// let mut program = Program::new();
    ///
    /// program
    ///     .on(Event::OutputVersion, |_cfg|{
    ///     // logic goes here...
    ///     });
    ///
    ///
    /// ```
    pub fn on(&mut self, event: Event, cb: EventCallback) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(event, cb, 0)
        }
    }

    // A method similar to the `on` method, the only difference being that this method not only adds a new listener, but also overrides the default one.
    pub fn override_default(&mut self, event: Event, cb: EventCallback) {
        if let Some(emitter) = &mut self.emitter {
            emitter.override_event(event);
            emitter.on(event, cb, 0);
        }
    }

    /// A simple method used to register a listener before all events
    pub fn before_all(&mut self, cb: EventCallback) {
        if let Some(emitter) = &mut self.emitter {
            emitter.insert_before_all(cb)
        }
    }

    /// A method to register a listener after all other listeners
    pub fn after_all(&mut self, cb: EventCallback) {
        if let Some(emitter) = &mut self.emitter {
            emitter.insert_after_all(cb)
        }
    }

    /// Register a listener only before help is printed out
    pub fn before_help(&mut self, cb: EventCallback) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(Event::OutputHelp, cb, -4)
        }
    }

    /// Register a listener to be invoked after help is printed out
    pub fn after_help(&mut self, cb: EventCallback) {
        if let Some(emitter) = &mut self.emitter {
            emitter.on(Event::OutputHelp, cb, 1)
        }
    }

    /// Used to emit events and thus trigger the callbacks
    pub(crate) fn emit(&self, cfg: EventConfig) {
        if let Some(emitter) = &self.emitter {
            emitter.emit(cfg);
        }
    }

    /********************************* Command Settings ***********************************/

    /// A method used to configure all settings of the program. This settings are defined in the `Setting` enum and are boolean values.
    ///
    /// ```
    /// use cmder::{Program, Setting};
    ///
    /// let mut p = Program::new();
    ///
    /// p.set(Setting::ShowHelpOnAllErrors, true);
    /// p.set(Setting::HideCommandAliases, false);
    /// // other settings...
    /// ```
    pub fn set(&mut self, setting: Setting, val: bool) {
        self.settings.set(setting, val);
    }

    /// A method to configure the program to use a predefine theme from the cmder crate.
    pub fn use_predefined_theme(&mut self, theme: PredefinedTheme) -> &mut Self {
        self.theme = get_predefined_theme(theme);
        self
    }

    /// A method to configure the theme to be used by the program. You can also use the method to define your own custom theme.
    ///
    /// ```
    /// use cmder::{Program, Theme};
    ///
    /// let mut program = Program::new();
    ///
    /// program.theme(Theme::new(Green, Magenta, Blue, Red, White));
    ///
    /// ```
    pub fn theme(&mut self, theme: Theme) -> &mut Self {
        self.theme = theme;
        self
    }

    /********************************* Parser functionality ***********************************/

    fn _handle_root_flags(&self, matches: &ParserMatches) {
        // let cmd = matches.get_matched_cmd().unwrap();
        // let program = matches.get_program();

        // let cfg = EventConfig::new(program);
        // if matches.contains_flag("-h") {
        //     self.emit(cfg.set_matched_cmd(cmd).set_event(Event::OutputHelp));
        // } else if matches.contains_flag("-v") && cmd.is_root {
        //     self.emit(
        //         cfg.arg_c(1_usize)
        //             .args(vec![program.get_version().to_string()])
        //             .set_event(Event::OutputVersion),
        //     );
        // }
    }

    fn __parse(&'p mut self, args: Vec<String>) {
        self._set_bin_name(&args[0]);

        // TODO: Rewrite this functionality
        self.__init(); // performance dip here

        {
            let mut parser = NewParser::new(self);
            parser.parse(["hey", "you"])
        }

        let mut parser = Parser::new(self);

        match parser.parse(args[1..].to_vec()) {
            Ok(matches) => {
                self._handle_root_flags(&matches);

                if let Some(cmd) = matches.get_matched_cmd() {
                    if let Some(cb) = cmd.callback {
                        // if matches.get_raw_args_count() <= 1
                        //     && cmd.settings.get(Setting::ShowHelpOnEmptyArgs)
                        // {
                        //     cmd.output_help();
                        //     return;
                        // }
                        (cb)(matches);
                    } else {
                        cmd.output_help();
                    }
                }
            }
            Err(e) => {
                // FIXME: No clones
                // TODO: Impl into eventcfg from cmdererror

                // TODO: Implement new Error handling
            }
        }
    }

    fn __init(&mut self) {
        if !self.subcommands.is_empty() && self.settings.get(Setting::AutoIncludeHelpSubcommand) {
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

            // All default listeners have a pstn of -4. Any listeners created using the `before_all` have a pstn of -5 and listeners created by the `program.on()` method have a pstn of 0. When an event occurs, these listeners are sorted according to pstn and executed in said order.

            // Default help listener - Cannot be overriden
            emitter.on(
                OutputHelp,
                |cfg| cfg.get_matched_cmd().unwrap().output_help(),
                -4,
            );

            // Register default listeners
            if !settings.get(Setting::OverrideAllDefaultListeners) {
                // Default behavior for errors is to print the error message
                if !settings.get(Setting::IgnoreAllErrors) {
                    emitter.on_errors(
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

                        // TODO: output version in better way
                        println!("{}, v{}", p.get_name(), p.get_version());
                        println!("{}", p.get_author());
                        println!("{}", p.get_description());
                    },
                    -4,
                );

                // Remove default listeners if behavior set to override
                for event in emitter.clone().get_events_to_override() {
                    emitter.rm_default_lstnr(*event)
                }
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
                        value.push_str(&(a.get_raw_value()));
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
            _ => {
                let mut leading: String = self.get_name().into();

                (leading, self.get_description().into())
            }
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
            &vec![Argument::new("<dummy>").help("Some dummy value")]
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
            &vec![CmderFlag::new("help")
                .short('h')
                .help("Print out help information")]
        );
    }
}
