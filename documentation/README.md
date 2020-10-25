### Documentation
Apart from commenting the code using the `//`, one can document the code with the blocks
starting with triple forward slash `///`. Executing `cargo doc --open`, one can check the 
`html` documentation created from running the command.

Another way of documentation , `//!` adds documentation to the item that contains the comments
rather than adding documentation to the items following the comments. These are typically used 
inside the `crate root` file (`src/lib.rs` by convention) or inside the module to document the 
module as a whole.

Using `pub use` at the top of the modules re-imports the modules to be used easily by the developers
consuming the API.