#![allow(unused)]

use std::collections::HashMap;

use crate::core::errors::{CmderError, CmderResult};
use crate::core::new_program::Command;
use crate::core::EventConfig;
use crate::Event;

use super::super::Program;
use super::flags::{resolve_new_flag, resolve_new_option, NewFlag, NewOption};
use super::matches::{ArgsMatches, CommandMatches, FlagsMatches, OptionsMatches, ParserMatches};
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
        if raw_args.is_empty() && parent == "cmd" {
            let cmd = self.cmd.unwrap();

            if !cmd.get_subcommands().is_empty() {
                cmd.output_command_help(self.program, "")
            }
        }

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
                } else if flg.short == "-v" && parent == "program" {
                    program.emit(Event::OutputVersion, program.get_version());
                    program.output_version_info();
                    std::process::exit(0);
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
            } else if arg.starts_with('-') {
                program.emit(Event::UnknownOption, arg);
                let msg = format!("Unknown option \"{}\"", arg);
                program.output_help(&msg);
                std::process::exit(1);
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
        let required = Argument::get_required_args(params);
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

pub struct NewParser<'a> {
    cmd: &'a Command<'a>,
    cursor_index: usize,
    marked_args: Vec<(String, bool)>,
    valid_arg_found: bool,
    allow_trailing_values: bool,
    parser_cfg: ParserMatches<'a>,
}

impl<'p> NewParser<'p> {
    pub fn new(cmd: &'p Command<'p>) -> Self {
        Self {
            cmd,
            cursor_index: 0,
            marked_args: vec![],
            valid_arg_found: false,
            allow_trailing_values: false,
            parser_cfg: ParserMatches::new(0, cmd),
        }
    }

    pub fn parse(&mut self, args: Vec<String>) -> CmderResult<ParserMatches<'p>> {
        if args.is_empty() {
            // handle empty args
        }

        // Mark all the args as untouched yet
        for a in &args {
            self.marked_args.push((a.clone(), false));
        }

        self.parser_cfg.arg_count = args.len();

        for (cursor_index, arg) in args.iter().enumerate() {
            let cmd = self.cmd;
            self.cursor_index = cursor_index;

            if arg.starts_with('-') {
                // It is either a flag, an option, or '--', or unknown option/flag
                if let Some(flag) = resolve_new_flag(cmd.get_flags(), arg.clone()) {
                    // parse flag
                    if !self.allow_trailing_values {
                        self.marked_args[cursor_index].1 = true;
                        self.parse_flag(flag)
                    }
                } else if let Some(opt) = resolve_new_option(cmd.get_options(), arg.clone()) {
                    // parse option
                    self.marked_args[cursor_index].1 = true;
                    // Parse any args following option
                    self.parse_option(opt, args[(cursor_index + 1)..].to_vec())?
                } else if arg.contains("=") && !self.allow_trailing_values {
                    println!("reached");
                    self.marked_args[cursor_index].1 = true;
                    // Split the arg into key and value
                    let parts = arg.split('=').collect::<Vec<_>>();

                    if let Some(opt) = resolve_new_option(cmd.get_options(), parts[0].into()) {
                        // parse option using parts[1]
                        let mut temp_args: Vec<String> = vec![];
                        temp_args.push(parts[1].into());
                        temp_args.extend_from_slice(&args[(cursor_index + 1)..]);

                        self.parse_option(opt, temp_args)?
                    }
                } else if arg == "--" {
                    self.allow_trailing_values = true;
                    self.marked_args[cursor_index].1 = true;
                    // parse positional args
                    for (i, a) in args[cursor_index..].to_vec().iter().enumerate() {
                        self.marked_args[(cursor_index + i)].1 = true;
                        // TODO: Refactor pos_args field into vec of arg_matches
                        self.parser_cfg.positional_args.push(a.to_owned());
                    }
                } else if !self.is_marked(cursor_index) {
                    return Err(CmderError::UnknownOption(arg.clone()));
                }
            } else if let Some(sub_cmd) = cmd.find_subcommand(arg) {
                self.marked_args[cursor_index].1 = true;
                // check if command pstn is valid
                if self.valid_arg_found {
                    // return invalid ctx error
                } else {
                    // parse sub_cmd
                    self.parse_cmd(sub_cmd, args[(cursor_index + 1)..].to_vec())?
                }
            } else if !self.is_marked(cursor_index) {
                // check if any arguments were expected
                let arg_cfg = self.parse_args(cmd.get_arguments(), args.clone())?;

                if !arg_cfg.is_empty() {
                    self.valid_arg_found = true;
                    self.parser_cfg.arg_matches.extend_from_slice(&arg_cfg[..]);
                } else {
                    match cursor_index {
                        0 => {
                            return Err(CmderError::UnknownCommand(arg.clone()));
                        }
                        _ => {
                            // return invalid ctx or unexpected arg error
                        }
                    }
                }
            }
        }

        Ok(self.parser_cfg.clone())
    }

    fn is_marked(&self, idx: usize) -> bool {
        self.marked_args.get(idx).unwrap().1 == true
    }

    // Returns option matches
    fn parse_option(&mut self, opt: NewOption<'p>, args: Vec<String>) -> CmderResult<()> {
        let count = self.parser_cfg.get_option_count(opt.short_version);
        let args = self.parse_args(&opt.arguments, args)?;
        let config = &mut self.parser_cfg;

        if config.contains_option(&opt.long_version) {
            for opt_cfg in config.option_matches.iter_mut() {
                if opt_cfg.option.long_version == opt.long_version {
                    opt_cfg.args.extend_from_slice(&args[..]);
                    opt_cfg.appearance_count += 1;
                }
            }
        } else {
            let opt_cfg = OptionsMatches {
                appearance_count: (count + 1) as usize,
                cursor_index: self.cursor_index,
                option: opt,
                args,
            };

            config.option_matches.push(opt_cfg);
        }

        Ok(())
    }

    // Returns flag matches
    fn parse_flag(&mut self, flag: NewFlag<'p>) {
        // TODO: Check if context is valid for flag position
        let cfg = FlagsMatches {
            appearance_count: 1,
            cursor_index: self.cursor_index,
            flag,
        };

        if !self.parser_cfg.contains_flag(cfg.flag.short_version) {
            self.parser_cfg.flag_matches.push(cfg);
        }
    }

    // Parse subcmds
    fn parse_cmd(&mut self, cmd: &'p Command<'p>, args: Vec<String>) -> CmderResult<()> {
        self.marked_args[self.cursor_index].1 = true;
        self.parser_cfg.matched_cmd = Some(cmd);
        self.cmd = cmd;

        for (i, a) in args.iter().enumerate() {
            if let Some(sc) = cmd.find_subcommand(a) {
                return self.parse_cmd(sc, args[(i + 1)..].to_vec());
            }
        }

        let args = self.parse_args(cmd.get_arguments(), args)?;

        if !args.is_empty() {
            self.valid_arg_found = true;
        }

        self.parser_cfg.arg_matches.extend_from_slice(&args[..]);

        Ok(())
    }

    // Parse arg
    fn parse_args(
        &mut self,
        arg_list: &Vec<Argument>,
        raw_args: Vec<String>,
    ) -> CmderResult<Vec<ArgsMatches>> {
        let cursor_index = self.cursor_index;
        let max_args_len = arg_list.len();
        let mut arg_vec = vec![];

        for (arg_idx, arg_val) in arg_list.iter().enumerate() {
            let step = arg_idx + 1;
            let mut raw_value = String::new();

            // check if arg is variadic
            if arg_val.variadic {
                for (i, val) in raw_args.iter().enumerate() {
                    let full_idx = cursor_index + i + 1;
                    if !val.starts_with('-') && !self.is_marked(full_idx) {
                        self.marked_args[full_idx].1 = true;

                        raw_value.push_str(val);
                        raw_value.push(' ');
                    }
                }
            } else if arg_idx <= max_args_len {
                // valid to collect arguments
                match raw_args.iter().enumerate().next() {
                    Some((idx, val)) => {
                        let full_index = cursor_index + idx + 1;
                        if !self.is_marked(full_index) && !val.starts_with('-') {
                            self.marked_args[full_index].1 = true;
                            raw_value.push_str(val)
                        } else if arg_val.required {
                            // return err: expected one value found another
                            let vals = vec![arg_val.literal.clone()];
                            return Err(CmderError::MissingArgument(vals));
                        }
                    }
                    None => {
                        if arg_val.required {
                            let vals = vec![arg_val.literal.clone()];
                            return Err(CmderError::MissingArgument(vals));
                        }
                    }
                }
            }

            let arg_cfg = ArgsMatches {
                cursor_index: (cursor_index + step),
                instance_of: arg_val.literal.clone(),
                raw_value,
            };

            arg_vec.push(arg_cfg);
        }

        Ok(arg_vec)
    }
}
