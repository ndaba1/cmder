use cmder::{Command, Event, Program, Setting};

fn create_program() -> Command<'static> {
    let mut p = Program::new();

    p.set(Setting::AutoIncludeHelpSubcommand(false));
    p.set(Setting::HideCommandAliases(true));
    p.set(Setting::OverrideSpecificEventListener(Event::OutputVersion));
    p.set(Setting::ShowHelpOnAllErrors(true));
    p.set(Setting::ShowHelpOnEmptyArgs(true));

    p.init_dbg();

    p
}

#[test]
fn test_settings() {
    let p = create_program();

    // TODO: Finish up test
    assert!(p.get_subcommands().len() == 0);
}
