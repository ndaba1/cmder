#![allow(unused)]

use std::collections::HashMap;

use crate::core::Command;
use crate::core::EventConfig;
use crate::core::{CmderError, CmderResult};
use crate::Event;

use super::flags::{resolve_new_flag, resolve_new_option, CmderFlag, CmderOption};
use super::matches::{ArgsMatches, CommandMatches, FlagsMatches, OptionsMatches, ParserMatches};
use super::Argument;

pub struct Parser<'a> {
    cmd: &'a Command<'a>,
    cursor_index: usize,
    marked_args: Vec<(String, bool)>,
    valid_arg_found: bool,
    allow_trailing_values: bool,
    parser_cfg: ParserMatches<'a>,
}

impl<'p> Parser<'p> {
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
        self.parser_cfg.matched_cmd = Some(self.cmd);

        for (cursor_index, arg) in args.iter().enumerate() {
            let cmd = self.cmd;
            self.cursor_index = cursor_index;

            if arg.is_empty() {
                // ignore empty args
                continue;
            } else if arg.starts_with('-') {
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
                    self.cursor_index += 1;
                    // Parse any args following option
                    self.parse_option(opt, args[(cursor_index + 1)..].to_vec())?
                } else if arg.contains('=') && !self.allow_trailing_values {
                    // Split the arg into key and value
                    let parts = arg.split('=').collect::<Vec<_>>();

                    if let Some(opt) = resolve_new_option(cmd.get_options(), parts[0].into()) {
                        // parse option using parts[1]
                        let mut temp_args: Vec<String> = vec![parts[1].into()];
                        temp_args.extend_from_slice(&args[(cursor_index + 1)..]);

                        self.parse_option(opt, temp_args)?
                    }
                } else if arg == "--" {
                    self.allow_trailing_values = true;
                    self.marked_args[cursor_index].1 = true;
                    // parse positional args
                    for (i, a) in args[(cursor_index + 1)..].to_vec().iter().enumerate() {
                        self.marked_args[(cursor_index + i) + 1].1 = true;
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
                    // TODO: return invalid ctx error
                    return Err(CmderError::UnresolvedArgument(vec![arg.clone()]));
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
                    self.parser_cfg.matched_cmd = Some(cmd);
                } else if cursor_index == 0 {
                    // if no args were expected and the first arg is not empty, then it was probably a command
                    return Err(CmderError::UnknownCommand(arg.clone()));
                } else {
                    // Otherwise, the argument is not valid and could not be resolved
                    // TODO: return invalid ctx error
                    return Err(CmderError::UnresolvedArgument(vec![arg.clone()]));
                }
            }
        }

        Ok(self.parser_cfg.clone())
    }

    fn is_marked(&self, idx: usize) -> bool {
        self.marked_args.get(idx).unwrap().1
    }

    // Returns option matches
    fn parse_option(&mut self, opt: CmderOption<'p>, args: Vec<String>) -> CmderResult<()> {
        let count = self.parser_cfg.get_option_count(opt.short);
        let args = self.parse_args(&opt.arguments, args)?;
        let config = &mut self.parser_cfg;

        if config.contains_option(opt.long) {
            for opt_cfg in config.option_matches.iter_mut() {
                if opt_cfg.option.long == opt.long {
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
    fn parse_flag(&mut self, flag: CmderFlag<'p>) {
        // TODO: Check if context is valid for flag position
        let cfg = FlagsMatches {
            appearance_count: 1,
            cursor_index: self.cursor_index,
            flag,
        };

        if !self.parser_cfg.contains_flag(cfg.flag.short) {
            self.parser_cfg.flag_matches.push(cfg);
        }
    }

    // Parse subcmds
    fn parse_cmd(&mut self, cmd: &'p Command<'p>, args: Vec<String>) -> CmderResult<()> {
        self.cursor_index += 1;
        self.parser_cfg.matched_cmd = Some(cmd);
        self.cmd = cmd;

        for (i, a) in args.iter().enumerate() {
            if let Some(sc) = cmd.find_subcommand(a) {
                self.marked_args[(self.cursor_index + i)].1 = true;
                return self.parse_cmd(sc, args[(i + 1)..].to_vec());
            }
        }
        let arg_cfg = self.parse_args(cmd.get_arguments(), args)?;

        if !arg_cfg.is_empty() {
            self.valid_arg_found = true;
        }

        self.parser_cfg.arg_matches.extend_from_slice(&arg_cfg[..]);

        Ok(())
    }

    // Parse arg
    fn parse_args(
        &mut self,
        arg_list: &[Argument],
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
                    let full_idx = cursor_index + i;
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
                        let full_index = cursor_index + idx;
                        if !self.is_marked(full_index) && !val.starts_with('-') {
                            self.marked_args[full_index].1 = true;
                            raw_value.push_str(val)
                        } else if val == "-h" || val == "--help" {
                            break;
                        } else if arg_val.required {
                            // return err: expected one value found another
                            let vals = vec![arg_val.literal.clone()];
                            return Err(CmderError::MissingRequiredArgument(vals));
                        } else {
                            continue;
                        }
                    }
                    None => {
                        if arg_val.required {
                            let vals = vec![arg_val.literal.clone()];
                            return Err(CmderError::MissingRequiredArgument(vals));
                        }
                    }
                }
            }

            let arg_cfg = ArgsMatches {
                cursor_index: (cursor_index + step),
                instance_of: arg_val.literal.clone(),
                raw_value: raw_value.trim().to_string(),
            };

            arg_vec.push(arg_cfg);
        }

        Ok(arg_vec)
    }

    // fn resolve_options(&mut self, args: Vec<String>) {}
}
