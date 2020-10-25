use add_one;
use add_two;

fn main() {
    let num = 10;
    let num_2 = 12;
    println!(
        "Hello, world! {} plus 1 is {}",
        num,
        add_one::add_one(num)
    );
    println!(
        "{} plus {} is {}",
        num,
        num_2,
        add_two::add_two(num, num_2)
    );
}
