### `smart pointers`
_SMART POINTERS_ are _pointers_ that have additional metadata and capabilities than the normal _pointers_.
For example `String` and `Vec<T>`, because they both can own some memory and allow manipulations on them.

**Smart Pointers** are usually implemented using `structs`. They also have to implement `Dref` and `Drop` 
`traits`. `Dref` allows an instance of smart pointer to behave like a reference so one can write code that
works with either references or smart pointers. The `Drop` trait allows one to customize the code that is
run when the instance goes out of scope.

Most common _smart pointers_ in the `standard library` are:
* `Box<T>` for allocating values on the heap
* `Rc<T>`,  a reference counting type that enable multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at 
_runtime_ instead of _compile time_.

### `Interior Mutability`
It is a design pattern in Rust that allows one to mutate data even when there are immutable references
to that object. Normally, this action is prohibited by the compiler. One can bypass the compiler by using
types like `RefCell<T>` which follows the described pattern. 

`Cell<T>` and `Mutex<T>` are other types in the `standard library` that follows the above pattern. 