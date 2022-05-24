#![allow(unused)]

use crate::{Command, Pattern, Theme};
pub struct ErrorWriter<'help> {
    theme: Theme,
    pattern: Pattern,
    cmd: &'help Command<'help>,
    error: &'help str,
}
