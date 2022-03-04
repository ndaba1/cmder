#![allow(dead_code)]

// Headline => Color::Cyan,
//             Description => Color::White,
//             Warning => Color::Yellow,
//             Error => Color::Red,
//             Other => Color::White,
//             Keyword => Color::Yellow,
//             Special => Color::Green,

use termcolor::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub keyword: Color,
    pub headline: Color,
    pub description: Color,
    pub error: Color,
    pub other: Color,
}

pub enum PredefinedThemes {
    Plain,
    Colorful,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            keyword: Color::Yellow,
            headline: Color::Cyan,
            description: Color::White,
            error: Color::Red,
            other: Color::White,
        }
    }

    pub fn plain() -> Self {
        Self {
            keyword: Color::White,
            headline: Color::White,
            description: Color::White,
            error: Color::Red,
            other: Color::White,
        }
    }

    pub fn colorful() -> Self {
        Self {
            keyword: Color::Blue,
            headline: Color::Magenta,
            description: Color::White,
            error: Color::Red,
            other: Color::Yellow,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}
