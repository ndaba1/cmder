#![allow(unused)]

use crate::ui::formatter::FormatGenerator;

use super::args::Argument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderOption<'op> {
    pub short: &'op str,
    pub long: &'op str,
    pub arguments: Vec<Argument>,
    pub description: &'op str,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderFlag<'f> {
    pub short: &'f str,
    pub long: &'f str,
    pub description: &'f str,
}

impl<'a> CmderFlag<'a> {
    pub(crate) fn new(short: &'a str, long: &'a str, desc: &'a str) -> Self {
        Self {
            short,
            long,
            description: desc,
        }
    }
}

impl<'d> Default for CmderFlag<'d> {
    fn default() -> Self {
        Self::new("", "", "")
    }
}

impl<'b> CmderOption<'b> {
    pub(crate) fn new(short: &'b str, long: &'b str, desc: &'b str, args: &[&str]) -> Self {
        let mut arguments = vec![];
        for a in args.iter() {
            arguments.push(Argument::new(a, None))
        }

        Self {
            short,
            long,
            description: desc,
            arguments,
            required: false,
        }
    }
}

impl<'d> Default for CmderOption<'d> {
    fn default() -> Self {
        Self::new("", "", "", &[])
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

impl<'f> FormatGenerator for CmderFlag<'f> {
    fn generate(&self, ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        use crate::ui::formatter::Pattern;
        match &ptrn {
            Pattern::Custom(ptrn) => {
                let base = &ptrn.flags_fmter;

                let mut floating = String::from("");
                let mut leading = base
                    .replace("{{short}}", self.short)
                    .replace("{{long}}", self.long);

                if leading.contains("{{description}}") {
                    leading = leading.replace("{{description}}", self.description);
                } else {
                    floating = self.description.into()
                }

                (leading, floating)
            }
            _ => {
                let short: String = if !self.short.is_empty() {
                    self.short.into()
                } else {
                    "  ".into()
                };
                (format!("{}, {}", short, self.long), self.description.into())
            }
        }
    }
}

impl<'f> FormatGenerator for CmderOption<'f> {
    fn generate(&self, ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        use crate::ui::formatter::Pattern;
        match &ptrn {
            Pattern::Custom(ptrn) => {
                let base = &ptrn.flags_fmter;

                let mut floating = String::from("");
                let mut leading = base
                    .replace("{{short}}", self.short)
                    .replace("{{long}}", self.long);

                if base.contains("{{args}}") && !self.arguments.is_empty() {
                    let mut value = String::new();

                    for a in &self.arguments {
                        value.push_str(&(a.literal));
                        value.push(' ');
                    }

                    leading = leading.replace("{{args}}", value.trim());
                }

                if base.contains("{{description}}") {
                    leading = leading.replace("{{description}}", self.description);
                } else {
                    floating = self.description.into()
                }

                (leading, floating)
            }
            _ => {
                let short: String = if self.short.is_empty() {
                    self.short.into()
                } else {
                    "  ".into()
                };

                let args = if !self.arguments.is_empty() {
                    let mut raw = String::new();

                    for a in &self.arguments {
                        raw.push_str(&(a.literal));
                        raw.push(' ');
                    }

                    raw
                } else {
                    "".into()
                };

                (
                    format!("{}, {} {}", short, self.long, args),
                    self.description.into(),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_flag_creation() {
        let f = CmderFlag::new("-h", "--help", "A help flag");

        assert_eq!(f.short, "-h");
        assert_eq!(f.long, "--help");
        assert_eq!(f.description, "A help flag");
    }

    #[test]
    fn test_options_creation() {
        let o = CmderOption::new("-p", "--port", "Port flag", &[]);

        assert_eq!(o.short, "-p");
        assert_eq!(o.long, "--port");
        assert_eq!(o.description, "Port flag");
        assert_eq!(o.required, false);
        assert_eq!(o.arguments, vec![]);
    }
}
