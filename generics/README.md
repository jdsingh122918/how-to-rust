## Introduction
`generics` are abstract stand-ins for concrete types to handle code duplication.
They along with `traits` are used to specify `function bounds`. That means they
place restrictions on the return types and on the types of parameters in the 
function.

## Traits
`traits` are like `interfaces` in `java`. In my opinion, they are a lot more
powerful than `java` `interfaces`. They can be used to conditionally implement 
functionality of a custom types. Concept of `blanket implementations` makes it 
really powerful to abstract away the common functionalities.

## Lifetimes
`lifetimes` are used by the Rust compiler to ensure that there are no dangling
pointers causing errors in memory allocation. It is one of the unique feature of
Rust which is not present in other languages. The compiler uses three rules to 
figure out what lifetimes references have when there aren't explicit annotations.
The first rule applies to `input` lifetimes, and the second and third rules apply to
`output` lifetimes. If the compiler gets to the end of the three rules and there are
still references for which it can’t figure out lifetimes, the compiler will stop
with an error. `'static` keyword is used to specify `lifetime` which can sustain 
the entire program as it is hard coded in the binary of the application. 

## Sample Example Problem
Write a function that will return the largest item given a list.