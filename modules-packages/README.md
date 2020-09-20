# Introductions

As the programs and complexity of the applications grow, it is advisable to separate the code into domains or split it on based of desired functionalities. One can potentially organize the code into multiple `modules` and then further into multiple `files`. A `package` can contain multiple `binary crates` and `optionally one library crate`.

# Module System

- `Packages`: A `cargo` feature that let you `build`, `test` and `share` crates.
- `Crates`: A tree of modules that produces a `library` or `executable`.
- `Modules` and `use`: Let one control the `organization`, `scope` and `privacy` of the `paths`.
- `Paths`: A way of naming an item, such as `struct`, `function`, or `module`.

# Notes

- Default for `enums` is `public`. However for `structs` it is private.
