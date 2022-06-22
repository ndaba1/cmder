#![allow(unused)]
use std::io::Write;
use termcolor::{Buffer, BufferWriter, ColorChoice, ColorSpec, WriteColor};

use crate::{parse::Argument, Theme};

pub struct Formatter {
    buffer: Buffer,
    writer: BufferWriter,
    theme: Theme,
    glob_pattern: Pattern,
    padding: usize,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Legacy,
    Standard,
    Custom(CustomPattern),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Designation {
    Headline,
    Description,
    Error,
    Other,
    Keyword,
}

pub trait FormatGenerator {
    fn generate(&self, ptrn: Pattern) -> (String, String);
}

#[derive(Debug, Clone)]
pub struct CustomPattern {
    pub args_fmter: String,     // {{literal}} {{name}} {{description}}
    pub flags_fmter: String,    // {{short}} {{long}}
    pub options_fmter: String,  // {{short}} {{long}} {{args}}
    pub sub_cmds_fmter: String, // {{name}} {{alias}} {{args}} {{description}}
    pub prettify_as_legacy: bool,
}

impl Default for CustomPattern {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomPattern {
    pub fn new() -> Self {
        Self {
            args_fmter: "{{name}}".into(),
            flags_fmter: "{{short}}, {{long}}".into(),
            options_fmter: "{{short}}, {{long}} {{args}}".into(),
            sub_cmds_fmter: "{{name}}".into(),
            prettify_as_legacy: true,
        }
    }

    pub fn args_fmter(mut self, val: &str) -> Self {
        self.args_fmter = val.into();
        self
    }

    pub fn flags_fmter(mut self, val: &str) -> Self {
        self.flags_fmter = val.into();
        self
    }

    pub fn options_fmter(mut self, val: &str) -> Self {
        self.options_fmter = val.into();
        self
    }

    pub fn sub_cmds_fmter(mut self, val: &str) -> Self {
        self.sub_cmds_fmter = val.into();
        self
    }

    pub fn prettify(mut self, val: bool) -> Self {
        self.prettify_as_legacy = val;
        self
    }
}

impl Formatter {
    pub fn new(theme: Theme) -> Self {
        let writer = BufferWriter::stderr(ColorChoice::Always);
        let buffer = writer.buffer();
        Self {
            buffer,
            writer,
            theme,
            padding: 10,
            glob_pattern: Pattern::Legacy,
        }
    }

    pub fn add(&mut self, designation: Designation, value: &str) {
        use Designation::*;

        let color = self.theme.get(designation);

        // Store ref to main buffer
        let main_buff = &mut self.buffer;

        // Create temporary buffer and buff_writer
        let temp_writer = BufferWriter::stderr(ColorChoice::Always);
        let mut temp_buff = temp_writer.buffer();

        // Use desired colors to write to the temporary buffer
        temp_buff
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        write!(&mut temp_buff, "{}", value).unwrap();

        // Copy contents of temporary buffer to main buffer
        main_buff.write_all(temp_buff.as_slice()).unwrap();

        // Reset both buffers
        WriteColor::reset(&mut temp_buff).unwrap();
        WriteColor::reset(main_buff).unwrap();
    }

    pub fn print(&mut self) {
        self.writer.print(&self.buffer).unwrap();
        WriteColor::reset(&mut self.buffer).unwrap();
    }

    pub fn section(&mut self, value: &str) {
        self.add(Designation::Headline, &format!("\n{}:\n", value));
    }

    pub fn close(&mut self) {
        self.add(Designation::Other, "\n");
    }

    pub fn format<'a, L, T: 'a>(&mut self, args: L, ptrn: &Pattern)
    where
        L: IntoIterator<Item = &'a T>,
        T: FormatGenerator,
    {
        let mut values = vec![];

        for item in args.into_iter() {
            let v = item.generate(ptrn.clone());
            values.push(v);
        }

        for (leading, floating) in values.iter() {
            match &ptrn {
                Pattern::Legacy => {
                    for (v, _) in values.iter() {
                        if v.capacity() > self.padding {
                            self.padding = v.capacity() + 5;
                        }
                    }

                    self.legacy_format_output(leading, floating);
                }
                Pattern::Standard => self.standard_format_output(leading, floating),
                Pattern::Custom(ptrn) => self.custom_format_output(ptrn, leading, floating),
            }
        }
    }

    fn legacy_format_output(&mut self, leading: &str, floating: &str) {
        let cap = self.padding;
        let mut string_buff = String::with_capacity(cap);
        string_buff.push_str(leading);

        let mut diff = if cap > string_buff.bytes().count() {
            cap - string_buff.bytes().count()
        } else {
            self.padding = string_buff.bytes().count() + 5;
            5
        };

        while diff > 0 {
            string_buff.push(' ');
            diff -= 1;
        }

        self.add(Designation::Keyword, &format!("   {}", string_buff));
        self.add(Designation::Description, &format!("{}\n", floating))
    }

    fn standard_format_output(&mut self, leading: &str, floating: &str) {
        self.add(Designation::Keyword, &format!("    {}\n", leading));
        self.add(Designation::Description, &format!("      {}\n", floating));
    }

    fn custom_format_output(&mut self, ptrn: &CustomPattern, leading: &str, floating: &str) {
        if ptrn.prettify_as_legacy {
            self.legacy_format_output(leading, floating);
        } else {
            self.add(Designation::Keyword, leading);

            if floating.is_empty() {
                self.add(Designation::Other, "\n");
            } else {
                self.add(Designation::Other, &format!("{}\n", floating));
            }
        }
    }
}
