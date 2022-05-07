use crate::{Pattern, PredefinedThemes, Theme};

#[derive(Debug, Clone)]
pub struct ProgramSettings {
    pub theme: Theme,
    pub pattern: Pattern,
    pub help_on_error: bool,
    pub suggest_cmds: bool,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct NewProgramSettings {
    pub(crate) show_help_on_error: bool,
    pub(crate) enable_command_suggestions: bool,
    pub(crate) hide_command_aliases: bool,
    pub(crate) separate_options_and_flags: bool,
}

impl Default for NewProgramSettings {
    fn default() -> Self {
        Self {
            show_help_on_error: false,
            enable_command_suggestions: true,
            hide_command_aliases: true,
            separate_options_and_flags: false,
        }
    }
}

#[allow(unused)]
pub enum Setting {
    ShowHelpOnError(bool),
    EnableCommandSuggestion(bool),
    HideCommandAliases(bool),
    SeparateOptionsAndFlags(bool),
    DefineCustomTheme(Theme),
    ChoosePredefinedTheme(PredefinedThemes),
    SetProgramPattern(Pattern),
}

impl ProgramSettings {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            pattern: Pattern::Legacy,
            help_on_error: true,
            suggest_cmds: true,
        }
    }

    // fn set(settng: Settings) -> fn() -> () {}
}

impl Default for ProgramSettings {
    fn default() -> Self {
        Self::new()
    }
}
