use crate::ui::formatter::FormatGenerator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmderFlag<'f> {
    pub short: &'f str,
    pub long: &'f str,
    pub name: &'f str,
    pub description: &'f str,
}

impl<'a> CmderFlag<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            short: "",
            long: "",
            name,
            description: "",
        }
    }

    pub fn short(mut self, val: &'a str) -> Self {
        self.short = val;
        self
    }

    pub fn long(mut self, val: &'a str) -> Self {
        self.long = val;
        self
    }

    pub fn help(mut self, val: &'a str) -> Self {
        self.description = val;
        self
    }

    pub(crate) fn generate(short: &'a str, long: &'a str, desc: &'a str) -> Self {
        Self {
            short,
            long,
            name: "",
            description: desc,
        }
    }
}

impl<'d> Default for CmderFlag<'d> {
    fn default() -> Self {
        Self::generate("", "", "")
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
                    format!("{},", self.short)
                } else {
                    "  ".into()
                };
                (format!("{} {}", short, self.long), self.description.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_manual_creation() {
        let f = CmderFlag::generate("-h", "--help", "A help flag");

        assert_eq!(f.short, "-h");
        assert_eq!(f.long, "--help");
        assert_eq!(f.description, "A help flag");
    }

    #[test]
    fn test_auto_creation() {
        let f = CmderFlag::new("help")
            .help("A help flag")
            .short("-h")
            .long("--help");

        assert_eq!(f.short, "-h");
        assert_eq!(f.long, "--help");
        assert_eq!(f.description, "A help flag");
    }
}
