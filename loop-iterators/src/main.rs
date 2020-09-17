fn main() {
    let counter_loop = 0;
    let counter_while = 3;
    let counter_array = 5;

    let result_loop = loop_example(counter_loop);
    let result_while = while_example(counter_while);

    println!("Result from loop ... {}", result_loop);
    println!("Result from while ... {}", result_while);
    array_example(counter_array);
}

fn loop_example(mut counter: i32) -> i32 {
    let result = loop {
        counter += 1;
        // notice that there is no semi-colon after if since it is an expression
        if counter == 10 {
            break counter * 2;
        }
    };
    result
}

fn while_example(mut number: i32) -> i32 {
    while number != 0 {
        println!("And .. {}", number);
        number -= 1;
    }
    number
}

fn array_example(length: i32) {
    for number in (0..length).rev() {
        println!("Inside the for loop. Get ready ...{}", { number });
    }
    println!("LIFTOFF !!");
}
