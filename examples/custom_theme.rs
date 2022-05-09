use cmder::{construct_theme, Color, Program, Theme};

fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .author("vndaba")
        .description("Custom theme example");

    program
        .argument("<text...>", "Some dummy text")
        .action(|values, options| {
            dbg!(values, options);
        });

    // You can use the `construct_theme` macro and play around with the colors
    use Color::*;
    program.set_custom_theme(construct_theme!(Green, Magenta, Blue, Red, White));

    // You could also construct the theme manually
    program.set_custom_theme(Theme {
        keyword: Green,
        headline: Magenta,
        description: Blue,
        error: Red,
        other: White,
    });
    // This method will achieve exactly the same outcome as the macro expression above

    program.parse();
}
