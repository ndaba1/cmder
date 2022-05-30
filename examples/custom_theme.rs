use cmder::{Color, Program, Setting, Theme};

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .author("vndaba")
        .description("Custom theme example");

    program
        .argument("<text...>", "Some dummy text")
        .action(|m| {
            dbg!(m);
        });

    // You can use the `construct_theme` macro and play around with the colors
    use Color::*;
    program.set(Setting::DefineCustomTheme(Theme::new(
        Green, Magenta, Blue, Red, White,
    )));

    // This method will achieve exactly the same outcome as the macro expression above

    program.parse();
}
