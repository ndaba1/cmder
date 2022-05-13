use crate::{Event, Pattern, PredefinedThemes, Theme};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ProgramSettings {
    pub(crate) ignore_all_errors: bool,
    pub(crate) hide_command_aliases: bool,
    pub(crate) show_help_on_all_errors: bool,
    pub(crate) show_help_on_empty_args: bool,
    pub(crate) enable_command_suggestions: bool,
    pub(crate) events_to_override: Vec<Event>,
    pub(crate) separate_options_and_flags: bool,
    pub(crate) override_all_default_listeners: bool,
    pub(crate) auto_include_help_subcommand: bool,
    pub(crate) enable_tree_view_subcommand: bool,
}

impl Default for ProgramSettings {
    fn default() -> Self {
        Self {
            ignore_all_errors: false,
            show_help_on_all_errors: false,
            show_help_on_empty_args: true,
            enable_command_suggestions: true,
            hide_command_aliases: true,
            separate_options_and_flags: false,
            override_all_default_listeners: false,
            events_to_override: vec![],
            auto_include_help_subcommand: true,
            enable_tree_view_subcommand: true,
        }
    }
}

#[allow(unused)]
pub enum Setting {
    IgnoreAllErrors(bool),
    ShowHelpOnAllErrors(bool),
    ShowHelpOnEmptyArgs(bool),
    EnableCommandSuggestion(bool),
    HideCommandAliases(bool),
    SeparateOptionsAndFlags(bool),
    DefineCustomTheme(Theme),
    ChoosePredefinedTheme(PredefinedThemes),
    SetProgramPattern(Pattern),
    OverrideAllDefaultListeners(bool),
    OverrideSpecificEventListener(Event),
    AutoIncludeHelpSubcommand(bool),
    EnableTreeViewSubcommand(bool),
}
