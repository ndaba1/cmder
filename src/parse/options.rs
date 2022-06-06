use crate::ui::formatter::FormatGenerator;

use super::Argument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderOption<'op> {
    pub(crate) name: String,
    pub(crate) short: String,
    pub(crate) long: String,
    pub(crate) arguments: Vec<Argument>,
    pub(crate) description: &'op str,
    pub(crate) is_required: bool,
    pub(crate) is_global: bool,
}

impl<'b> CmderOption<'b> {
    pub fn new(name: &'b str) -> Self {
        let mut long = String::from("--");
        long.push_str(name);
        Self {
            name: name.into(),
            short: "".into(),
            long,
            arguments: vec![],
            description: "",
            is_required: false,
            is_global: false,
        }
    }

    pub fn short(mut self, val: char) -> Self {
        let mut short = String::from("-");
        short.push(val);
        self.short = short;
        self
    }

    pub fn help(mut self, val: &'b str) -> Self {
        self.description = val;
        self
    }

    pub fn required(mut self, v: bool) -> Self {
        self.is_required = v;
        self
    }

    pub fn global(mut self, v: bool) -> Self {
        self.is_global = v;
        self
    }

    pub fn argument(mut self, val: &'b str) -> Self {
        self.arguments.push(Argument::new(val));
        self
    }

    pub fn add_argument(mut self, a: Argument) -> Self {
        self.arguments.push(a);
        self
    }
}

impl<'d> Default for CmderOption<'d> {
    fn default() -> Self {
        Self::new("")
    }
}

pub(crate) fn resolve_option<'o>(list: &'o [CmderOption], val: String) -> Option<CmderOption<'o>> {
    let mut flag = None;

    let val = val.as_str();
    for f in list {
        if f.short == val || f.long == val {
            flag = Some(f.clone());
        }
    }
    flag
}

impl<'f> FormatGenerator for CmderOption<'f> {
    fn generate(&self, _ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        let short: String = if !self.short.is_empty() {
            format!("{},", self.short)
        } else {
            "  ".into()
        };

        let args = if !self.arguments.is_empty() {
            let mut raw = String::new();

            for a in &self.arguments {
                raw.push_str(&(a.get_raw_value()));
                raw.push(' ');
            }

            raw
        } else {
            "".into()
        };

        (
            format!("{} {} {}", short, self.long, args),
            self.description.into(),
        )
    }
}

pub fn new_option(val: &str, help: &'static str) -> CmderOption<'static> {
    let values: Vec<_> = val.split_whitespace().collect();

    let mut short = "";
    let mut long = "";
    let mut args = vec![];
    let mut raw_args = vec![];

    for v in &values {
        if v.starts_with("--") {
            long = v;
        } else if v.starts_with('-') {
            short = v
        } else {
            raw_args.push(v);
        }
    }

    for a in raw_args {
        let arg = Argument::new(a);
        args.push(arg);
    }

    CmderOption {
        name: long.replace("--", ""),
        short: short.into(),
        long: long.into(),
        arguments: args,
        description: help,
        is_required: false,
        is_global: false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_options_creation() {
        let opt = CmderOption::new("port")
            .short('p')
            .help("A port option")
            .required(true)
            .add_argument(Argument::new("value").required(true).default("9000"));

        assert!(opt.is_required);
        assert!(!opt.is_global);
        assert_eq!(opt.description, "A port option");
        assert_eq!(opt.short, "-p".to_owned());
        assert_eq!(opt.long, "--port".to_owned());
        assert_eq!(opt.arguments.len(), 1);
    }
}
