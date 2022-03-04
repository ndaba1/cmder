use super::super::{Event, Program};
use super::args::Argument;
use super::Cmd;

#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    /// A short version of the switch/flag, usually begins with a single hyphen, such as -h
    pub short: String,

    /// The full/long version of the switch, usually begins with double hyphens, ie. --help
    pub long: String,

    /// Any parameters that the switch accepts, or requires
    pub params: Vec<Argument>,

    /// A description of the flag and the inputs its accepts
    pub docstring: String,
}

impl Flag {
    pub fn new(body: &str, desc: &str) -> Self {
        let chunks: Vec<_> = body.split(' ').collect();
        let chunks: Vec<_> = chunks.iter().map(|c| c.trim().to_string()).collect();

        // If the length is more than two it means that params have been passed
        let params = if chunks.len() > 2 {
            chunks[2..]
                .to_vec()
                .iter()
                .map(|v| Argument::new(v))
                .collect()
        } else {
            vec![]
        };

        Self {
            short: chunks[0].clone(),
            long: chunks[1].clone(),
            params,
            docstring: desc.trim().to_string(),
        }
    }

    pub fn get_matches(
        &self,
        cmd: &Cmd,
        program: &Program,
        idx: usize,
        raw_args: &[String],
    ) -> Option<(String, String)> {
        // assuming raw_args look something like exe test -a -p 1 -x
        let max_len = self.params.len();
        let cleaned = self.long.replace("--", "").replace('-', "_");
        let mut result = Some((cleaned, "true".to_string()));

        for (i, val) in self.params.iter().enumerate() {
            let step = i + 1;
            if i <= max_len {
                // try to find the
                match raw_args.get(idx + step) {
                    Some(v) => {
                        result = if val.required {
                            Some((val.name.clone(), v.clone()))
                        } else {
                            // do some custom logic to see if value should be added
                            None
                        };
                    }
                    None => {
                        if val.required {
                            // emit missing required argument
                            program.emit(
                                Event::OptionMissingArgument,
                                format!("{} {}", val.literal, raw_args[idx]).as_str(),
                            );

                            let msg = format!(
                                "Missing required argument: {} for option: {}",
                                val.literal, raw_args[idx]
                            );
                            cmd.output_command_help(program, &msg);
                            std::process::exit(1)
                        }
                    }
                }
            }
        }

        result
    }
}

pub fn resolve_flag(list: &[Flag], val: &str) -> Option<Flag> {
    let mut flag = None;

    for f in list {
        if f.short == val || f.long == val {
            flag = Some(f.clone());
        }
    }
    flag
}
