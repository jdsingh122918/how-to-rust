fn main() {
    let vector_new = vec![1, 2, 3];
    println!("value of the vector is {:?}", vector_new);

    let third_element = &vector_new[2];
    println!("{}", third_element);
    match vector_new.get(4) {
        Some(third_element) => println!("Value is {}", third_element),
        None => println!("Element not found"),
    }

    for i in &vector_new {
        println!("{}", i);
    }

    let mut string_new = String::from("Jodhan");
    string_new.push_str("deep");
    println!("{}", string_new);

    let first_name = String::from("Misha Preet");
    let last_name = String::from("Kaur");
    let full_name = format!("{} {}", first_name, last_name);
    println!("{}", full_name);

    use std::collections::HashMap;

    let mut hashmap_new = HashMap::new();
    hashmap_new.insert(String::from("First Name"), String::from("Jodhandeep"));
    hashmap_new.insert(String::from("Last Name"), String::from("Singh"));

    for (key, value) in hashmap_new {
        println!("{}: {}", key, value);
    }
}
