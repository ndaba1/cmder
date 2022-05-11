#![allow(unused)]

use crate::ui::formatter::FormatGenerator;

use super::args::Argument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderOption<'op> {
    pub short_version: &'op str,
    pub long_version: &'op str,
    pub arguments: Vec<Argument>,
    pub description: &'op str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderFlag<'f> {
    pub short_version: &'f str,
    pub long_version: &'f str,
    pub description: &'f str,
}

impl<'a> CmderFlag<'a> {
    pub(crate) fn new(short: &'a str, long: &'a str, desc: &'a str) -> Self {
        Self {
            short_version: short,
            long_version: long,
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
            short_version: short,
            long_version: long,
            description: desc,
            arguments,
        }
    }
}

impl<'d> Default for CmderOption<'d> {
    fn default() -> Self {
        Self {
            short_version: "",
            long_version: "",
            arguments: vec![Argument::new("", None)],
            description: "",
        }
    }
}

pub(crate) fn resolve_new_flag<'f>(list: &'f [CmderFlag], val: String) -> Option<CmderFlag<'f>> {
    let mut flag = None;

    let val = val.as_str();
    for f in list {
        if f.short_version == val || f.long_version == val {
            flag = Some(f.clone());
        }
    }
    flag
}

pub(crate) fn resolve_new_option<'o>(
    list: &'o [CmderOption],
    val: String,
) -> Option<CmderOption<'o>> {
    let mut flag = None;

    let val = val.as_str();
    for f in list {
        if f.short_version == val || f.long_version == val {
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
                    .replace("{{short}}", self.short_version)
                    .replace("{{long}}", self.long_version);

                if leading.contains("{{description}}") {
                    leading = leading.replace("{{description}}", self.description);
                } else {
                    floating = self.description.into()
                }

                (leading, floating)
            }
            _ => {
                let short: String = if !self.short_version.is_empty() {
                    self.short_version.into()
                } else {
                    "  ".into()
                };
                (
                    format!("{}, {}", short, self.long_version),
                    self.description.into(),
                )
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
                    .replace("{{short}}", self.short_version)
                    .replace("{{long}}", self.long_version);

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
                let short: String = if self.short_version.is_empty() {
                    self.short_version.into()
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
                    format!("{}, {} {}", short, self.long_version, args),
                    self.description.into(),
                )
            }
        }
    }
}
