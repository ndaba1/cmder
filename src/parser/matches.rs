#![allow(dead_code)]

use std::collections::HashMap;

use super::{Cmd, Flag};

struct ParserMatches {
    arg_count: i32,
    matched_cmd: Option<CommandConfig>,
    flags: FlagsConfig,
    program_args: HashMap<String, String>,
}

struct FlagsConfig {
    cursor_index: i32,
    flag: Flag,
    args: HashMap<String, String>,
}

struct CommandConfig {
    cursor_index: i32,
    command: Cmd,
    is_subcommand: bool,
    args: HashMap<String, String>,
}

impl ParserMatches {
    fn get_matched_cmd() {}

    fn get_values() {}

    fn get_options() {}
}
