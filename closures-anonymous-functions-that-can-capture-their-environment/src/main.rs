use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

struct Cacher<T, K, V>
//  it’s a closure by using the Fn trait
where
    T: Fn(&K) -> V,
    K: Eq + Hash
{
    calculation: T,
    // value: Option<u32>,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(&K) -> V,
    K: Eq + Hash
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            // value: None,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        use std::collections::hash_map::Entry;

        // the first time we called c.value with 1,
        // the Cacher instance saved Some(1) in self.value.
        match self.value.entry(arg) {
            // if the value of arg key exist
            Entry::Occupied(entry) => entry.into_mut(),
            // if the value of arg key does not exist
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
                entry.insert(v)
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    // error:
    // Those types are then locked into the closure in example_closure
    // let n = example_closure(5);
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // To define a closure, we start with a pair of vertical pipes (|)
    // Smalltalk and Ruby has the same syntax
    // more parameter should be like `|param1, param2|`
    // hey’re stored in variables and used without naming them
    // and exposing them to users of our library.

// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

    // num is a reference
    // return type is not a reference
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        *num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        // a is a reference
        // return type is not a reference
        let mut c = Cacher::new(|a| *a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(*v2, 2);
    }

    #[test]
    fn value_can_differ_return_type_from_parameter_type() {
        let mut c = Cacher::new(|_a| String::from("hello"));

        let v = c.value(5);
        assert_eq!(*v, "hello");
    }
}
