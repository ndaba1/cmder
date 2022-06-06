use crate::ui::formatter::FormatGenerator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderFlag<'f> {
    pub(crate) name: String,
    pub(crate) long: String,
    pub(crate) short: String,
    pub(crate) description: &'f str,
    pub(crate) is_global: bool,
}

impl<'a> CmderFlag<'a> {
    pub fn new(name: &'a str) -> Self {
        let mut long = String::from("--");
        long.push_str(name);
        Self {
            name: name.into(),
            short: "".into(),
            long,
            description: "",
            is_global: false,
        }
    }

    pub fn short(mut self, val: char) -> Self {
        let mut short = String::from("-");
        short.push(val);
        self.short = short;
        self
    }

    pub fn help(mut self, val: &'a str) -> Self {
        self.description = val;
        self
    }

    pub fn global(mut self, val: bool) -> Self {
        self.is_global = val;
        self
    }
}

impl<'d> Default for CmderFlag<'d> {
    fn default() -> Self {
        Self::new("")
    }
}

pub(crate) fn resolve_flag<'f>(list: &'f [CmderFlag], val: String) -> Option<CmderFlag<'f>> {
    let mut flag = None;

    let val = val.as_str();
    for f in list {
        if f.short == val || f.long == val {
            flag = Some(f.clone());
        }
    }
    flag
}

impl<'f> FormatGenerator for CmderFlag<'f> {
    fn generate(&self, _ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        let short: String = if !self.short.is_empty() {
            format!("{},", self.short)
        } else {
            "  ".into()
        };
        (format!("{} {}", short, self.long), self.description.into())
    }
}

pub(crate) fn new_flag(val: &str, help: &'static str) -> CmderFlag<'static> {
    let values: Vec<_> = val.split_whitespace().collect();

    let mut short = "";
    let mut long = "";

    for v in &values {
        if v.starts_with("--") {
            long = v;
        } else if v.starts_with('-') {
            short = v
        }
    }

    CmderFlag {
        name: long.replace("--", ""),
        long: long.into(),
        short: short.into(),
        description: help,
        is_global: false,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_flag_creation() {
        let f = CmderFlag::new("help")
            .short('h')
            .help("Help flag")
            .global(true);

        assert!(f.is_global);
        assert_eq!(f.name, "help".to_owned());
        assert_eq!(f.long, "--help".to_owned());
        assert_eq!(f.short, "-h".to_owned());
        assert_eq!(f.description, "Help flag")
    }
}
