use std::collections::HashMap;

use crate::Event;

use super::super::Program;
use super::{resolve_flag, Argument, Cmd};

pub struct Parser<'a> {
    program: &'a Program,
    cmd: Option<&'a Cmd>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &'a Program, cmd: Option<&'a Cmd>) -> Self {
        Self { program, cmd }
    }

    pub fn parse(
        &self,
        parent: &str,
        raw_args: &[String],
    ) -> (HashMap<String, String>, HashMap<String, String>) {
        let mut values = HashMap::new();
        let mut options = HashMap::new();

        let mut flags_and_args = vec![];
        let program = self.program;
        for (idx, arg) in raw_args.iter().enumerate() {
            // let cursor_pstn = idx + 1;

            let list = if parent == "cmd" {
                self.cmd.unwrap().get_cmd_options()
            } else {
                program.get_options()
            };

            if let Some(flg) = resolve_flag(list, arg) {
                if flg.short == "-h" {
                    match parent {
                        "cmd" => {
                            let cmd = self.cmd.unwrap();
                            cmd.output_command_help(program, "");
                            program.emit(Event::OutputCommandHelp, cmd.get_name());
                            std::process::exit(0);
                        }
                        _ => {
                            program.output_help("");
                            program.emit(Event::OutputHelp, program.get_bin_name());
                            std::process::exit(0);
                        }
                    }
                }

                match flg.get_matches(idx, raw_args) {
                    Ok(res) => {
                        if let Some(ans) = res {
                            options.insert(ans.0.clone(), ans.1.clone());

                            flags_and_args.push(arg.clone());
                            flags_and_args.push(ans.1);
                        }
                    }
                    Err(err) => {
                        program.emit(
                            Event::OptionMissingArgument,
                            format!("{} {}", err.0, err.1).as_str(),
                        );

                        let msg =
                            format!("Missing required argument: {} for option: {}", err.0, err.1);

                        match parent {
                            "cmd" => {
                                self.cmd.unwrap().output_command_help(program, &msg);
                            }
                            _ => {
                                program.output_help(&msg);
                            }
                        }
                        std::process::exit(1)
                    }
                };
            }
        }

        // get all values that were not matched as flags or flags' params
        let mut input = vec![];
        for a in raw_args {
            if !flags_and_args.contains(a) {
                input.push(a)
            }
        }

        let params = if parent == "cmd" {
            self.cmd.unwrap().get_cmd_input()
        } else {
            program.get_input()
        };

        let name = if parent == "cmd" {
            self.cmd.unwrap().get_name()
        } else {
            program.get_bin_name()
        };

        // check if any required inputs are missing and act accordingly if so
        let required = Argument::get_required_args(&params);
        let handler = |i: usize| {
            let msg = format!("{}, {}", name, params[i].literal);
            program.emit(Event::MissingArgument, &msg);

            let msg = format!("Missing required argument: {}", params[i].literal);

            match parent {
                "cmd" => {
                    self.cmd.unwrap().output_command_help(program, &msg);
                }
                _ => program.output_help(&msg),
            }
            std::process::exit(1)
        };

        // handle mutiple inputs required
        match input.len() {
            0 => {
                if !required.is_empty() {
                    handler(0);
                }
            }
            val if val < required.len() => handler(val),
            _ => {}
        }

        //TODO: more robust code for checking the input values
        for (i, k) in params.iter().enumerate() {
            let name = &k.name;

            if k.variadic {
                let mut value = String::new();
                for (idx, arg) in input.iter().enumerate() {
                    if idx >= i {
                        value.push_str(arg);
                        value.push(' ')
                    }
                }
                values.insert(name.to_owned(), value.trim().to_string());
            } else if let Some(v) = input.get(i) {
                let val = v.to_owned();
                values.insert(name.to_owned(), val.to_owned());
            }
        }

        (values, options)
    }
}

// /**
//  * ['val1', 'val2', '-n', 'optVal', '-p']
//  * go through each of the values, det if its input or flag
//  * if flag and has input, scan next token
//  *
//  * for arg in raw_args {
//  *  let cursor_pstn = idx
//  *
//  *  if flag and required input, scan next token and save as input ,
//  *  do while all inputs are required
//  *  handle flag optional args
//  *
//  *  if not flag and input not variadic, store input in hs
//  *
//  *  if variadic, scan till end or until you find flag and store in one input
//  * }
//  */
