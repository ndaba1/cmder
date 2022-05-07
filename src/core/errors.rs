#![allow(unused)]

use std::fmt;

#[derive(Debug, Clone)]
pub enum CmderError<'err> {
    MissingArgument(Vec<String>),       // exit code 1
    OptionMissingArgument(Vec<String>), // exit code 5
    UnknownCommand(&'err str),          // exit code 10
    UnknownOption(&'err str),           // exit code 15
}

impl<'a> Into<String> for CmderError<'a> {
    fn into(self) -> String {
        match self {
            CmderError::MissingArgument(ref val) => {
                let arg_string = get_vector_string(val);
                format!("Missing the following required argument(s): {arg_string}")
            }
            CmderError::OptionMissingArgument(ref args) => {
                format!(
                    "Missing required argument(s): `{}` for option: `{}`",
                    args[0], args[1]
                )
            }
            CmderError::UnknownCommand(cmd) => {
                format!("Could not find command: `{cmd}`")
            }
            CmderError::UnknownOption(opt) => {
                format!("You have passed an unknown option: `{opt}`")
            }
        }
    }
}

impl<'a> fmt::Display for CmderError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let val: String = self.to_owned().into();
        f.write_str(&val)
    }
}

fn get_vector_string(args: &Vec<String>) -> String {
    let mut res = String::new();
    for a in args {
        res.push_str(&format!("`{a}`"));
        res.push(' ');
    }

    res.trim().to_owned()
}
