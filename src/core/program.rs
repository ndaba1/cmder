use std::collections::HashMap;

use crate::parse::{Argument, Cmd, Flag, Parser};
use crate::ui::{Designation, Formatter, Pattern, PredefinedThemes, Theme};
use crate::utils::{print_help, suggest};

use super::{Event, EventEmitter, ProgramSettings};

type Callback = fn(HashMap<String, String>, HashMap<String, String>) -> ();
/// The crux of the library, the program holds all information about your cli. It contains a vector field that stores all the commands that can be invoked from your program and also stores some metadata about your program
pub struct Program {
    /// Stores all the commands that your program contains. You won't have to deal with this field directly rather by calling specific methods that allow you to build commands and add them to this vector
    cmds: Vec<Cmd>,

    /// Hold the version information of your program, It's gets printed out when the -v | --version flag is passed as an arg
    version: String,

    /// Optional metadata that contains the author of the program, also gets printed out when the -v flag is passed to the program
    author: String,

    /// A short description of what the program does, usually the programs tagline. It gets printed out when the version and help flags are passed
    description: String,

    /// The name of the program. It is obtained from the args passed to the cli and is used when printing help information, or any other cases that require the program name
    name: String,

    /// A vector containing the flags/swicthed that can be passed to the root instance of the program and not the subcommands
    options: Vec<Flag>,

    /// A vector holding args/params that can be passed to the program itself directly, rather than to its commands.
    arguments: Vec<Argument>,

    /// This is applicable in cases where the program can be executed directly without necessarily requiring a command to be passed to it
    callback: Option<Callback>,

    /// An instance of the EventEmitter struct that the program can use to emit and listen to events. The program also contains utility functions to interface with the event_emitter which it contains.
    event_emitter: EventEmitter,

    /// A collection of settings including the theme and pattern used by the program that determine its default behavior
    settings: ProgramSettings,
}

impl Program {
    /// Creates a simple, blank instance of the program struct to which methods then get chained allowing the fields to be mutated accordingly
    pub fn new() -> Self {
        Self {
            cmds: vec![],
            name: "".to_owned(),
            description: "".to_owned(),
            author: "".to_owned(),
            callback: None,
            version: "0.1.0".to_owned(),
            arguments: vec![],
            event_emitter: EventEmitter::new(),
            settings: ProgramSettings::default(),
            options: vec![
                Flag::new("-h --help", "Output help information for the program"),
                Flag::new("-v --version", "Output the version info for the program"),
            ],
        }
    }

    /// A simple method for setting the version info of the program. It can be chained onto an instance of a program and also returns a mutable ref to the program allowing more methods to be chained on.
    pub fn version(&mut self, vers: &str) -> &mut Program {
        self.version = vers.to_string();
        self
    }

    /// A method for setting the author information of the program, it acts in the same manner as the version method.
    pub fn author(&mut self, auth: &str) -> &mut Program {
        self.author = auth.to_string();
        self
    }

    /// A method to override the program name displayed to users when printing out help information. This method can be used when the name of the executable is different from the name to be displayed.
    pub fn bin_name(&mut self, name: &str) -> &mut Program {
        self.name = name.to_string();
        self
    }

    /// A method that receives a mutable ref to a program and a description, and mutates the about field in the program struct then returns another mutable ref to the program
    pub fn description(&mut self, desc: &str) -> &mut Program {
        self.description = desc.to_string();
        self
    }

    /// A simpler way to register a command to the program. Instead of chaining the .add_cmd() method and the command method, this method does it for you.
    pub fn command(&self, val: &str) -> Cmd {
        let mut cmd = Cmd::new();
        cmd.command(val);
        cmd
    }

    /// A simple method that takes in a ref to self allowing it to be an associated method, then returns a new Cmd struct that can be manipulated and when the build method is called, the constructed command is pushed onto the cmds field
    pub fn add_cmd(&mut self, cmd: Cmd) {
        self.cmds.push(cmd);
    }

    /// A getter for the version information for the program instance
    pub fn get_version(&self) -> &str {
        &self.version
    }

    /// A getter for the author information
    pub fn get_author(&self) -> &str {
        &self.author
    }

    /// Returns the configured name of the executable
    pub fn get_bin_name(&self) -> &str {
        &self.name
    }

    /// A getter that returns the description of the program
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Returns a reference to the vector containing all the commands configured into the program.
    pub fn get_all_cmds(&self) -> &Vec<Cmd> {
        &self.cmds
    }

    /// A getter for the theme of the program
    pub fn get_theme(&self) -> &Theme {
        &self.settings.theme
    }

    /// A getter for the configured pattern of the program
    pub fn get_pattern(&self) -> &Pattern {
        &self.settings.pattern
    }

    /// Returns a reference to the vector containing all the options configured into the program.
    pub fn get_options(&self) -> &Vec<Flag> {
        &self.options
    }

    /// This method is similar to the `get_options` except it returns the params of the program, both required and optional ones
    pub fn get_input(&self) -> &Vec<Argument> {
        &self.arguments
    }

    /// A private utility function that receives the first argument passed to the program, being the path to the binary file and extracts the name of the executable to be set as the name of the program and utilized when printing out help information.
    /// The behavior of this function can be overriden by using the .bin_name() method. The method can be used when the name to be displayed to the users is different from the actual name of the executable binary.
    fn get_target_name(&self, val: String) -> String {
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
            self.name.clone()
        }
    }

    pub fn argument(&mut self, val: &str, body: &str) -> &mut Program {
        let arg = Argument::new(val, Some(body.to_string()));

        self.arguments.push(arg);
        self
    }

    /// A method for adding options/flags directly to the program. It follows the same rules as the Cmd.options() method
    pub fn option(&mut self, body: &str, desc: &str) -> &mut Program {
        let flag = Flag::new(body, desc);

        if !self.options.contains(&flag) {
            self.options.push(flag);
        }

        self
    }

    /// This method receives the raw arguments passed to the program, and tries to get matches from all the configured commands or flags
    /// If no command is matched, it either acts in a default manner or executes the configured callbacks if any
    /// Also checks for the help and version flags.
    fn _parse(&mut self, args: Vec<String>) {
        if args.is_empty() {
            let msg = if self.arguments.is_empty() && !self.cmds.is_empty() {
                "You did not pass a command".to_string()
            } else if !self.arguments.is_empty() && self.cmds.is_empty() {
                let arg = self.arguments.first().unwrap();
                format!("Missing required argument: {}", arg.literal)
            } else {
                "No command or arguments passed".to_string()
            };
            self.output_help(&msg);
            self.emit(Event::OutputHelp, "");
            return;
        }

        let first_arg = args[0].to_lowercase();
        let mut parent_cmd: Option<Cmd> = None;
        let parent = if self
            .cmds
            .iter()
            .any(|c| c.get_name() == first_arg || c.get_alias() == first_arg)
        {
            "cmd"
        } else if self.cmds.iter().any(|c| {
            if c.get_subcommands()
                .iter()
                .any(|s| s.get_name() == first_arg || s.get_alias() == first_arg)
            {
                parent_cmd = Some(c.clone());
                return true;
            }
            false
        }) {
            "subcmd"
        } else {
            "program"
        };

        match parent {
            "cmd" => {
                let cmd = self.get_cmd(&first_arg).unwrap();

                if args.len() >= 2
                    && cmd
                        .get_subcommands()
                        .iter()
                        .any(|sc| sc.get_alias() == args[1] || sc.get_name() == args[1])
                {
                    self._parse(args[1..].to_vec())
                } else {
                    let parser = Parser::new(self, Some(cmd));
                    let (values, options) = parser.parse(parent, &args[1..].to_vec());
                    (cmd.callback)(values, options);
                }
            }
            "subcmd" => {
                let p_cmd = parent_cmd.unwrap();
                let cmd = p_cmd.find_subcmd(&first_arg).unwrap();

                let parser = Parser::new(self, Some(cmd));
                let (values, options) = parser.parse("cmd", &args[1..].to_vec());
                (cmd.callback)(values, options);
            }
            _ => {
                let parser = Parser::new(self, None);
                let (vals, opts) = parser.parse(parent, &args);

                if !self.arguments.is_empty() && self.callback.is_some() {
                    (self.callback.unwrap())(vals, opts);
                } else {
                    self.emit(Event::UnknownCommand, &first_arg);
                    if let Some(cmd) = suggest(&first_arg, self.get_all_cmds()) {
                        println!("error: Unknown command: {first_arg}");
                        println!();
                        println!("       did you mean \"{cmd}\"");
                    }
                    // let msg = format!("Unknown command \"{}\"", &first_arg);
                    // self.output_help(&msg);
                    std::process::exit(1);
                }
            }
        }
    }

    /// This method automatically receives the arguments passed to the program itself and parses the arguments accordingly.
    pub fn parse(&mut self) {
        let raw_args: Vec<String> = std::env::args().collect();
        let args = raw_args[1..].to_vec();

        self.name = self.get_target_name(raw_args[0].clone());
        self._parse(args);
    }

    /// Similar to the parse function with one fundamental difference. The parse function receives no arguments and will automatically get them from the args passed to the program. However, the parse from requires the args to parse to be passed to it as a vector of string slices.
    pub fn parse_from(&mut self, values: Vec<&str>) {
        let mut args = vec![];
        for v in values {
            args.push(v.to_string())
        }
        self._parse(args)
    }

    /// Used to set the callback function that will be executed when the program is executed directly rather than a subcommand of the program.
    pub fn action(
        &mut self,
        cb: fn(HashMap<String, String>, HashMap<String, String>) -> (),
    ) -> &mut Program {
        self.callback = Some(cb);

        self
    }

    /// A simple method that tries to find a command from a given string slice that can either be the name of the command or its alias.
    pub fn get_cmd(&self, val: &str) -> Option<&Cmd> {
        self.cmds
            .iter()
            .find(|c| c.get_alias() == val || c.get_name() == val)
    }

    /// This method is used to set event listeners. It doesn't set the listeners itself but rather calls a similar method on the program's configured event emitter which then registers the listener.
    pub fn on(&mut self, event: Event, callback: fn(&Program, String) -> ()) {
        self.event_emitter.on(event, callback);
    }

    /// A similar method to the .on() method. This method is called when events occur and are therefore `emitted`, after which any listeners are invoked. Just like the on method, it doesnt actually invoke the listeners itself, but interfaces with the event_emitter.
    pub fn emit(&self, event: Event, param: &str) {
        self.event_emitter.emit(self, event, param.to_owned())
    }

    /// This method receives a pattern and simply modifies the pattern of the program.
    pub fn set_pattern(&mut self, ptrn: Pattern) {
        self.settings.pattern = ptrn
    }

    /// Similar to the set_pattern() method only that this one is used to set a theme from a list of predefined ones.
    pub fn set_theme(&mut self, theme: PredefinedThemes) {
        match theme {
            PredefinedThemes::Plain => self.settings.theme = Theme::plain(),
            PredefinedThemes::Colorful => self.settings.theme = Theme::colorful(),
        }
    }

    /// When you wish to define your own custom theme, the set_custom_theme method is what is to be used. The method receives a theme struct with all the  fields configured accordingly.
    pub fn set_custom_theme(&mut self, theme: Theme) {
        self.settings.theme = theme
    }

    /// This method is used to output help information to standard out. It uses the themes and patterns configured for the program.
    pub fn output_help(&self, err: &str) {
        print_help(self, None, err)
    }

    /// Simply outputs the name and version of the program. As well as the author information and program description.
    pub fn output_version_info(&self) {
        let mut fmtr = Formatter::new(self.settings.theme.clone());

        use Designation::*;

        fmtr.add(Keyword, &format!("{}, v{}\n", self.name, self.version));
        fmtr.add(Description, &format!("{}\n", self.description));
        fmtr.add(Other, &format!("{}\n", self.author));

        fmtr.print();
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_prog() {
        let mut auto = Program::new();
        auto.author("me").description("a test");

        let manual = Program {
            cmds: vec![],
            name: "".to_owned(),
            description: "a test".to_string(),
            callback: None,
            author: "me".to_string(),
            options: vec![],
            version: "0.1.0".to_string(),
            arguments: vec![],
            event_emitter: EventEmitter::new(),
            settings: ProgramSettings::default(),
        };

        assert_eq!(auto.author, manual.author);
        assert_eq!(auto.description, manual.description);
    }

    #[test]
    fn test_add_cmd_func() {
        let mut prog = Program::new();

        prog.command("name <some-name>")
            .alias("n")
            .description("some random command")
            .build(&mut prog);

        assert_eq!(prog.cmds.len(), 1);
    }
}
