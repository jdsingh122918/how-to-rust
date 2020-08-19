Guessing game. The applicatin is going to guess a certain number between 1 and 10. The player has to guess the number. If the number is too big or small, the application will prompt the user appropriately.

Conceptual Notes:

By default Rust only brings in few types into the scope of every program. If we want to bring anything else other than the default types, we have to use the `use` keyword. Example `use std::io` has been used to import the `io` (input/output) library in scope.

By default the variable are `immutable`. We have to use the `mut` keyword to make them `mutable`. `let mut guess = String::new()` has created a mutable variable that is currently bound to a new, empty instance of `String`.

`Crate` is a collection of rust source code files. `Crate` can be of the type `binary` or `library` like the `rand` crate.
