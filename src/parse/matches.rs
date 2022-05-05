#![allow(dead_code)]

use crate::core::new_program::Command;

use super::flags::{NewFlag, NewOption};

#[derive(Debug, Clone)]
pub struct ParserMatches<'pm> {
    pub(crate) arg_count: usize,
    pub(crate) cursor_offset: usize,
    pub(crate) root_cmd: &'pm Command<'pm>,
    pub(crate) matched_subcmd: Option<CommandMatches<'pm>>,
    pub(crate) flag_matches: Vec<FlagsMatches<'pm>>,
    pub(crate) option_matches: Vec<OptionsMatches<'pm>>,
    pub(crate) arg_matches: Vec<ArgsMatches>,
    pub(crate) positional_options: &'pm [&'pm str],
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct FlagsMatches<'a> {
    pub(crate) cursor_index: usize,
    pub(crate) flag: NewFlag<'a>,
    pub(crate) appearance_count: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct OptionsMatches<'o> {
    pub(crate) cursor_index: usize,
    pub(crate) option: NewOption<'o>,
    pub(crate) args: Vec<ArgsMatches>,
    pub(crate) appearance_count: usize,
}

impl<'o> OptionsMatches<'o> {
    pub(crate) fn new() -> Self {
        Self {
            appearance_count: 0,
            args: vec![],
            cursor_index: 0,
            option: NewOption::default(),
        }
    }
}

impl<'d> Default for OptionsMatches<'d> {
    fn default() -> Self {
        Self {
            appearance_count: 0,
            args: vec![],
            cursor_index: 0,
            option: NewOption::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CommandMatches<'b> {
    pub(crate) cursor_index: usize,
    pub(crate) command: Command<'static>,
    pub(crate) args: ArgsMatches,
    pub(crate) flags: Vec<FlagsMatches<'b>>,
}

#[derive(Debug, Clone)]
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
    pub(crate) fn new(count: usize) -> Self {
        Self {
            arg_count: count,
            flags: vec![],
            matched_subcmd: None,
            args: ArgsMatches::new(),
            options: vec![OptionsMatches::default()],
            positional_options: &[],
        }
    }

    pub fn get_arg_count(&self) -> usize {
        self.arg_count
    }

    pub fn contains_flag(&self, val: &str) -> bool {
        match self.flags.iter().find(|f| {
            let flag = &f.flag;
            flag.short_version == val || flag.long_version == val
        }) {
            Some(_f) => true,
            _ => false,
        }
    }

    pub fn contains_option(&self, val: &str) -> bool {
        match self.options.iter().find(|o| {
            let op = &o.option;
            op.short_version == val || op.long_version == val
        }) {
            Some(_o) => true,
            _ => false,
        }
    }

    pub fn get_flag_count(&self, val: &str) -> i32 {
        let mut count = 0;

        for fc in &self.flags {
            let flag = &fc.flag;

            if flag.short_version == val || flag.long_version == val {
                count += 1;
            }
        }

        count
    }

    pub fn get_option_count(&self, val: &str) -> i32 {
        let mut count = 0;

        for fc in &self.options {
            let flag = &fc.option;

            if flag.short_version == val || flag.long_version == val {
                count += 1;
            }
        }

        count
    }

    pub(crate) fn get_option_config(&self, val: &str) -> Option<OptionsMatches> {
        let mut cfg = None;

        for opc in &self.options {
            let op = &opc.option;

            if op.short_version == val || op.long_version == val {
                cfg = Some(opc.clone());
            }
        }

        cfg
    }
}
