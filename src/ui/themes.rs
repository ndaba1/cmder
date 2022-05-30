use std::collections::HashMap;

use termcolor::Color;

use crate::Designation;

// Theme is defined a struct containing a field of values which itself is a hashmap with Designation as a key and a color as a value. This way, every designation is mapped to a color.
#[derive(Debug, Clone)]
pub struct Theme {
    values: HashMap<Designation, Color>,
}

/// Contains a few predefined themes that can be set to the program
pub enum PredefinedTheme {
    Plain,
    Colorful,
}

pub fn get_predefined_theme(theme: PredefinedTheme) -> Theme {
    use PredefinedTheme::*;
    match theme {
        Colorful => Theme::colorful(),
        Plain => Theme::plain(),
    }
}

impl Theme {
    pub fn new(
        keywords: Color,
        headlines: Color,
        descriptions: Color,
        errors: Color,
        others: Color,
    ) -> Self {
        let mut values = HashMap::new();

        use Designation::*;
        values.insert(Keyword, keywords);
        values.insert(Headline, headlines);
        values.insert(Description, descriptions);
        values.insert(Error, errors);
        values.insert(Other, others);

        Self { values }
    }

    pub fn get(&self, designation: Designation) -> Color {
        self.values[&designation]
    }

    pub fn plain() -> Self {
        use Color::*;
        Self::new(White, White, White, Red, White)
    }

    pub fn colorful() -> Self {
        use Color::*;
        Self::new(Green, Magenta, Blue, Red, White)
    }
}

impl Default for Theme {
    fn default() -> Self {
        use Color::*;
        Self::new(Yellow, Cyan, White, Red, White)
    }
}
