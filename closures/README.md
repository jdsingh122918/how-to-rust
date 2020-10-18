## Use case for `closures`
Let's say we have an expensive computation method, which is called multiple times
in the application. We want to define that expensive computation method in one place
in our program, but only `execute` the code, when we actually need the result, instead
of duplicating the code.

`closures` can capture the environment variables within the scope, unlike `functions`.
The three `traits` that can be implemented by `closures` are:
* `FnOnce`: Captures the environmental variables and takes ownership when called.
* `FnMut`: Same as above and only borrow it mutably.
* `Fn`: Same as above two and borrows immutably.

`move` keyword is used if one needs to take ownership of the environmental variable.
