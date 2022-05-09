use termcolor::Color;

/// A simple struct containing the color palette to be used by the formatter when writing to standard output.
/// It contains various fields each referring to a color variant to be used to print a specific value.
/// Each of these values can be mapped to values in the `Designation` struct which is simply a fancy way of referring to what role a string value is assigned to.
#[derive(Debug, Clone)]
pub struct Theme {
    /// This field sets the color to use when printing out keywords: i.e program name/flags etc. Default is yellow
    pub keyword: Color,

    /// Contains the color to be used for headlines eg. "COMMANDS: " "OPTIONS: " Default is Cyan
    pub headline: Color,

    /// Sets the color to use for descriptive texts, the default is white
    pub description: Color,

    /// Sets the color to use for error messages, the default is red
    pub error: Color,

    /// Any other designations will use the value set in the other field.
    pub other: Color,
}

/// This macro eases the work of creating a new theme, instead of creating the theme struct yourself, you can use the macro to do so.
/// The macro receives  all the desired colors. Do note that the order in which you place your colors is important.
/// 1. Keywords color
/// 2. Headlines color
/// 3. Descriptions color
/// 4. Errors color
/// 5. Others' color
#[macro_export]
macro_rules! construct_theme {
    ($kw:expr, $hd:expr, $dsc:expr, $err:expr, $othr:expr) => {
        Theme {
            keyword: $kw,
            headline: $hd,
            description: $dsc,
            error: $err,
            other: $othr,
        }
    };
}
/// Contains a few predefined themes that can be set to the program
pub enum PredefinedThemes {
    Plain,
    Colorful,
}

impl Theme {
    pub fn new() -> Self {
        use Color::*;
        construct_theme!(Yellow, Cyan, White, Red, White)
    }

    pub fn plain() -> Self {
        use Color::*;
        construct_theme!(White, White, White, Red, White)
    }

    pub fn colorful() -> Self {
        use Color::*;
        construct_theme!(Green, Magenta, Blue, Red, White)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}
