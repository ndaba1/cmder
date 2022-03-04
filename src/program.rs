use crate::parser::resolve_flag;
use crate::{PredefinedThemes, Theme};

use super::parser::{Cmd, Flag};
use super::ui::{Designation, Formatter, FormatterRules, Pattern};
use super::{Event, EventEmitter};

/// The crux of the library, the program holds all information about your cli. It contains a vector field that stores all the commands that can be invoked from your program and also stores some metadata about your program
pub struct Program {
    /// Stores all the commands that your program contains. You won't have to deal with this field directly rather by calling specific methods that allow you to build commands and add them to this vector
    pub cmds: Vec<Cmd>,

    /// Hold the version information of your program, It's gets printed out when the -v | --version flag is passed as an arg
    pub version: String,

    /// Optional metadata that contains the author of the program, also gets printed out when the -v flag is passed to the program
    pub author: String,

    /// A short description of what the program does, usually the programs tagline. It gets printed out when the version and help flags are passed
    pub about: String,

    /// A vector containing the flags/swicthed that can be passed to the root instance of the program and not the subcommands
    pub options: Vec<Flag>,

    /// An instance of the EventEmitter struct that the program can use to emit and listen to events. The program also contains utility functions to interface with the event_emitter which it contains.
    pub event_emitter: EventEmitter,

    /// The name of the program. It is obtained from the args passed to the cli and is used when printing help information, or any other cases that require the program name
    pub name: String,

    /// Refers to the pattern to be used by the proram when printing to stdout. Patterns can be selected from the default ones or you can create your own pattern.
    /// This field is customized by calling the `set_pattern` method
    pub pattern: Pattern,

    /// Similar to the pattern field in that they are both concerned with stdout. The theme hover differs in that it refers to the color schemes to be used by the program.
    /// There is a default theme, some predefined themes and you can also create your own custom theme.
    pub theme: Theme,
}

impl Program {
    /// Creates a simple, blank instance of the program struct to which methods then get chained allowing the fields to be mutated accordingly
    pub fn new() -> Self {
        Self {
            cmds: vec![],
            name: "".to_owned(),
            about: "".to_owned(),
            theme: Theme::default(),
            author: "".to_owned(),
            pattern: Pattern::Legacy,
            version: "0.1.0".to_owned(),
            event_emitter: EventEmitter::new(),
            options: vec![
                Flag::new("-h --help", "Output help for the program"),
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

    /// A method that receives a mutable ref to a program and a description, and mutates the about field in the program struct then returns another mutable ref to the program
    pub fn description(&mut self, desc: &str) -> &mut Program {
        self.about = desc.to_string();
        self
    }

    /// A simple method that takes in a ref to self allowing it to be an associated method, then returns a new Cmd struct that can be manipulated and when the build method is called, the constructed command is pushed onto the cmds field
    pub fn add_cmd(&self) -> Cmd {
        Cmd::new()
    }

    /// A private utility function that receives the first argument passed to the program, being the path to the binary file and extracts the name of the executable to be set as the name of the program and utilized when printing out help information.
    ///
    /// The behavior of this function can be overriden by using the .bin_name() method. The method can be used when the name to be displayed to the users is different from the actual name of the executable binary.
    fn get_target_name(&self, val: String) -> String {
        if cfg!(windows) {
            let path_buff: Vec<&str> = val.split('\\').collect();
            let target = path_buff.last().unwrap();
            target.replace(".exe", "")
        } else {
            let path_buff: Vec<&str> = val.split('/').collect();
            let target = path_buff.last().unwrap();
            target.to_string()
        }
    }

    /// This method receives the raw arguments passed to the program, and tries to get matches from all the configured commands or flags
    /// If no command is matched, it either acts in a default manner or executes the configured callbacks if any
    /// Also checks for the help and version flags.
    pub fn parse(&mut self) {
        let raw_args: Vec<String> = std::env::args().collect();
        let args = raw_args[1..].to_vec();

        self.name = self.get_target_name(raw_args[0].clone());

        if args.is_empty() {
            self.output_help("You did not pass a command!");
            self.emit(Event::OutputHelp, "");
            return;
        }

        match args[0].to_lowercase().as_str() {
            val if val.starts_with('-') => self.get_matches(val),
            val if self.cmds.iter().any(|c| c.name == val || c.alias == val) => {
                let cmd = self.get_cmd(val).unwrap();
                let (vals, opts) = cmd.parse(self, &args[1..].to_vec());
                (cmd.callback)(vals, opts);
            }
            val => {
                self.emit(Event::UnknownCommand, val);
                let msg = format!("Unknown command \"{}\"", val);
                self.output_help(msg.as_str());
            }
        }
    }

    /// A method that try to get matches for any flags passed to the program itself, rather than a subcommand of the program.
    fn get_matches(&self, val: &str) {
        if let Some(v) = resolve_flag(&self.options, val) {
            if v.short.as_str() == "-h" {
                self.output_help("");
                self.emit(Event::OutputHelp, "");
            } else if v.short.as_str() == "-v" {
                self.emit(Event::OutputVersion, self.version.as_str());
                self.output_version_info()
            }
        } else {
            self.emit(Event::UnknownOption, val);
            let msg = format!("Unknown option \"{}\"", val);
            self.output_help(msg.as_str());
        }
    }

    /// A simple method that tries to find a command from a given string slice that can either be the name of the command or its alias.
    pub fn get_cmd(&self, val: &str) -> Option<&Cmd> {
        self.cmds.iter().find(|c| c.alias == val || c.name == val)
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
        self.pattern = ptrn
    }

    /// Similar to the set_pattern() method only that this one is used to set a theme from a list of predefined ones.
    pub fn set_theme(&mut self, theme: PredefinedThemes) {
        match theme {
            PredefinedThemes::Plain => self.theme = Theme::plain(),
            PredefinedThemes::Colorful => self.theme = Theme::colorful(),
        }
    }

    /// When you wish to define your own custom theme, the set_custom_theme method is what is to be used. The method receives a theme struct with all the  fields configured accordingly.
    pub fn set_custom_theme(&mut self, theme: Theme) {
        self.theme = theme
    }

    /// This method is used to output help information to standard out. It uses the themes and patterns configured for the program.
    pub fn output_help(&self, err: &str) {
        let mut fmtr = Formatter::new(self.theme.clone());

        use Designation::*;

        fmtr.add(Description, &format!("\n{}\n", self.about));
        fmtr.add(Headline, "\nUSAGE: \n");
        fmtr.add(Keyword, &format!("   {} ", self.name));
        fmtr.add(Description, "<COMMAND> [options] \n");

        fmtr.add(Headline, "\nOPTIONS: \n");
        fmtr.format(
            FormatterRules::Option(self.pattern.clone()),
            Some(self.options.clone()),
            None,
        );

        fmtr.add(Headline, "\nCOMMANDS: \n");
        fmtr.format(
            FormatterRules::Cmd(self.pattern.clone()),
            None,
            Some(self.cmds.clone()),
        );

        if !err.is_empty() {
            fmtr.add(Error, &format!("\nError: {}\n", err))
        }

        fmtr.print();

        self.emit(Event::OutputHelp, "");
    }

    /// Simply outputs the name and version of the program. As well as the author information and program description.
    pub fn output_version_info(&self) {
        let mut fmtr = Formatter::new(self.theme.clone());

        use Designation::*;

        fmtr.add(Keyword, &format!("{}, v{}\n", self.name, self.version));
        fmtr.add(Description, &format!("{}\n", self.about));
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
            options: vec![],
            name: "".to_owned(),
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            version: "0.1.0".to_string(),
            author: "me".to_string(),
            about: "a test".to_string(),
            event_emitter: EventEmitter::new(),
        };

        assert_eq!(auto.author, manual.author);
        assert_eq!(auto.about, manual.about);
    }

    #[test]
    fn test_add_cmd_func() {
        let mut prog = Program::new();

        prog.add_cmd()
            .command("name <some-name>")
            .alias("n")
            .describe("some random command")
            .build(&mut prog);

        assert_eq!(prog.cmds.len(), 1);
    }
}
