use std::collections::HashMap;

use super::super::ui::{Designation, Formatter, FormatterRules};
use super::super::Program;
use super::{Argument, Flag};

#[derive(Clone)]
/// The Command struct represents the structure of a command/subcommand that can be invoked from your CLI.
/// Each of the said fields are manipulated via implementations of the struct each of which return the struct allowing for methods to be chained continously.
pub struct Cmd {
    /// The actual name of the command
    name: String,

    /// Any parameters, optional or otherwise, to be passed into the command
    params: Vec<Argument>,

    /// An alias assigned to the command, usually the first letter of the command but not necessarily
    alias: String,

    /// The description of the command is what gets printed out when the -h | --help flag is passed
    description: String,

    /// Options refer to the flags/switches that your command can receive
    options: Vec<Flag>,

    /// The callback is a closure that takes in two hashmaps, each of which contain string keys and values, the first hashmap contains all the values of the params to the given command, while the second hashmap contains the metadata for any flags passed to the command and their values if any.
    pub callback: fn(HashMap<String, String>, HashMap<String, String>) -> (),
}

impl Cmd {
    /// This function receives a string slice that contains the name of the command and any params required by the said command and modifies the struct to which it is chained accordingly.
    /// The string slice is plit by whitespace and the first value of the resulting array is set to the name of the command while the rest are set to Params of the command using the Argument struct. Depending on whether the argument starts with angle brackets or square brackets, the argument is marked as required or not. To avoid repetition, this functionality for cleaning the args was moved to the params module: `src/utils/params.rs`
    pub fn command(&mut self, val: &str) -> &mut Cmd {
        let arr: Vec<_> = val.split(' ').collect();
        self.name = arr[0].to_owned();

        for p in arr[1..].iter() {
            self.params.push(Argument::new(p, None))
        }

        self
    }

    /// Takes a string slice containing the desired alias of the command as input and sets it as so
    pub fn alias(&mut self, val: &str) -> &mut Cmd {
        self.alias = val.to_owned();

        self
    }

    /// A getter that returns the alias of a given command
    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    /// A simple method to return the configured name of a command
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// This method simply returns the configured description of a command
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Returns all the configured options of a command
    pub fn get_cmd_options(&self) -> &Vec<Flag> {
        &self.options
    }

    /// This method is similar to the `get_cmd_options` except it returns the params of a command, both required and optional ones
    pub fn get_cmd_input(&self) -> &Vec<Argument> {
        &self.params
    }

    /// The describe command is passed the description of the command, which gets printed out when the help flag is passed
    pub fn description(&mut self, desc: &str) -> &mut Cmd {
        self.description = desc.to_owned();

        self
    }

    /// A method for adding options/flags to a command. It takes in two string slices as input in the form: `short long params?`, `docstring`
    /// The params field is optional, but if included, follows the same rules as the params in the command method above.
    pub fn option(&mut self, body: &str, desc: &str) -> &mut Cmd {
        let flag = Flag::new(body, desc);

        if !self.options.contains(&flag) {
            self.options.push(flag);
        }

        self
    }

    /// Takes in the actual callback function that is called once all the parsing is done and the command resolved. The closure takes in two hashmaps and returns a unit type.
    /// Any extra functionality can be implemented here. Such as calling a different handler or anything else.
    pub fn action(
        &mut self,
        cb: fn(HashMap<String, String>, HashMap<String, String>) -> (),
    ) -> &mut Cmd {
        self.callback = cb;

        self
    }

    /// Receives the instance of the program as input and pushes the constructed command to the `cmds` field of the program struct
    /// This should be the last method to be chained as it returns a unit type.
    pub fn build(&mut self, prog: &mut Program) {
        // TODO: avoid mutating the cmds field like this
        prog.add_cmd(self.to_owned());
    }

    pub fn output_command_help(&self, prog: &Program, err: &str) {
        let mut fmtr = Formatter::new(prog.get_theme().to_owned());

        use Designation::*;

        fmtr.add(Description, &format!("\n{}\n", self.description));
        fmtr.add(Headline, "\nUSAGE: \n");

        let mut params = String::new();
        for p in &self.params {
            params.push_str(p.literal.as_str());
            params.push(' ');
        }

        fmtr.add(
            Keyword,
            &format!("   {} {} ", prog.get_bin_name(), self.name),
        );
        fmtr.add(Description, &format!("[options] {} \n", params.trim()));

        fmtr.add(Headline, "\nOPTIONS: \n");
        fmtr.format(
            FormatterRules::Option(prog.get_pattern().to_owned()),
            Some(self.options.clone()),
            None,
            None,
        );

        if !err.is_empty() {
            fmtr.add(Error, &format!("\nError: {}\n", err))
        }

        fmtr.print();
    }
}

impl Cmd {
    /// Returns a boilerplate new instance of the Cmd struct to which methods are then chained to modify the Cmd values.
    /// Also contains the default help flag as it is common in most if not all cmds.
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            params: vec![],
            alias: "".to_owned(),
            description: "".to_owned(),
            options: vec![Flag::new(
                "-h --help",
                "Output help information for a command",
            )],
            callback: |_cmd, _args| {},
        }
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new_cmd_fn() {
        let cmd = Cmd {
            name: "test".to_string(),
            alias: "t".to_string(),
            params: vec![Argument {
                name: "app_name".to_string(),
                required: true,
                literal: "<app-name>".to_string(),
                description: None,
            }],
            callback: |_cmd, _args| {},
            description: "Some test".to_string(),
            options: vec![Flag::new(
                "-h --help",
                "Output help information for a command",
            )],
        };

        let mut auto_cmd = Cmd::new();
        auto_cmd
            .command("test <app-name>")
            .alias("t")
            .description("Some test")
            .option("-h --help", "Output help information for a command")
            .action(|_cmd, _args| {});

        assert_eq!(
            cmd.name, auto_cmd.name,
            "Testing that {} = {}",
            cmd.name, auto_cmd.name
        );
        assert_eq!(cmd.params, auto_cmd.params);
        assert_eq!(cmd.options, auto_cmd.options);
    }
}
