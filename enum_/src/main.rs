fn main() {
    println!("Hello, world!");
}

enum Coin {
    Sathish,
    Shakthi,
    Sakara,
    Srihari,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Sathish => 1,
        Coin::Shakthi => 5,
        Coin::Sakara => 10,
        Coin::Srihari => 25,
    }
}


