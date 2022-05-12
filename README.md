## A simple, lightweight and extensible command line argument parser for rust codebases.

<p align="center" > 
<img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/ndaba1/cmder/cmder-ci-workflow">
<img alt="Crates.io" src="https://img.shields.io/crates/d/cmder">
<img alt="Crates.io" src="https://img.shields.io/crates/v/cmder">
</p>

This crate aims to provide you with an easy-to-use and extensible API without compromising the speed and performance that is expected of rust codebases. The syntax of the builder interface can be attributed to the javascript package `commander-js`

```rust

let mut program = Program::new();

program
    .version("0.1.0")
    .description("An example CLI")
    .author("Author's name");

program
    .command("test <app-name>")
    .alias("t")
    .description("A test command")
    .option("-s --skip", "Skip checking/installing the dependencies")
    .option("-p --priority", "The priority to use when testing apps")
    .action(|vals, opts| {
        dbg!(vals);
        dbg!(opts);
    })
    .build(&mut program);

program.parse();

```

You can also override the default behavior of the program. You can edit the Themes and how information is printed out to stdout as follows:

```rust
program.on(Event::OutputVersion, |p, v| {
        println!("You are using version {} of my program", v);
        println!("This program was authored by: {}", p.get_author();
    });
```

Refer to docs.rs for full documentation on the crate. Also check out the repository on github for examples of crate usage [here](https://github.com/ndaba1/cmder/tree/main/examples).

If you found this crate useful, consider [starring this repo](https://github.com/ndaba1/cmder/stargazers).

## Contributing

All contributions are welcome and highly appreciated!
