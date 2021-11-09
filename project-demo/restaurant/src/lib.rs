use std::io::{self, Write};

fn serve_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // only pub the enum, so the var in enum can be public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

// use crate::front_of_house::hosting;
// hosting now is public
// so external code could take advantage of this new path
pub use self::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

use std::collections::HashMap;

// use Result but in deferent parent modules
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // use 'use' keyword
    hosting::add_to_waitlist();

    add_to_waitlist();

    // create a Breakfast type meal
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // access meal toast var
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // it will be error because seasonal_fruit is a private var in Breakfast
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}
