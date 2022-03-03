use crate::parser::{Cmd, Flag};

use super::super::parser::commands::Argument;

pub fn clean_arg(val: &str) -> Argument {
    let delimiters: &str;
    let required: bool;

    if val.starts_with('<') {
        delimiters = "< >";
        required = true;
    } else {
        delimiters = "[ ]";
        required = false;
    }

    let vals: Vec<_> = delimiters.split(' ').collect();
    let start = vals[0];
    let last = vals[1];

    let name = val.replace(start, "").replace(last, "").replace('-', "_");
    Argument {
        name,
        required,
        literal: val.to_string(),
    }
}

pub fn filter_flags(cmd: &Cmd, args: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut flags = vec![];
    let mut vals = vec![];
    for (i, a) in args.iter().enumerate() {
        if a.starts_with('-') {
            flags.push(a.clone());
            match get_flag(a, cmd) {
                Some(f) => {
                    let _len = f.params.len();
                    for (idx, p) in f.params.iter().enumerate() {
                        if p.required {
                            flags.push(args[i + (idx + 1)].clone())
                        } else {
                            if !args[i + 1].starts_with('-') {
                                flags.push(args[i + (idx + 1)].clone())
                            }
                        }
                    }
                }
                None => {}
            }
        }
    }

    for a in args {
        if !flags.contains(a) {
            vals.push(a.clone())
        }
    }
    (flags, vals)
}

fn get_flag(val: &str, cmd: &Cmd) -> Option<Flag> {
    let mut ans: Option<Flag> = None;

    for f in &cmd.options {
        if val == f.short || val == f.long {
            ans = Some(f.clone());
        } else {
            ans = None;
        }
    }

    ans
}
