#![allow(unused)]

use std::fmt;

#[derive(Debug, Clone)]
pub enum CmderError<'err> {
    MissingArgument(Vec<String>),
    OptionMissingArgument(Vec<String>),
    UnknownCommand(&'err str),
    UnknownOption(&'err str),
}

impl<'a> fmt::Display for CmderError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            CmderError::MissingArgument(ref args) => {
                let arg_string = get_vector_string(args);
                f.write_fmt(format_args!(
                    "Missing the following required argument(s): {arg_string}"
                ))
            }
            CmderError::OptionMissingArgument(ref args) => f.write_fmt(format_args!(
                "Missing required argument(s): {} for option: {}",
                args[0], args[1]
            )),
            CmderError::UnknownOption(ref opt) => {
                f.write_fmt(format_args!("You have passed an unknown option: {opt}"))
            }
            CmderError::UnknownCommand(ref cmd) => {
                f.write_fmt(format_args!("Could not find command: {cmd}"))
            }
            _ => f.write_str("An error occurred"),
        }
    }
}

fn get_vector_string(args: &Vec<String>) -> String {
    let mut res = String::new();
    for a in args {
        res.push_str(a.as_str());
        res.push(' ');
    }

    res.trim().to_owned()
}
