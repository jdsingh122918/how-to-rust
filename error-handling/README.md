# Introduction

`Rust` groups error into two main categories: `recoverable` and `unrecoverable`. An example of recoverable error is file not found error when searching for a file in the file system and being unable to find it. An example of unrecoverable error is trying to access a location beyond the end of the array. Unlike most language which handles this using `exceptions`, Rust does not have the concept of `exceptions`. Instead, it has the type `Result<T, E>` for recoverable errors and `!panic` macro for that stops the execution when the program encounters an unrecoverable error.
