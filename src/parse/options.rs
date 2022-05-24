use crate::ui::formatter::FormatGenerator;

use super::Argument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderOption<'op> {
    pub short: &'op str,
    pub long: &'op str,
    pub arguments: Vec<Argument>,
    pub description: &'op str,
    pub required: bool,
    pub name: &'op str,
}

impl<'b> CmderOption<'b> {
    pub fn new(name: &'b str) -> Self {
        Self {
            short: "",
            name,
            arguments: vec![],
            description: "",
            long: "",
            required: false,
        }
    }

    pub fn short(mut self, val: &'b str) -> Self {
        self.short = val;
        self
    }

    pub fn long(mut self, val: &'b str) -> Self {
        self.long = val;
        self
    }

    pub fn help(mut self, val: &'b str) -> Self {
        self.description = val;
        self
    }

    pub fn is_required(mut self, v: bool) -> Self {
        self.required = v;
        self
    }

    pub fn argument(mut self, val: &'b str) -> Self {
        self.arguments.push(Argument::generate(val, None));
        self
    }

    pub fn add_argument(mut self, a: Argument) -> Self {
        self.arguments.push(a);
        self
    }

    pub(crate) fn generate(short: &'b str, long: &'b str, desc: &'b str, args: &[&str]) -> Self {
        let mut arguments = vec![];
        for a in args.iter() {
            arguments.push(Argument::generate(a, None))
        }

        Self {
            short,
            long,
            description: desc,
            arguments,
            required: false,
            name: "",
        }
    }
}

impl<'d> Default for CmderOption<'d> {
    fn default() -> Self {
        Self::generate("", "", "", &[])
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
                let short: String = if !self.short.is_empty() {
                    format!("{},", self.short)
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
                    format!("{} {} {}", short, self.long, args),
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
    fn test_options_creation() {
        let o = CmderOption::generate("-p", "--port", "Port flag", &[]);

        assert_eq!(o.short, "-p");
        assert_eq!(o.long, "--port");
        assert_eq!(o.description, "Port flag");
        assert_eq!(o.required, false);
        assert_eq!(o.arguments, vec![]);
    }
}
