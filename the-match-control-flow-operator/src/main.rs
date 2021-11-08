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

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        // we don't want to run any code in this case
        _ => ()
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
// fn move_player(num_spaces: u8) {}
// fn reroll() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // matches are exhaustive, so matching None is very necessary
        None => {
            println!("The x is None!");
            None
        },
        Some(i) => {
            println!("The x is {}, after plus one is {}", i, i + 1);
            Some(i + 1)
        }
    }
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
