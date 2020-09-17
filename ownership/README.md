# Introduction

Ownership is one of the central concepts begin Rust programming language. It is what enables Rust to NOT rely on any sort of garbage collector. Here is an exercpt from book:

"All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running."

Stack: Stores the data in the form of "last in first out" (LIFO). Example: Stack of plates in the kitchen. One would always take the plate from the top and never from the middle or bottom (unless they are trying to strengthen their muscle LOL!!). Any data stored in Stack should be known at the compile time. If it is unknown (like size and details of the object are not known), then that data is stored in the 'Heap'.

Heap: Data with unknown size is stored in the heap by the memory allocator. The allocator allocates part of the heap which is empty and mark it as use. It furthers returns the pointer to that location on the heap back!! The thing to note here is that the pointer actually stores the information about the object and it's location and is therefore of fixed size and information. This implies, pointer itself can be stored in the stack. Hence, to get the result of the object from the heap, we have to follow the pointer in the stack.

When we call a certain function in Rust (including parameters which can be either in stack or is a pointer pointing towards a location in the heap), all of the information about that function call is stored in the stack. When that function is no longer in the scope, the values of that function are popped off of the stack.

# Basic Rules of Ownership

- Each value in `Rust` has a variable called its, you guessed it right!!, `owner`
- Thy shall be `ONLY ONE OWNER` at a time. That means, any object cannot have more than one owner at a given moment.
- When the `owner` goes out of scope, it's value will be `DROPPED`.

# References and Borrowing

To borrow the variable instead of taking ownership, use this `&` symbol. Example `some_function(s: &String) -> usize`.
Reference scope starts from when it is introduced and continues through the last time that reference is used. At any given time, you can have either one `mutable` reference or any number of `immutable` references. References must always be valid.

# Notes

- Rust never makes `deep copies` of the data unless explicitly mentioned by using the `clone()` method.
- `Deep Copying` and `Shallow Copying` is the same for the `types` that `implement` the `Copy` trait.
- `Rust` won't let `annotate` a `type` with `Copy` trait, if that `type` or any of it's `parts` implement the `Drop` trait.
- Some `types` in the `Standard Library` that have implemented that `Copy` trait:
  - All integer types like `u32, i32`
  - Boolean type `bool` with values `true` and `false`
  - All floating point type like `f64`
  - Character type `char`
  - Tuples --> Only if their components have `Copy` trait
- We can use `{}` curly brackets, to introduce temporary scopes.
