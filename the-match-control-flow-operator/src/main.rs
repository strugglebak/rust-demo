#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("penny: {}", value_in_cents(Coin::Penny));
    println!("nickel: {}", value_in_cents(Coin::Nickel));
    println!("quarter: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("quarter: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        },
    }
}
