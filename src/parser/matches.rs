#![allow(dead_code)]

use crate::core::new_program::Command;

use super::Flag;

pub struct ParserMatches<'pm> {
    pub(crate) arg_count: usize,
    pub(crate) matched_subcmd: Option<CommandConfig<'pm>>,
    pub(crate) flags: Vec<FlagsConfig<'pm>>,
    pub(crate) args: ArgsConfig<'pm>,
}

pub(crate) struct FlagsConfig<'a> {
    cursor_index: usize,
    flag: Flag,
    args: ArgsConfig<'a>,
    appearance_count: usize,
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
        }
    }

    fn get_matched_cmd() {}

    fn get_values() {}

    fn get_options() {}
}
