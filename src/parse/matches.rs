#![allow(dead_code)]

use crate::core::new_program::Command;

use super::flags::{NewFlag, NewOption};

pub struct ParserMatches<'pm> {
    pub(crate) arg_count: usize,
    pub(crate) matched_subcmd: Option<CommandConfig<'pm>>,
    pub(crate) flags: Vec<FlagsConfig<'pm>>,
    pub(crate) options: Vec<OptionsConfig<'pm>>,
    pub(crate) args: ArgsConfig<'pm>,
}

pub(crate) struct FlagsConfig<'a> {
    pub(crate) cursor_index: usize,
    pub(crate) flag: NewFlag<'a>,
    pub(crate) appearance_count: usize,
}

pub(crate) struct OptionsConfig<'o> {
    cursor_index: usize,
    option: NewOption<'o>,
    args: ArgsConfig<'o>,
    appearance_count: usize,
}

impl<'o> OptionsConfig<'o> {
    pub(crate) fn new() -> Self {
        Self {
            appearance_count: 0,
            args: ArgsConfig::new(),
            cursor_index: 0,
            option: NewOption::default(),
        }
    }
}

impl<'d> Default for OptionsConfig<'d> {
    fn default() -> Self {
        Self {
            appearance_count: 0,
            args: ArgsConfig::new(),
            cursor_index: 0,
            option: NewOption::default(),
        }
    }
}

pub(crate) struct CommandConfig<'b> {
    cursor_index: usize,
    command: Command<'static>,
    args: ArgsConfig<'b>,
    flags: Vec<FlagsConfig<'b>>,
}

pub(crate) struct ArgsConfig<'c> {
    cursor_index: usize,
    raw_value: &'c str,
    value_of: &'c str,
}

impl<'args> ArgsConfig<'args> {
    pub(crate) fn new() -> Self {
        Self {
            cursor_index: 0,
            raw_value: "",
            value_of: "",
        }
    }
}

impl<'a> ParserMatches<'a> {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            arg_count: count,
            flags: vec![],
            matched_subcmd: None,
            args: ArgsConfig::new(),
            options: vec![OptionsConfig::default()],
        }
    }
}
