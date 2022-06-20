# A simple, lightweight and extensible command line argument parser for rust codebases.

<p align="center" > 
<img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/ndaba1/cmder/cmder-ci-workflow">
<img alt="docs.rs" src="https://img.shields.io/docsrs/cmder">
<img alt="Crates.io" src="https://img.shields.io/crates/d/cmder">
<img alt="Crates.io" src="https://img.shields.io/crates/v/cmder">
</p>

This crate aims to provide you with an easy-to-use and extensible API without compromising the speed and performance that is expected of rust codebases. The syntax of the builder interface can be attributed to the javascript package `commander-js`. To get started, create a new instance of the program and manipulate it as shown.

## A basic example

The following is a basic example on how to use the crate. More in depth documentation can be found from docs.rs or the github repo.

```rust

let mut program = Program::new();

program
    .version("0.1.0")
    .description("An example CLI")
    .author("Author's name");

program
    .subcommand("test")
    .argument("<app-name>", "Pass the name of the app to test")
    .alias("t")
    .description("A test command")
    .option("-s --skip", "Skip checking/installing the dependencies")
    .option("-p --priority", "The priority to use when testing apps")
    .action(|matches| { dbg!(matches); });

// ...

program.parse();

```

## Extending the functionality using event listeners

The default behaviour of the program can be easily extended or even overriden by use of event listeners. View the docs to see all possible events of the program.

```rust
//...

program.on(Event::OutputVersion, |config|{
    let prog_ref = config.get_program();
    println!("Current program version is: {}", prog_ref.get_version());
});

program.after_help(|config|{
    let prog_ref = config.get_program();
    println!("This program was authored by: {}", prog_ref.get_author());
});

program.before_all(|config|{
    println!("An Aram Mojtabai banger!!!😂")
});

// ...
```

## Configuring program settings

Modify the settings to control the behavior of the program. See the documentation for all possible configurable settings

```rust
// ...

program.set(Setting::ShowHelpOnAllErrors(true));
program.set(Setting::ChoosePredefinedTheme(PredefinedThemes::Colorful));
program.set(Setting::SetProgramPattern(Pattern::Legacy));
program.set(Setting::OverrideAllDefaultListeners(true));

// ...
```

## Customizing themes and patterns

You can define your own color pallete to be used when printing to std_out. Also, you can control how the information is printed by use of patterns.

```rust
// ...

use Color::*;
program.set(Setting::DefineCustomTheme(construct_theme!(
    Green, Magenta, Blue, Red, White
)));


// ...
```

Refer to docs.rs for full documentation on the crate. Also check out the repository on github for examples of crate usage [here](https://github.com/ndaba1/cmder/tree/main/examples). If you found this crate useful, consider [starring this repo](https://github.com/ndaba1/cmder/stargazers).

## Contributing

Whether you have an awesome color palette to create a new predefined theme, or you wish to add a new use-case to the [examples/](examples) directory, or you have a change that can improve the crate, all contributions are welcome and highly appreciated!
