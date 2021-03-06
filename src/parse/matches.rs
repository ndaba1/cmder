#![allow(dead_code)]

use crate::core::Command;

use super::{CmderFlag, CmderOption};

#[derive(Debug, Clone)]
pub struct ParserMatches<'pm> {
    pub(crate) arg_count: usize,
    pub(crate) root_cmd: &'pm Command<'pm>,
    pub(crate) matched_cmd: Option<&'pm Command<'pm>>,
    pub(crate) flag_matches: Vec<FlagsMatches>,
    pub(crate) option_matches: Vec<OptionsMatches>,
    pub(crate) arg_matches: Vec<ArgsMatches>,
    pub(crate) positional_args: Vec<String>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct FlagsMatches {
    pub(crate) cursor_index: usize,
    pub(crate) flag: CmderFlag,
    pub(crate) appearance_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct OptionsMatches {
    pub(crate) cursor_index: usize,
    pub(crate) option: CmderOption,
    pub(crate) args: Vec<ArgsMatches>,
    pub(crate) appearance_count: usize,
}

impl<'o> OptionsMatches {
    pub(crate) fn new() -> Self {
        Self {
            appearance_count: 0,
            args: vec![],
            cursor_index: 0,
            option: CmderOption::default(),
        }
    }

    pub(crate) fn contains_option(&self, option: &str) -> bool {
        self.option.long == option || self.option.short == option
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CommandMatches {
    pub(crate) cursor_index: usize,
    pub(crate) command: Command<'static>,
    pub(crate) args: ArgsMatches,
    pub(crate) flags: Vec<FlagsMatches>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArgsMatches {
    pub(crate) cursor_index: usize,
    pub(crate) raw_value: String,
    pub(crate) instance_of: String,
}

impl<'args> ArgsMatches {
    pub(crate) fn new() -> Self {
        Self {
            cursor_index: 0,
            raw_value: String::new(),
            instance_of: String::new(),
        }
    }
}

impl<'a> ParserMatches<'a> {
    pub(crate) fn new(count: usize, root_cmd: &'a Command<'a>) -> Self {
        Self {
            arg_count: count,
            flag_matches: vec![],
            root_cmd,
            matched_cmd: None,
            arg_matches: vec![],
            option_matches: vec![],
            positional_args: vec![],
        }
    }

    pub fn get_program(&self) -> &'a Command<'a> {
        self.root_cmd
    }

    pub fn get_matched_cmd(&self) -> Option<&'a Command<'a>> {
        self.matched_cmd
    }

    pub fn get_raw_args(&self) -> Vec<String> {
        let mut args = vec![];

        for arg in self.arg_matches.iter() {
            args.push(arg.raw_value.clone());
        }

        args
    }

    pub fn get_raw_args_count(&self) -> usize {
        self.arg_count
    }

    pub fn get_arg(&self, val: &str) -> Option<String> {
        self.arg_matches
            .iter()
            .find(|arg| arg.instance_of == val)
            .map(|a| a.raw_value.clone())
    }

    pub fn get_positional_args(&self) -> Vec<String> {
        self.positional_args.clone()
    }

    pub fn get_option_arg(&self, val: &str) -> Option<String> {
        let mut arg = None;
        self.option_matches.iter().for_each(|o| {
            o.args.iter().for_each(|a| {
                if (a.instance_of).as_str() == val {
                    arg = Some(a.raw_value.to_owned());
                }
            })
        });
        arg
    }

    pub fn get_instances_of(&self, val: &str) -> Vec<&str> {
        let mut instances = vec![];
        for opt_cfg in &self.option_matches {
            for arg_cfg in &opt_cfg.args {
                if (arg_cfg.instance_of).as_str() == val {
                    instances.push((arg_cfg.raw_value).as_str())
                }
            }
        }

        instances
    }

    pub fn get_flag(&self, val: &str) -> Option<CmderFlag> {
        self.flag_matches
            .iter()
            .find(|f| {
                let flag = &f.flag;
                flag.short == val || flag.long == val
            })
            .map(|fm| fm.flag.clone())
    }

    pub fn get_option(&self, val: &str) -> Option<CmderOption> {
        self.option_matches
            .iter()
            .find(|opc| {
                let option = &opc.option;
                option.long == val || option.short == val
            })
            .map(|opm| opm.option.clone())
    }

    pub fn contains_flag(&self, val: &str) -> bool {
        self.flag_matches.iter().any(|f| {
            let flag = &f.flag;
            flag.short == val || flag.long == val
        })
    }

    pub fn contains_option(&self, val: &str) -> bool {
        self.option_matches.iter().any(|o| {
            let op = &o.option;
            op.short == val || op.long == val
        })
    }

    pub fn get_flag_count(&self, val: &str) -> i32 {
        let mut count = 0;

        for fc in &self.flag_matches {
            let flag = &fc.flag;

            if flag.short == val || flag.long == val {
                count += 1;
            }
        }

        count
    }

    pub fn get_option_count(&self, val: &str) -> i32 {
        let mut count = 0;

        for fc in &self.option_matches {
            let flag = &fc.option;

            if flag.short == val || flag.long == val {
                count += 1;
            }
        }

        count
    }
}
