#![allow(unused)]

use std::fmt;

pub enum CmderError<'err> {
    MissingArgument(Vec<&'err str>),
    OptionMissingArgument(Vec<&'err str>),
    UnknownCommand(&'err str),
    UnknownOption(&'err str),
}

// impl<'a> fmt::Display for CmderError<'a>  {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {

//     }
// }
