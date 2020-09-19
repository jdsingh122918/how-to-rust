enum Coin {
    Dime,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 1,
    }
}

fn main() {
    let test_coin = Coin::Dime;
    println!("The value of coin is: {}", value_in_cents(test_coin));
}
