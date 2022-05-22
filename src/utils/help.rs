use crate::{core::Command, ui::formatter::Pattern, Designation, Formatter, Theme};

pub struct HelpWriter {}

impl<'help> HelpWriter {
    pub fn write(cmd: &'help Command<'help>, theme: &Theme, ptrn: &Pattern) {
        let mut fmter = Formatter::new(theme.to_owned());

        // Utility vars
        let has_flags = !cmd.get_flags().is_empty();
        let has_args = !cmd.get_arguments().is_empty();
        let has_options = !cmd.get_options().is_empty();
        let has_subcmds = !cmd.get_subcommands().is_empty();
        let has_info = !cmd.get_cmd_info().is_empty();

        use Designation::*;

        if !cmd.get_description().is_empty() {
            fmter.add(Description, &format!("{}\n", cmd.get_description()));
        }

        fmter.section("USAGE");
        fmter.add(Keyword, &format!("    {}", cmd.get_usage_str()));
        fmter.add(Other, " [OPTIONS]");

        if has_args {
            fmter.add(Other, " <ARGS>");
        }

        if has_subcmds {
            fmter.add(Other, " <SUBCOMMAND>");
        }
        fmter.close();

        if has_args {
            fmter.section("ARGS");
            fmter.format(cmd.get_arguments(), ptrn);
        }

        if has_flags {
            fmter.section("FLAGS");
            fmter.format(cmd.get_flags(), ptrn);
        }

        if has_options {
            fmter.section("OPTIONS");
            fmter.format(cmd.get_options(), ptrn);
        }

        if has_subcmds {
            fmter.section("SUB-COMMANDS");
            fmter.format(cmd.get_subcommands(), ptrn);
        }

        if has_info {
            fmter.section("INFO");
            fmter.add(Description, cmd.get_cmd_info());
        }

        fmter.print();
    }
}
