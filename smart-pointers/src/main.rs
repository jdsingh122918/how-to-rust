use std::cell::RefCell;
use std::mem::drop;
use std::rc::Rc;

use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with the data {} ", self.data);
    }
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("a: {:?}", a);
    *value.borrow_mut() += 10;
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    let c = CustomSmartPointer {
        data: String::from("Hello JD Singh"),
    };
    drop(c);
    // This will error out during runtime `c` was dropped earlier
//    println!("{:?}", c);
}
