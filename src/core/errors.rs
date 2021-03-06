#![allow(unused)]

use std::fmt;

use crate::{Command, Event};

use super::EventConfig;

#[derive(Debug, Clone)]
pub enum CmderError {
    MissingRequiredArgument(Vec<String>), // exit code 5
    OptionMissingArgument(Vec<String>),   // exit code 10
    UnknownCommand(String),               // exit code 15
    UnknownOption(String),                // exit code 20
    UnresolvedArgument(Vec<String>),      // exit code 25
}

// #[derive(Debug, Clone)]
// pub struct CmderErrorr<'err> {
//     pub(crate) kind: Event,
//     pub(crate) message: String,
//     pub(crate) help: &'err str,
//     pub(crate) args: Vec<String>,
//     pub(crate) exit_code: usize,
//     pub(crate) matched_cmd: Option<&'err Command<'err>>,
// }

pub type CmderResult<T, E = CmderError> = Result<T, E>;

// impl<'e> Into<EventConfig<'e>> for CmderErrorr<'e> {
//     fn into(self) -> EventConfig<'e> {
//         EventConfig::new(self.matched_cmd.unwrap())
//             .info(self.help)
//             .exit_code(self.exit_code)
//             .error_str(self.message)
//             .arg_c(self.args.len())
//             .args(self.args)
//     }
// }

impl From<CmderError> for String {
    fn from(err: CmderError) -> Self {
        use CmderError::*;
        match err {
            MissingRequiredArgument(ref val) => {
                let arg_string = get_vector_string(val);
                format!("Missing the following required argument(s): {arg_string}")
            }
            OptionMissingArgument(ref args) => {
                format!(
                    "Missing required argument(s): `{}` for option: `{}`",
                    args[0], args[1]
                )
            }
            UnknownCommand(cmd) => {
                format!("Could not find command: `{cmd}`")
            }
            UnknownOption(opt) => {
                format!("You have passed an unknown option: `{opt}`")
            }
            UnresolvedArgument(ref vals) => {
                let arg_string = get_vector_string(vals);
                format!("Could not resolve the following argument(s): {arg_string}")
            }
        }
    }
}

impl<'a> fmt::Display for CmderError {
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
