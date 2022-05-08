#![allow(unused)]

use std::collections::HashMap;

use crate::core::errors::CmderError;
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

pub struct NewParser {}

impl NewParser {
    pub fn parse<'a>(
        cmd: &'a Command<'a>,
        args: Vec<String>,
        root_cfg: Option<ParserMatches<'a>>,
    ) -> Result<ParserMatches<'a>, CmderError> {
        if args.is_empty() {
            let arg_name = if cmd.get_arguments().is_empty() {
                (&cmd.get_arguments()[0]).clone().literal
            } else {
                format!("<SUB-COMMAND>")
            };

            return Err(CmderError::MissingArgument(vec![arg_name]));
        }

        let mut marked_args = vec![];
        let mut config = if let Some(cfg) = root_cfg {
            marked_args = cfg.marked_args.clone();
            cfg
        } else {
            ParserMatches::new(args.len(), &cmd)
        };

        let marked_args_iter = marked_args.iter().map(|f| f.0.clone()).collect::<Vec<_>>();
        for a in &args {
            if !marked_args_iter.contains(a) {
                marked_args.push((a.to_owned(), 0))
            }
        }

        for (cursor_index, arg) in args.iter().enumerate() {
            // Check whether its a flag
            if let Some(flag) = resolve_new_flag(cmd.get_flags(), arg.clone()) {
                let count = config.get_flag_count(flag.long_version);

                let flag_cfg = FlagsMatches {
                    appearance_count: (count + 1) as usize,
                    cursor_index: cursor_index + config.cursor_offset,
                    flag,
                };

                if !config.contains_flag(flag_cfg.flag.long_version) {
                    config.flag_matches.push(flag_cfg);
                }

                // Mark flag as used
                marked_args.get_mut(cursor_index).unwrap().1 = 1;
                config.marked_args.push((arg.clone(), 1));
            } else if let Some(opt) = resolve_new_option(cmd.get_options(), arg.clone()) {
                let max_args_len = opt.arguments.len();
                let cleaned_name = opt.long_version.replace("--", "").replace('-', "_");
                let count = config.get_option_count(opt.long_version) as usize;

                for (opt_arg_idx, opt_arg_val) in opt.arguments.iter().enumerate() {
                    let opt_arg_val = opt_arg_val.clone();
                    let step = opt_arg_idx + 1;
                    let mut raw_arg_value = String::new();

                    if opt_arg_val.variadic {
                        for (i, a) in args.iter().enumerate() {
                            if i >= cursor_index && !a.starts_with('-') {
                                raw_arg_value.push_str(&a);
                                raw_arg_value.push(' ')
                            }
                        }
                    } else if opt_arg_idx <= max_args_len {
                        match args.get(cursor_index + step) {
                            Some(val) => {
                                // Mark value as visited
                                marked_args.get_mut(cursor_index + step).unwrap().1 = 1;
                                config.marked_args.push((val.clone(), 1));

                                raw_arg_value.push_str(&format!("{}", val.as_str()));
                            }
                            None => {
                                if opt_arg_val.required {
                                    let values =
                                        vec![opt_arg_val.literal, args[cursor_index].to_string()];
                                    return Err(CmderError::OptionMissingArgument(values));
                                }
                            }
                        }
                    }

                    let arg_cfg = ArgsMatches {
                        cursor_index: (cursor_index + 1) + config.cursor_offset,
                        instance_of: opt_arg_val.literal.clone(),
                        raw_value: raw_arg_value,
                    };

                    if config.contains_option(opt.clone().long_version) {
                        for opt_match in config.option_matches.iter_mut() {
                            if opt_match.option.long_version == opt.clone().long_version {
                                opt_match.args.push(arg_cfg.clone());
                                opt_match.appearance_count += 1;
                            }
                        }
                    } else {
                        let opt_cfg = OptionsMatches {
                            appearance_count: (count + 1) as usize,
                            cursor_index: cursor_index + config.cursor_offset,
                            option: opt.clone(),
                            args: vec![arg_cfg],
                        };

                        config.option_matches.push(opt_cfg);
                    }

                    // Mark option as used
                    marked_args.get_mut(cursor_index).unwrap().1 = 1;
                    config.marked_args.push((arg.clone(), 1));
                } // end for
            } else if let Some(sub_cmd) = cmd.find_subcommand(&arg.clone()) {
                let arguments = sub_cmd.get_arguments();
                let max_args_len = arguments.len();

                for (cmd_arg_idx, cmd_arg_val) in arguments.iter().enumerate() {
                    let step = cmd_arg_idx + 1;
                    let mut raw_value = String::new();

                    if cmd_arg_val.variadic {
                        for (i, a) in args.iter().enumerate() {
                            if i >= cursor_index && !a.starts_with('-') {
                                raw_value.push_str(a.as_str());
                                raw_value.push(' ')
                            }
                        }
                    } else if cmd_arg_idx <= max_args_len {
                        match args.get(cursor_index + step) {
                            Some(val) => {
                                if val.starts_with('-') {
                                    // let mut vals = vec![cmd_arg_val.literal.clone()];
                                    // return Err(CmderError::MissingArgument(vals));
                                    continue;
                                } else {
                                    // Mark value as visited
                                    marked_args.get_mut(cursor_index + step).unwrap().1 = 1;
                                    config.marked_args.push((val.clone(), 1));

                                    raw_value.push_str(val);
                                }
                            }
                            None => {
                                if cmd_arg_val.required {
                                    let vals = vec![cmd_arg_val.literal.clone()];
                                    return Err(CmderError::MissingArgument(vals));
                                }
                            }
                        }
                    }

                    let arg_cfg = ArgsMatches {
                        cursor_index: (cursor_index + 1) + config.cursor_offset,
                        instance_of: cmd_arg_val.literal.clone(),
                        raw_value,
                    };

                    config.arg_matches.push(arg_cfg);
                }

                config.matched_subcmd = Some(sub_cmd);
                config.cursor_offset += 1;

                // Mark subcommand as used
                marked_args.get_mut(cursor_index).unwrap().1 = 1;
                config.marked_args.push((arg.clone(), 1));

                // Parse subcommand
                return NewParser::parse(sub_cmd, args[cursor_index + 1..].to_vec(), Some(config));
            } else if arg == "--" {
                config.marked_args.push((arg.clone(), 1));
                for (i, a) in args[cursor_index + 1..].to_vec().iter().enumerate() {
                    // Mark argument as used
                    marked_args.get_mut(cursor_index + (i + 1)).unwrap().1 = 1;
                    config.marked_args.push((a.clone(), 1));
                    config.ignored_options.push(a.to_string());
                }
                break;
            } else if marked_args.get(cursor_index).unwrap().1 == 0 {
                // Check whether arg is marked as used

                if arg.starts_with('-') && *arg != "--".to_string() {
                    return Err(CmderError::UnknownOption(arg.clone()));
                }

                return Err(CmderError::UnknownCommand(arg.clone()));
            }
        }

        Ok(config)
    }

    // pub fn parse<'a>(
    //     root: &'a Command<'a>,
    //     raw_args: &'a [&'a str],
    //     root_cfg: Option<ParserMatches<'a>>,
    // ) -> Result<ParserMatches<'a>, CmderError> {
    //     // Check if args empty
    //     if raw_args.is_empty() {
    //         // handle empty args
    //         // return Ok(ParserMatches::new(0, root));
    //     }

    //     let mut config = if let Some(cfg) = root_cfg {
    //         cfg
    //     } else {
    //         ParserMatches::new(raw_args.len(), root)
    //     };

    //     // // ["image", "ls", "-p", "80", "--", "-xc", "-pv"]
    //     // for (cursor_idx, arg) in raw_args.iter().enumerate() {
    //     //     if let Some(flag) = resolve_new_flag(root.get_flags(), *arg.to_owned()) {
    //     //         // handle flags input
    //     //         let count = config.get_flag_count(flag.long_version);

    //     //         let cfg = FlagsMatches {
    //     //             appearance_count: (count + 1) as usize,
    //     //             cursor_index: cursor_idx + config.cursor_offset,
    //     //             flag,
    //     //         };

    //     //         if !config.contains_flag(cfg.flag.long_version) {
    //     //             config.flag_matches.push(cfg);
    //     //         }
    //     //     } else if let Some(opt) = resolve_new_option(root.get_options(), arg) {
    //     //         //handle opts input
    //     //         let max_args_len = opt.arguments.len();
    //     //         let cleaned_name = opt.long_version.replace("--", "").replace('-', "_");
    //     //         let count = config.get_option_count(opt.long_version) as usize;

    //     //         let mut cfg = OptionsMatches {
    //     //             cursor_index: cursor_idx + config.cursor_offset,
    //     //             appearance_count: count + 1,
    //     //             option: opt.clone(),
    //     //             args: vec![],
    //     //         };

    //     //         for (arg_idx, arg) in opt.arguments.into_iter().enumerate() {
    //     //             let step = arg_idx + 1;
    //     //             let mut raw_value = String::new();

    //     //             if arg.variadic {
    //     //                 for (index, a) in raw_args.iter().enumerate() {
    //     //                     if index >= cursor_idx && !a.starts_with('-') {
    //     //                         raw_value.push_str(a);
    //     //                         raw_value.push(' ')
    //     //                     }
    //     //                 }
    //     //             } else if arg_idx <= max_args_len {
    //     //                 // try to any args input values
    //     //                 match raw_args.get(cursor_idx + step) {
    //     //                     Some(v) => {
    //     //                         raw_value.push_str(v);
    //     //                     }
    //     //                     None => {
    //     //                         if arg.required {
    //     //                             let mut vals =
    //     //                                 vec![arg.literal, raw_args[cursor_idx].to_string()];

    //     //                             return Err(CmderError::OptionMissingArgument(vals));
    //     //                         }
    //     //                     }
    //     //                 }
    //     //             }

    //     //             let temp_cfg = ArgsMatches {
    //     //                 cursor_index: (cursor_idx + 1) + config.cursor_offset,
    //     //                 instance_of: arg.literal.clone(),
    //     //                 raw_value,
    //     //             };

    //     //             cfg.args.push(temp_cfg);
    //     //         }

    //     //         config.option_matches.push(cfg);
    //     //     } else if let Some(sub_cmd) = root.find_subcommand(arg) {
    //     //         // it is a subcommand/command
    //     //         let arguments = sub_cmd.get_arguments();
    //     //         let max_args_len = arguments.len();

    //     //         for (arg_idx, arg) in sub_cmd.get_arguments().iter().enumerate() {
    //     //             let step = arg_idx + 1;
    //     //             let mut raw_value = String::new();

    //     //             if arg.variadic {
    //     //                 for (index, a) in raw_args.iter().enumerate() {
    //     //                     if index >= cursor_idx && !a.starts_with('-') {
    //     //                         raw_value.push_str(a);
    //     //                         raw_value.push(' ')
    //     //                     }
    //     //                 }
    //     //             } else if arg_idx <= max_args_len {
    //     //                 // try to any args input values
    //     //                 match raw_args.get(cursor_idx + step) {
    //     //                     Some(v) => {
    //     //                         if v.starts_with('-') {
    //     //                             let mut vals = vec![arg.literal.clone()];
    //     //                             return Err(CmderError::MissingArgument(vals));
    //     //                         } else {
    //     //                             raw_value.push_str(v);
    //     //                         }
    //     //                     }
    //     //                     None => {
    //     //                         if arg.required {
    //     //                             let mut vals = vec![arg.literal.clone()];
    //     //                             return Err(CmderError::MissingArgument(vals));
    //     //                         }
    //     //                     }
    //     //                 }
    //     //             }

    //     //             let temp_cfg = ArgsMatches {
    //     //                 cursor_index: (cursor_idx + 1) + config.cursor_offset,
    //     //                 instance_of: arg.literal.clone(),
    //     //                 raw_value,
    //     //             };

    //     //             config.arg_matches.push(temp_cfg);
    //     //         }

    //     //         config.cursor_offset += 1;
    //     //         config.matched_subcmd = Some(sub_cmd);

    //     //         return NewParser::parse(sub_cmd, &raw_args[1..], Some(config));
    //     //     } else if *arg == "--" {
    //     //         // register positional_options and break loop
    //     //         config.positional_options = &raw_args[(cursor_idx + 1)..];
    //     //         break;
    //     //     } else {
    //     //         // it is either an argument or unknown
    //     //         let max_len = root.get_arguments().len();

    //     //         if max_len > 0 && cursor_idx <= max_len && !arg.starts_with('-') {
    //     //             continue;
    //     //         }

    //     //         if arg.starts_with('-') && *arg != "--" {
    //     //             return Err(CmderError::UnknownOption(arg));
    //     //         }

    //     //         return Err(CmderError::UnknownCommand(arg));
    //     //     }
    //     // }

    //     Ok(config)
    // }
}
