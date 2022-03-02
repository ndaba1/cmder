#[allow(unused_doc_comments)]
use std::collections::HashMap;

use super::super::program::Program;

#[derive(Clone)]
/// The Command struct represents the structure of a command/subcommand that can be invoked from your CLI.
/// Each of the said fields are manipulated via implementations of the struct each of which return the struct allowing for methods to be chained continously.
pub struct Cmd {
    /// The actual name of the command
    pub name: String,

    /// Any parameters, optional or otherwise, to be passed into the command
    pub params: Vec<Argument>,

    /// An alias assigned to the command, usually the first letter of the command but not necessarily
    pub alias: String,

    /// The description of the command is what gets printed out when the -h | --help flag is passed
    pub description: String,

    /// Options refer to the flags/switches that your command can receive
    pub options: Vec<Flag>,

    /// The callback is a closure that takes a ref to the command and a vec of strings, which are the actual args, it gets invoked when the passed command gets matched
    pub callback: fn(HashMap<String, String>, HashMap<String, String>) -> (),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub required: bool,
    pub literal: String,
}

#[derive(Debug, Clone, PartialEq)]
/// The Flag struct holds the fields that the `options/switches` of a given command
pub struct Flag {
    /// A short version of the switch/flag, usually begins with a single hyphen, such as -h
    pub short: String,

    /// The full/long version of the switch, usually begins with double hyphens, ie. --help
    pub long: String,

    /// Any parameters that the switch accepts, or requires
    pub params: String,

    /// A description of the flag and the inputs its accepts
    pub docstring: String,
}

impl Cmd {
    /// This function received a string slice that contains the name of the command and any params required by the said command and modifies the struct to which it is chained accordingly
    pub fn command(&mut self, val: &str) -> &mut Cmd {
        let arr: Vec<_> = val.split(' ').collect();

        self.name = arr[0].to_owned();
        for p in arr[1..].iter() {
            //TODO: Remove the angle brackets if any, and make the names camel cased
            if p.starts_with("<") {
                let name = p.replace("<", "").replace(">", "").replace("-", "_");
                self.params.push(Argument {
                    name,
                    required: true,
                    literal: p.to_string(),
                })
            } else if p.starts_with("[") {
                let name = p.replace("[", "").replace("]", "").replace("-", "_");
                self.params.push(Argument {
                    name,
                    required: false,
                    literal: p.to_string(),
                })
            }
        }

        self
    }

    /// Takes a string slice containing the desired alias of the command as input and sets it as so
    pub fn alias(&mut self, val: &str) -> &mut Cmd {
        self.alias = val.to_owned();

        self
    }

    /// The describe command is passed the description of the command, which gets printed out when the help flag is passed
    pub fn describe(&mut self, desc: &str) -> &mut Cmd {
        self.description = desc.to_owned();

        self
    }

    /// A method for adding options/flags to a command. It takes in a string slice as input in the form `short | long | params? | docsting`
    /// Each of the fields have to be separated with the pipe symbol as so
    /// If no params are required by the flag then an empty space should be passed, but not omitted.
    pub fn option(&mut self, body: &str, desc: &str) -> &mut Cmd {
        let opts: Vec<_> = body.split(' ').collect();

        let r_opts: Vec<String> = opts.iter().map(|o| o.trim().to_string()).collect();
        let params = if r_opts.len() > 2 {
            r_opts[2].clone()
        } else {
            String::from("")
        };

        let flag = Flag {
            short: r_opts[0].clone(),
            long: r_opts[1].clone(),
            params,
            docstring: desc.to_string(),
        };

        if !self.options.contains(&flag) {
            self.options.push(flag);
        }

        self
    }

    /// Takes in a closure that has two params: a ref to a command and a ref to 2 vector of Strings which are the actual args
    /// The closure returns a unit type and any program specific functionality should be implemented within the closure, such as calling a different handler.
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
        prog.cmds.push(self.clone())
    }

    /// When the command is matched and resolved from the args passed, this methos is invoked and returns a hashmap containing all the flags passed and their inputs as well as any params passed to the command itself
    pub fn parse(
        &self,
        raw_args: &Vec<String>,
    ) -> (HashMap<String, String>, HashMap<String, String>) {
        // TODO: Check if flag is unknown, act accordingly if so
        // TODO: Return Two hashmaps, one containing all the params and their values and the other containing all the flags and their values and or boolean

        let vals: Vec<&String> = raw_args.iter().filter(|a| !a.starts_with("-")).collect();
        let flags: Vec<&String> = raw_args.iter().filter(|a| a.starts_with("-")).collect();

        let mut required = vec![];
        for a in &self.params {
            if a.required {
                required.push(a.name.clone())
            }
        }

        match vals.len() {
            0 => {
                if !required.is_empty() {
                    let msg = format!("Missing required argument: {}", self.params[0].literal);
                    self.output_command_help(&msg);
                    std::process::exit(1)
                }
            }
            val if val < required.len() => {
                let msg = format!("Missing required argument: {}", self.params[val].literal);
                self.output_command_help(&msg);
                std::process::exit(1)
            }
            _ => {}
        }

        if flags.contains(&&String::from("-h")) || flags.contains(&&String::from("--help")) {
            self.output_command_help("");
            std::process::exit(1)
        }

        let mut options: HashMap<String, String> = HashMap::new();
        let mut values: HashMap<String, String> = HashMap::new();

        for f in &self.options {
            for arg in flags.iter().enumerate() {
                if arg.1 == &&f.short || arg.1 == &&f.long {
                    let name = f.long.replace("--", "").replace("-", "_");
                    options.insert(name, "true".to_string());

                    let name = f.long.replace("--", "").replace("-", "_");
                    if !f.params.is_empty() {
                        options.insert(name, raw_args[arg.0 + 1].clone());
                    }
                }
            }
        }

        for (i, k) in self.params.iter().enumerate() {
            let name = &k.name;
            values.insert(name.to_owned(), vals[i].to_owned());
        }

        (values, options)
    }

    pub fn output_command_help(&self, err: &str) {
        println!("\n{}\n", self.description);
        println!("USAGE: ");
        println!("\texe [options] command\n");
        println!("OPTIONS: ");
        for opt in &self.options {
            println!("\t{}, {}", opt.short, opt.long);
            println!("\t{}\n", opt.docstring)
        }

        if !err.is_empty() {
            println!("{}", err)
        }
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
            options: vec![Flag {
                short: "-h".to_owned(),
                long: "--help".to_owned(),
                params: "".to_owned(),
                docstring: "Displays the help command".to_owned(),
            }],
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
            }],
            callback: |_cmd, _args| {},
            description: "Some test".to_string(),
            options: vec![Flag {
                short: "-h".to_string(),
                long: "--help".to_string(),
                params: "".to_string(),
                docstring: "Displays the help command".to_string(),
            }],
        };

        let mut auto_cmd = Cmd::new();
        auto_cmd
            .command("test <app-name>")
            .alias("t")
            .describe("Some test")
            .option("-h --help", "Displays the help command")
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
