// Write a function that takes in a string and returns the first word it finds
// If the function doesn't find a space in the string, the entire word must be returned.
fn main() {
    let word = String::from("Hello World");
    let result = first_word(&word);
    println!("And the result is {}", result);
}

// &str -> signifies the type of string slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
