#![allow(dead_code)]

use crate::{parser::Cmd, Designation, Event, Formatter, FormatterRules, Program};

pub fn print_help(program: &Program, cmd: Option<&Cmd>, error: &str) {
    let mut fmtr = Formatter::new(program.get_theme().to_owned());

    use Designation::*;

    let description = if let Some(cmd) = &cmd {
        cmd.get_description()
    } else {
        program.get_description()
    };

    if !description.is_empty() {
        fmtr.add(Description, &format!("\n{}\n", description))
    }

    fmtr.add(Headline, "\nUSAGE: \n");

    if let Some(cmd) = &cmd {
        let mut params = String::new();
        for p in cmd.get_cmd_input() {
            params.push_str(p.literal.as_str());
            params.push(' ');
        }

        fmtr.add(Keyword, &format!("   {} ", program.get_bin_name()));

        if cmd.is_subcommand() {
            let parent = cmd.parent.clone();
            let cmd = parent.unwrap();
            fmtr.add(Keyword, &format!("{} ", cmd.get_name()))
        }

        fmtr.add(Keyword, &format!("{} ", cmd.get_name()));

        if !cmd.get_subcommands().is_empty() {
            fmtr.add(Description, "<SUB-COMMAND> ")
        }

        fmtr.add(Description, &format!("[options] {} \n", params.trim()));
    } else {
        fmtr.add(Keyword, &format!("   {} ", program.get_bin_name()));

        let get_args = || {
            let mut temp = String::new();
            for arg in &program.get_input().to_owned() {
                temp.push_str(&arg.literal);
                temp.push(' ');
            }
            temp
        };

        if !program.get_all_cmds().is_empty() && !program.get_input().is_empty() {
            let body = format!("[options] <COMMAND> | {} \n", get_args().trim());
            fmtr.add(Description, &body);
        } else if !program.get_all_cmds().is_empty() && program.get_input().is_empty() {
            fmtr.add(Description, "[options] <COMMAND> \n")
        } else {
            fmtr.add(Description, &format!("[options] {} \n", get_args().trim()))
        }
    }

    fmtr.add(Headline, "\nOPTIONS: \n");

    let options = if let Some(cmd) = &cmd {
        cmd.get_cmd_options()
    } else {
        program.get_options()
    };

    fmtr.format(
        FormatterRules::Option(program.get_pattern().clone()),
        Some(options.clone()),
        None,
        None,
    );

    let arguments = if let Some(cmd) = &cmd {
        cmd.get_cmd_input()
    } else {
        program.get_input()
    };

    if !arguments.is_empty() {
        fmtr.add(Headline, "\nARGS: \n");
        fmtr.format(
            FormatterRules::Args(program.get_pattern().clone()),
            None,
            None,
            Some(arguments.clone()),
        );
    }

    if let Some(cmd) = &cmd {
        if !cmd.get_subcommands().is_empty() {
            fmtr.add(Headline, "\nSUB-COMMANDS: \n");
            fmtr.format(
                FormatterRules::Cmd(program.get_pattern().clone()),
                None,
                Some(cmd.get_subcommands().clone()),
                None,
            );
        }
    } else if !program.get_all_cmds().is_empty() {
        fmtr.add(Headline, "\nCOMMANDS: \n");
        fmtr.format(
            FormatterRules::Cmd(program.get_pattern().clone()),
            None,
            Some(program.get_all_cmds().clone()),
            None,
        );
    }

    if !error.is_empty() {
        fmtr.add(Error, &format!("\nError: {}\n", error))
    }

    fmtr.print();

    if let Some(_cmd) = &cmd {
        program.emit(Event::OutputCommandHelp, "")
    } else {
        program.emit(Event::OutputHelp, "")
    }
}

pub fn print_error(error: &str, program: &Program) {
    let mut fmtr = Formatter::new(program.get_theme().to_owned());

    use Designation::*;
    fmtr.add(Error, error);

    fmtr.print();
}
