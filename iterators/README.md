### Iterators
The iterator pattern allows you to perform a certain task on series of elements in a sequence. It handles the 
responsibility of iterating over the elements and determining when the sequence has ended, hence making it
easier for developers to stick to business logic.

The `iterators` in `Rust` are `lazy loading`. That means, if they are declared, they themselves in isolation 
doesn't do much. For example
```rust
let v1 = vec![1, 2, 3]
let v1_iter = v1.iter()
```
`v1_ter` by itself doesn't do anything useful until called. For example, one possible of continuing the above
code is
```rust
for val in v1_iter {
    println!("Got value: {}", val);
}
```
`iter()` method returns an `iterator` over `immutable` references. To create an iterator that takes ownership,
`iter_into()` method is used to return the owned values as well. To iterate over `mutable` reference, `iter_mut()`
is used instead of `iter`.