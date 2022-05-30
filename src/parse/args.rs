use crate::ui::formatter::FormatGenerator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument {
    /// Sets the name of the argument. The name is what actually gets returned in the resulting hashmaps once the arguments are parsed. It is in a rust-friendly format, that is, all the leading hyphens are removed and any other hyphens replaced with underscores
    pub name: String,

    /// Depending on whether the argument is wrapped in angle brackets or square brackets, it is marked as required or not. This field is later checked when the cmd.parse method is called and if a required arg is missing, and error is thrown and the program exits, althought this behavior can be modified by adding a listener to the `Event::OptionMissingArgument` event.
    pub is_required: bool,

    /// The raw literal of the argument, in the same way that it was passed, without any modifications, angle brackets and all.
    pub raw: String,

    /// An optional description about the argument
    pub description: Option<String>,

    /// Whether or not the arg is variadic or not
    pub is_variadic: bool,

    pub valid_values: Vec<String>,
    pub default_value: String,
}

impl Argument {
    pub fn new(val: &str) -> Self {
        let mut delimiters = vec![' ', ' '];
        let mut variadic = false;
        let mut required = false;
        let mut name = val.to_string();
        let mut raw = "".to_owned();

        if name.starts_with('<') {
            delimiters = vec!['<', '>'];
            required = true;
            raw = val.into()
        } else if name.starts_with('[') {
            delimiters = vec!['[', ']'];
            raw = val.into()
        };

        name = name
            .replace(delimiters[0], "")
            .replace(delimiters[1], "")
            .replace('-', "");

        if name.ends_with("...") {
            name = name.replace("...", "");
            variadic = true
        };

        Self {
            raw,
            name,
            description: None,
            is_required: required,
            is_variadic: variadic,
            valid_values: vec![],
            default_value: String::new(),
        }
    }

    pub fn default(mut self, val: &str) -> Self {
        if !self.valid_values.is_empty() && !self.test_value(val) {
            println!("You have provided a default value but it does not match the valid values. It will therefore be ignored");
            return self;
        }
        self.default_value = val.into();
        self
    }

    pub fn help(mut self, val: &str) -> Self {
        self.description = Some(val.into());
        self
    }

    pub fn variadic(mut self, val: bool) -> Self {
        self.is_variadic = val;
        self
    }

    pub fn required(mut self, val: bool) -> Self {
        self.is_required = val;
        self
    }

    pub fn validate_with(mut self, vals: Vec<&str>) -> Self {
        let mut valid = vec![];
        for s in vals {
            valid.push(s.into())
        }
        self.valid_values = valid;
        self
    }

    pub fn display_as(mut self, val: &str) -> Self {
        self.raw = val.into();
        self
    }

    pub fn test_value(&self, val: &str) -> bool {
        if self.valid_values.contains(&val.into()) {
            true
        } else {
            false
        }
    }
}

// Getters for argument values
impl Argument {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_default_value(&self) -> &str {
        &self.default_value
    }

    pub fn has_default_value(&self) -> bool {
        !self.default_value.is_empty()
    }

    pub fn get_valid_values(&self) -> &Vec<String> {
        &self.valid_values
    }

    pub fn is_required(&self) -> bool {
        self.is_required
    }

    pub fn is_variadic(&self) -> bool {
        self.is_variadic
    }

    pub fn get_raw_value(&self) -> String {
        if self.raw.is_empty() {
            let mut builder = String::new();
            let mut enclose = |a, z| {
                builder.push(a);
                builder.push_str(&self.name.replace('_', "-"));
                if self.is_variadic {
                    builder.push_str("...")
                }
                builder.push(z)
            };

            if self.is_required {
                enclose('<', '>')
            } else {
                enclose('[', ']')
            }

            builder
        } else {
            self.raw.clone()
        }
    }
}

impl FormatGenerator for Argument {
    fn generate(&self, _ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        (self.raw.clone(), self.description.clone().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_creation() {
        let arg = Argument::new("<basic>");

        assert!(arg.is_required());
        assert!(!arg.is_variadic());
        assert_eq!(arg.get_name(), "basic");
        assert_eq!(arg.get_raw_value(), "<basic>".to_owned());

        let a = Argument::new("<text...>").help("Variadic text");
        let b = Argument::new("text")
            .required(true)
            .variadic(true)
            .help("Variadic text")
            .display_as("<text...>");

        assert_eq!(a, b);

        let mut arg = Argument::new("<arg>").validate_with(vec!["1", "2", "3"]);

        assert!(arg.test_value("2"));
        assert!(!arg.test_value("4"));

        arg = arg.default("3");
        assert_eq!(arg.get_default_value(), "3");

        // Invalid arg default value should be ignored
        arg = arg.default("6");
        assert_eq!(arg.get_default_value(), "3")
    }
}
