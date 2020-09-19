# Introductions

`Enums` like `structs` are another tool in the toolbox to create custom types. Example of an `enum` which is used frequently in application development is `Option`. `Enums` value can only be one of its `variants`. `Enums` just like `structs` can use the `impl` to define `methods`.

# Option Enum

The `Option` type `enum` is used in many places because it encodes the very common scenario in which a value could be something or could be nothing. `RUST DOES NOT HAVE NULL FEATURE LIKE OTHER LANGUAGES`. `Option` enum is used to take care of `null` values.

# `match` Control Flow operator

`match` is used to compare values against a series of pattern and execute code against it. They are also exhaustive in nature. The compile will complain if we are accounting for all the options. The `_` placeholder is used for defualt cases when nothing else matches.
