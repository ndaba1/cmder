use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProgramSettings {
    values: HashMap<Setting, bool>,
}

impl ProgramSettings {
    pub fn get(&self, setting: Setting) -> bool {
        self.values[&setting]
    }

    pub fn set(&mut self, setting: Setting, val: bool) {
        self.values.insert(setting, val);
    }
}

impl Default for ProgramSettings {
    fn default() -> Self {
        let mut values = HashMap::new();

        use Setting::*;
        values.insert(AutoIncludeHelpSubcommand, true);
        values.insert(IgnoreAllErrors, false);
        values.insert(OverrideAllDefaultListeners, false);
        values.insert(ShowCommandAliases, false);
        values.insert(ShowHelpOnAllErrors, false);
        values.insert(ShowHelpOnEmptyArgs, true);

        Self { values }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Setting {
    IgnoreAllErrors,
    ShowHelpOnAllErrors,
    ShowHelpOnEmptyArgs,
    ShowCommandAliases,
    OverrideAllDefaultListeners,
    AutoIncludeHelpSubcommand,
}
