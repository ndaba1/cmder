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
        let raw = val.to_owned();

        if name.starts_with('<') {
            delimiters = vec!['<', '>'];
            required = true
        } else if name.starts_with('[') {
            delimiters = vec!['[', ']'];
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

    /// Takes in a string literal as input and returns a new argument instance after resolving all the struct fields of an argument by calling the `clean_arg` function.
    pub fn generate(value: &str, description: Option<String>) -> Self {
        let (name, required, variadic) = clean_arg(value.trim());

        Self {
            name,
            is_required: required,
            raw: value.to_string(),
            description,
            is_variadic: variadic,
            valid_values: vec![],
            default_value: String::new(),
        }
    }

    pub fn default(mut self, val: &str) -> Self {
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
}

/// Cleans an argument by removing any brackets and determining whether the argument is required is not.
fn clean_arg(val: &str) -> (String, bool, bool) {
    let delimiters;

    let required = if val.starts_with('<') {
        delimiters = vec!['<', '>'];
        true
    } else {
        delimiters = vec!['[', ']'];
        false
    };

    let mut name = val
        .replace(delimiters[0], "")
        .replace(delimiters[1], "")
        .replace('-', "_");

    let variadic = if name.ends_with("...") {
        name = name.replace("...", "");
        true
    } else {
        false
    };

    (name, required, variadic)
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
        // TODO: Update tests heavily
        let a = Argument::generate("<test-app>", Some("Dummy help str".into()));

        assert!(a.is_required);
        assert!(!a.is_variadic);
        assert_eq!(a.name, "test_app");
        assert_eq!(a.description, Some("Dummy help str".into()));
    }
}
