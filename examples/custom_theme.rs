use cmder::{Color, Program, Theme};

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

    use Color::*;
    program.theme(Theme::new(Green, Magenta, Blue, Red, White));

    program.parse();
}
