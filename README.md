## A simple, lightweight and extensible command line argument parser for rust codebases.

<p align="center" > 
<img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/ndaba1/cmder/cmder-ci-workflow">
<img alt="Crates.io" src="https://img.shields.io/crates/d/cmder">
<img alt="Crates.io" src="https://img.shields.io/crates/v/cmder">
</p>

Add this to your Cargo.toml
```rust
[dependencies]
cmder="0.1.0"
```

This crate is fairly similar to the javascript package `commander-js`. To get started, create an instance of the program struct and use it to add commands. The following is an example:

```rust

let mut program = Program::new();

    program
        .version("0.1.0")
        .description("An example CLI")
        .author("Author's name");

    program
        .add_cmd()
        .command("test <app-name>")
        .alias("t")
        .describe("A test command")
        .option("-s --skip", "Skip checking/installing the dependencies")
        .option("-p --priority", "The priority to use when testing apps")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

```

You can also override the default behavior of the program. You can edit the Themes and how information is printed out to stdout as follows:

```rust
program.on(Event::OutputVersion, |p, v| {
        println!("You are using version {} of my program", v);
        println!("This program was authored by: {}", p.author);
    });
```

Refer to docs.rs for full documentation on the crate.