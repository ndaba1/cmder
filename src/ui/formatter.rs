use std::io::Write;
use termcolor::{Buffer, BufferWriter, ColorChoice, ColorSpec, WriteColor};

use crate::{
    parser::{Cmd, Flag},
    Theme,
};

#[macro_export]
macro_rules! resolve_formatter {
    ($fmtr:expr, $type:ty, $cb:ident, $vals:expr, $ptrn:expr) => {
        let vals = $vals;
        let ptrn = &$ptrn;

        for (value, docstring) in $cb(&vals, ptrn) {
            let val;
            match ptrn {
                Pattern::Legacy => {
                    let mut default_buf_size = 0;
                    for (value, _) in $cb(&vals, ptrn) {
                        if value.capacity() > default_buf_size {
                            // add some padding
                            default_buf_size = value.capacity() + 5;
                        }
                    }
                    val = legacy_format_output(&value, &docstring, default_buf_size);
                }
                Pattern::Standard => val = standard_format_output(&value, &docstring),
                Pattern::Custom(ref _str) => val = standard_format_output(&value, &docstring),
            }
            $fmtr.add(Designation::Keyword, &format!("\t{}", val.0));
            $fmtr.add(Designation::Description, &format!("{}\n", val.1));
        }
    };
}

pub struct Formatter {
    buffer: Buffer,
    #[allow(unused)]
    writer: BufferWriter,
    #[allow(unused)]
    theme: Theme,
}

pub enum FormatterRules {
    Option(Pattern),
    Cmd(Pattern),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Legacy,
    Standard,
    Custom(String),
}

pub enum Designation {
    Headline,
    Description,
    Error,
    Other,
    Keyword,
}

impl Formatter {
    pub fn new(theme: Theme) -> Self {
        let bfwrt = BufferWriter::stderr(ColorChoice::Always);
        let buffer = bfwrt.buffer();
        Self {
            writer: bfwrt,
            buffer,
            theme,
        }
    }

    pub fn add(&mut self, designation: Designation, value: &str) {
        use Designation::*;

        let color = match designation {
            Headline => self.theme.headline,
            Description => self.theme.description,
            Error => self.theme.error,
            Other => self.theme.other,
            Keyword => self.theme.keyword,
        };

        let temp_writer = BufferWriter::stderr(ColorChoice::Always);
        let mut temp_buff = temp_writer.buffer();

        let og_buff = &mut self.buffer;

        temp_buff
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        write!(&mut temp_buff, "{}", value).unwrap();

        og_buff.write_all(temp_buff.as_slice()).unwrap();
        // &self.buffer.write(temp_buff.as_slice()).unwrap();

        WriteColor::reset(&mut temp_buff).unwrap();
        WriteColor::reset(og_buff).unwrap();
    }

    pub fn format(
        &mut self,
        rule: FormatterRules,
        flags: Option<Vec<Flag>>,
        cmds: Option<Vec<Cmd>>,
    ) {
        match rule {
            FormatterRules::Cmd(ptrn) => {
                resolve_formatter!(self, Cmd, command_iterator, cmds.unwrap(), ptrn);
            }
            FormatterRules::Option(ptrn) => {
                resolve_formatter!(self, Flag, option_iterator, flags.unwrap(), ptrn);
            }
        }
    }

    pub fn print(&mut self) {
        self.writer.print(&self.buffer).unwrap();
        WriteColor::reset(&mut self.buffer).unwrap();
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

fn legacy_format_output(pre: &str, leading: &str, cap: usize) -> (String, String) {
    let mut string_buff = String::with_capacity(cap);
    string_buff.push_str(pre);

    let mut diff = cap - string_buff.bytes().count();
    while diff > 0 {
        string_buff.push(' ');
        diff -= 1;
    }

    (string_buff, leading.to_string())
}

fn standard_format_output(pre: &str, leading: &str) -> (String, String) {
    let mut str_buff = String::new();
    str_buff.push_str(&format!("{}\n", &pre));

    (str_buff, format!("\t{}\n", leading))
}

fn option_iterator(flags: &[Flag], ptrn: &Pattern) -> Vec<(String, String)> {
    let mut vals = vec![];
    for opt in flags {
        let mut params = String::new();
        for v in &opt.params {
            params.push_str(v.literal.as_str());
            params.push(' ');
        }

        let value;

        match &ptrn {
            Pattern::Custom(_syn) => {
                value = format!("{}, {} {}", opt.short, opt.long, params.trim())
            }
            _ => value = format!("{}, {} {}", opt.short, opt.long, params.trim()),
        }

        vals.push((value, opt.docstring.clone()));
    }

    vals
}

fn command_iterator(cmds: &[Cmd], ptrn: &Pattern) -> Vec<(String, String)> {
    let mut vals = vec![];

    for cmd in cmds {
        let mut params = String::new();
        for a in &cmd.params {
            params.push_str(a.literal.as_str());
            params.push(' ');
        }

        let value;
        match &ptrn {
            &Pattern::Legacy => value = format!("{} | {}", cmd.name, cmd.alias),
            &Pattern::Standard => value = format!("{} | {}, {}", cmd.name, cmd.alias, params),
            &Pattern::Custom(_syn) => value = format!("{} | {}", cmd.name, cmd.alias),
        }

        vals.push((value, cmd.description.clone()))
    }

    vals
}