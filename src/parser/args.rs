#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    /// Sets the name of the argument. The name is what actually gets returned in the resulting hashmaps once the arguments are parsed. It is in a rust-friendly format, that is, all the leading hyphens are removed and any other hyphens replaced with underscores
    pub name: String,

    /// Depending on whether the argument is wrapped in angle brackets or square brackets, it is marked as required or not. This field is later checked when the cmd.parse method is called and if a required arg is missing, and error is thrown and the program exits, althought this behavior can be modified by adding a listener to the `Event::OptionMissingArgument` event.
    pub required: bool,

    /// The raw literal of the argument, in the same way that it was passed, without any modifications, angle brackets and all.
    pub literal: String,
}

impl Argument {
    pub fn new(value: &str) -> Self {
        let (name, required) = clean_arg(value);

        Self {
            name,
            required,
            literal: value.to_string(),
        }
    }

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

fn clean_arg(val: &str) -> (String, bool) {
    let delimiters;

    let required = if val.starts_with('<') {
        delimiters = vec!['<', '>'];
        true
    } else {
        delimiters = vec!['[', ']'];
        false
    };

    let name = val
        .replace(delimiters[0], "")
        .replace(delimiters[1], "")
        .replace('-', "_");

    (name, required)
}
