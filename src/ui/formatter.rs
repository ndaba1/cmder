use std::io::Write;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::parser::{Cmd, Flag};

pub struct Formatter {
    buffer: Buffer,
    #[allow(unused)]
    writer: BufferWriter,
}

pub enum FormatterRules {
    Option(Pattern),
    Cmd(Pattern),
}

pub enum Pattern {
    Legacy,
    Standard,
    Custom(String),
}

pub enum Designation {
    Headline,
    Description,
    Warning,
    Error,
    Other,
    Keyword,
    Special,
}

impl Formatter {
    pub fn new() -> Self {
        let bfwrt = BufferWriter::stderr(ColorChoice::Always);
        let buffer = bfwrt.buffer();
        Self {
            writer: bfwrt,
            buffer,
        }
    }

    pub fn add(&mut self, designation: Designation, value: &String) {
        use Designation::*;

        let color = match designation {
            Headline => Color::Cyan,
            Description => Color::White,
            Warning => Color::Yellow,
            Error => Color::Red,
            Other => Color::White,
            Keyword => Color::Yellow,
            Special => Color::Green,
        };

        let temp_writer = BufferWriter::stderr(ColorChoice::Always);
        let mut temp_buff = temp_writer.buffer();

        let og_buff = &mut self.buffer;

        temp_buff
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        write!(&mut temp_buff, "{}", value).unwrap();

        og_buff.write(temp_buff.as_slice()).unwrap();
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
                let cmds = cmds.unwrap();

                for (value, docstring) in command_iterator(&cmds, &ptrn) {
                    let val;
                    match ptrn {
                        Pattern::Legacy => {
                            let mut default_buf_size = 0;
                            for (val, _) in command_iterator(&cmds, &ptrn) {
                                if val.capacity() > default_buf_size {
                                    // add some padding
                                    default_buf_size = val.capacity() + 5;
                                }
                            }

                            val = legacy_format_output(&value, &docstring, default_buf_size);
                        }
                        Pattern::Standard => (val = standard_format_output(&value, &docstring)),
                        Pattern::Custom(ref _str) => {
                            val = standard_format_output(&value, &docstring)
                        }
                    }
                    self.add(Designation::Keyword, &format!("\t{}", val.0));
                    self.add(Designation::Description, &format!("{}\n", val.1));
                }
            }
            FormatterRules::Option(ptrn) => {
                let flags = flags.unwrap();

                for (value, docstring) in option_iterator(&flags, &ptrn) {
                    let val;
                    match ptrn {
                        Pattern::Legacy => {
                            let mut default_buf_size = 0;
                            for (value, _) in option_iterator(&flags, &ptrn) {
                                if value.capacity() > default_buf_size {
                                    // add some padding
                                    default_buf_size = value.capacity() + 5;
                                }
                            }

                            val = legacy_format_output(&value, &docstring, default_buf_size);
                        }
                        Pattern::Standard => (val = standard_format_output(&value, &docstring)),
                        Pattern::Custom(ref _str) => {
                            val = standard_format_output(&value, &docstring)
                        }
                    }
                    self.add(Designation::Keyword, &format!("\t{}", val.0));
                    self.add(Designation::Description, &format!("{}\n", val.1));
                }
            }
        }
    }

    pub fn print(&mut self) {
        self.writer.print(&self.buffer).unwrap();
        WriteColor::reset(&mut self.buffer).unwrap();
    }
}

fn legacy_format_output(pre: &String, leading: &String, cap: usize) -> (String, String) {
    let mut string_buff = String::with_capacity(cap);
    string_buff.push_str(&pre);

    let mut diff = &cap - string_buff.bytes().count();
    while diff > 0 {
        string_buff.push(' ');
        diff = diff - 1;
    }

    (string_buff, leading.clone())
}

fn standard_format_output(pre: &String, leading: &String) -> (String, String) {
    let mut str_buff = String::new();
    str_buff.push_str(&format!("{}\n", &pre));

    (str_buff, format!("\t{}\n", leading.clone()))
}

fn option_iterator(flags: &Vec<Flag>, _ptrn: &Pattern) -> Vec<(String, String)> {
    let mut vals = vec![];
    for opt in flags {
        let mut params = String::new();
        for v in &opt.params {
            params.push_str(v.literal.as_str());
            params.push(' ');
        }
        let value = format!("{}, {} {}", opt.short, opt.long, params.trim());
        vals.push((value, opt.docstring.clone()));
    }

    vals
}

fn command_iterator(cmds: &Vec<Cmd>, _ptrn: &Pattern) -> Vec<(String, String)> {
    let mut vals = vec![];

    for cmd in cmds {
        let mut params = String::new();
        for a in &cmd.params {
            params.push_str(a.literal.as_str());
            params.push(' ');
        }
        let value = format!("{} | {}, {}", cmd.name, cmd.alias, params);
        vals.push((value, cmd.description.clone()))
    }

    vals
}
