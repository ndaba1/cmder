use super::super::parser::commands::Argument;

pub fn clean_arg(val: &str) -> Argument {
    let delimiters: &str;

    if val.starts_with('<') {
        delimiters = "< >"
    } else {
        delimiters = "[ ]"
    }

    let vals: Vec<_> = delimiters.split(' ').collect();
    let start = vals[0];
    let last = vals[1];

    let name = val.replace(start, "").replace(last, "").replace('-', "_");
    Argument {
        name,
        required: true,
        literal: val.to_string(),
    }
}
