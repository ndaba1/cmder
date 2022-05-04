#![allow(unused)]

use std::fmt;

pub enum CmderError<'err> {
    MissingArgument(Vec<&'err str>),
    OptionMissingArgument(Vec<&'err str>),
    UnknownCommand(&'err str),
    UnknownOption(&'err str),
}
