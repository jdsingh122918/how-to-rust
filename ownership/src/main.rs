fn main() {
    // creating a string 'from' method
    let mut from_string = String::from("hello");
    from_string.push_str(", world");
    println!("{}", from_string);

    let string_one = String::from("Jodhandeep Singh");
    let string_two = string_one.clone();

    println!("{}", string_one);
    println!("{}", string_two);
}
