use crate::{Pattern, Theme};

#[derive(Debug, Clone)]
pub struct ProgramSettings {
    pub theme: Theme,
    pub pattern: Pattern,
    pub help_on_error: bool,
    pub suggest_cmds: bool,
}

#[allow(unused)]
pub enum Settings {
    ShowHelpOnError,
    EnableCommandSuggestion,
    HideCommandAliases,
    MaxSubcommandLevel,
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
