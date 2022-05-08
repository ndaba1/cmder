use crate::{Event, Pattern, PredefinedThemes, Theme};

#[derive(Debug, Clone)]
pub struct ProgramSettings {
    pub theme: Theme,
    pub pattern: Pattern,
    pub help_on_error: bool,
    pub suggest_cmds: bool,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) struct NewProgramSettings {
    pub(crate) show_help_on_error: bool,
    pub(crate) enable_command_suggestions: bool,
    pub(crate) hide_command_aliases: bool,
    pub(crate) events_to_override: Vec<Event>,
    pub(crate) separate_options_and_flags: bool,
    pub(crate) override_all_default_listeners: bool,
}

impl Default for NewProgramSettings {
    fn default() -> Self {
        Self {
            show_help_on_error: false,
            enable_command_suggestions: true,
            hide_command_aliases: true,
            separate_options_and_flags: false,
            override_all_default_listeners: false,
            events_to_override: vec![],
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
    OverrideAllDefaultListeners(bool),
    OverrideSpecificEventListener(Event),
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
