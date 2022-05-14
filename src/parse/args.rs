use crate::ui::formatter::FormatGenerator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument {
    /// Sets the name of the argument. The name is what actually gets returned in the resulting hashmaps once the arguments are parsed. It is in a rust-friendly format, that is, all the leading hyphens are removed and any other hyphens replaced with underscores
    pub name: String,

    /// Depending on whether the argument is wrapped in angle brackets or square brackets, it is marked as required or not. This field is later checked when the cmd.parse method is called and if a required arg is missing, and error is thrown and the program exits, althought this behavior can be modified by adding a listener to the `Event::OptionMissingArgument` event.
    pub required: bool,

    /// The raw literal of the argument, in the same way that it was passed, without any modifications, angle brackets and all.
    pub literal: String,

    /// An optional description about the argument
    pub description: Option<String>,

    /// Whether or not the arg is variadic or not
    pub variadic: bool,
}

impl Argument {
    /// Takes in a string literal as input and returns a new argument instance after resolving all the struct fields of an argument by calling the `clean_arg` function.
    pub fn new(value: &str, description: Option<String>) -> Self {
        let (name, required, variadic) = clean_arg(value.trim());

        Self {
            name,
            required,
            literal: value.to_string(),
            description,
            variadic,
        }
    }

    /// A method that takes in a vector of arguments, determines and returns which of the arguments are required.
    pub fn get_required_args(list: &[Self]) -> Vec<Self> {
        let mut req = vec![];

        for arg in list {
            if arg.required {
                req.push(arg.clone())
            }
        }

        req
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
    fn generate(&self, ptrn: crate::ui::formatter::Pattern) -> (String, String) {
        use crate::ui::formatter::Pattern;
        match &ptrn {
            Pattern::Custom(ptrn) => {
                let base = &ptrn.args_fmter;

                let mut floating = String::from("");
                let mut leading = base
                    .replace("{{name}}", &self.name)
                    .replace("{{literal}}", &self.literal);

                if base.contains("{{description}}") {
                    leading =
                        leading.replace("{{description}}", &self.description.clone().unwrap());
                } else {
                    floating = self.description.clone().unwrap()
                }

                (leading, floating)
            }
            _ => (self.literal.clone(), self.description.clone().unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_creation() {
        let a = Argument::new("<test-app>", Some("Dummy help str".into()));

        assert!(a.required);
        assert!(!a.variadic);
        assert_eq!(a.name, "test_app");
        assert_eq!(a.description, Some("Dummy help str".into()));
    }
}
