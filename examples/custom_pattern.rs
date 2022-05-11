use cmder::{CustomPattern, Pattern, Program, Setting};

fn main() {
    let mut program = Program::new();

    program
        .author("vndaba")
        .description("An example using a custom pattern")
        .version("0.1.0");

    program
        .argument("<text...>", "Some dummy text")
        .action(|m| {
            dbg!(m);
        });

    let custom_ptrn = CustomPattern::new()
        .args_fmter("{{name}}: {{description}}")
        .flags_fmter("{{short}} or {{long}}:")
        .options_fmter("{{short}} or {{long}}: {{args}}")
        .sub_cmds_fmter("({{name}} | {{alias}})")
        .prettify(true);

    program.set(Setting::SetProgramPattern(Pattern::Custom(custom_ptrn)));

    program.parse();
}
