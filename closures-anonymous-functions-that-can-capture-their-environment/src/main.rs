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
        // match self.value {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
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
    // Closures can capture values from their environment in three ways
    // 1. `FnOnce`: taking ownership
    // 2. `FnMut` : borrowing mutably
    // 3. `Fn`    : borrowing immutably
    // when creating a environment
    // Rust infers which trait to use based on
    // how the closure uses the values from the environment
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    // error:
    // functions are never allowed to capture their environment
    // defining and using functions will never incur this overhead
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // error:
    // x value moved into closure
    // The closure then has ownership of x
    // and `main` isn’t allowed to use x anymore in the println!
    // the traits implemented by a closure type are determined by what the closure does with captured values,
    // not how it captures them
    // `move` closures may still implement `Fn` or `FnMut`
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
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
