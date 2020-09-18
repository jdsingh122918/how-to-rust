# Introduction

`Structs` are similar to `tuples` but unlike `tuples`, you'll name each piece of data in `structs`. As a result of this, order becomes a non-necessity in case of `structs`. To define a `struct` use the `struct` keyword. `Tuple structs` are `structs` without any `names`. To use `references` in `structs`, one has to use the concept of `lifetime`, otherwise the option is to use owned data types like `String`.

# Methods

`Methods` are similar to `fn` (functions) in the manner that they are both declared by the `fn` keyword, they can have parameters and can have a return value when called. But they differ from normal functions from the fact that they are used in the context of `structs` or either `traits` or `enum`.

# Associated Functions

Within `impl` block, we can also declare `functions` that are not `methods` in the sense that they don't have the instance of the `struct` to work with. Example `String::from` is an associated function.

# IMPORTANT

`Rust` offers by default debbuging information but that has to be opted in by appending `[derive(Debug)]` on top of the `structs` definition.
