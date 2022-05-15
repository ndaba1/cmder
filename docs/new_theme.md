## Creating a new predefined theme for the crate:

The colors output to the terminal when printing help information are controlled by the program theme. All theme functionality is housed in the [ui](../src/ui/mod.rs) module. To allow for creation of custom themes, colors are not configured directly. This is done by use of designations, i.e. A healine will have this color, an error another color: 'headline' and 'error' are the designations.

The full list of designations is:

- Headline
- Keyword
- Description
- Error
- Other
  This functionality is declared in the [formatter](../src/ui/formatter.rs) module which is used to print to stdout.

A new theme struct will have fields corresponding to each of the designation. To create a new theme, the fields must be instantiated with a color from the `termcolor` crate as so:

```rust
use Color::*;

let theme = Theme {
    keyword: Green,
    headline: Magenta,
    description: Blue,
    error: Red,
    other: White,
};

```

The colors are re-exported from `termcolor`. You could also use the `construct_theme!()` macro which is easier to use.
