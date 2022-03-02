use super::parser::{Cmd, Flag};

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
}

impl Program {
    /// Creates a simple, blank instance of the program struct to which methods then get chained allowing the fields to be mutated accordingly
    pub fn new() -> Self {
        Self {
            cmds: vec![],
            version: "0.1.0".to_owned(),
            author: "".to_owned(),
            about: "".to_owned(),
            options: vec![
                Flag {
                    short: "-h".to_string(),
                    long: "--help".to_string(),
                    params: "".to_string(),
                    docstring: "Output help for the program".to_string(),
                },
                Flag {
                    short: "-v".to_string(),
                    long: "--version".to_string(),
                    params: "".to_string(),
                    docstring: "Output the version info for the program".to_string(),
                },
            ],
        }
    }

    /// A simple method for setting the version info of the program. It can be chained onto an instance of a program and also returns a mutable ref to the program allowing more methods to be chained
    pub fn version(&mut self, vers: &str) -> &mut Program {
        self.version = vers.to_string();
        self
    }

    /// A method for setting the author information of, acts in the same manner as the version method
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

    pub fn parse(&self) {
        // Check if very first arg is special flag, act and exit
        // Check if first arg is known command, act then
        // Check if args are empty
        let raw_args: Vec<String> = std::env::args().collect();
        let args = raw_args[1..].to_vec();

        let cmd_names: Vec<String> = self.cmds.iter().map(|c| c.name.clone()).collect();
        let cmd_aliases: Vec<String> = self.cmds.iter().map(|c| c.alias.clone()).collect();

        match args[0].to_lowercase().as_str() {
            "-h" | "--help" => self.output_help(""),
            "-v" | "--version" => self.output_version_info(),
            val if cmd_names.contains(&val.to_string())
                | cmd_aliases.contains(&val.to_string()) =>
            {
                let matched: Vec<&Cmd> = self
                    .cmds
                    .iter()
                    .filter(|c| c.name.as_str() == val || c.alias.as_str() == val)
                    .collect();
                let cmd = matched[0];
                // Call cmd.parse here and pass the config object to the callback
                let (vals, opts) = cmd.parse(&args[1..].to_vec());
                (cmd.callback)(vals, opts);
            }
            val if val.starts_with("-") => {
                let msg = format!("Unknown option {}", val);
                self.output_help(msg.as_str());
            }
            val => {
                let msg = format!("Unknown command {}", val);
                self.output_help(msg.as_str());
            }
        }
    }

    pub fn output_help(&self, err: &str) {
        println!("\n{}\n", self.about);
        println!("USAGE: ");
        println!("\texe [options] command\n");
        println!("OPTIONS: ");
        for opt in &self.options {
            println!("\t{}, {}", opt.short, opt.long);
            println!("\t{}\n", opt.docstring)
        }
        println!("COMMANDS: ");
        for cmd in &self.cmds {
            let params: Vec<String> = cmd.params.iter().map(|c| c.name.clone()).collect();
            println!("\t({} | {}) [options] {:?}", cmd.name, cmd.alias, params);
            println!("\t {}", cmd.description);
        }

        if !err.is_empty() {
            println!("{}", err)
        }
    }

    pub fn output_version_info(&self) {
        println!("{}", self.version)
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
            version: "0.1.0".to_string(),
            author: "me".to_string(),
            about: "a test".to_string(),
            options: vec![],
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
