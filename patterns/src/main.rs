fn main() {
    println!("Hello and welcome to the world of patterns");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8,_> = "34".parse();

    //if, else-if, if let, else if let demo
    if let Some(color) = favorite_color {
        println!("Favorite Color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is Green Day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Age greater than 30");
        } else {
            println!("Age less than 30");
        }
    } else {
        println!("End of the program");
    }

    // while let demo
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //for loop demo
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // function parameters sa patterns
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Location: ({}, {})", x, y);
}
