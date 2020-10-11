## Use case for `closures`
Let's say we have an expensive computation method, which is called multiple times
in the application. We want to define that expensive computation method in one place
in our program, but only `execute` the code, when we actually need the result, instead
of duplicating the code.