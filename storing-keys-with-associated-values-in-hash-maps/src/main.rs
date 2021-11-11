use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // the hash map will only contain one key/value pair
    // because we’re inserting the value for the Red team’s key both times
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Red"), 60);

    // Yellow key doesn't exist in scores HashMap
    // so the value of Yellow key is 70
    scores.entry(String::from("Yellow")).or_insert(70);
    scores.entry(String::from("Red")).or_insert(0);

    // Red is 60
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let init_scores = vec![10, 50];
    let mut _scores2: HashMap<_, _>
        = teams
            .into_iter()
            // zip create an iterator of tuples
            // ("Blue", 10)
            // ("Red", 50)
            .zip(init_scores.into_iter())
            .collect();

    let field_name = String::from("Favor color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // error:
    // field_name and field_value are invalid at this point
    // HashMap take ownership of them both
    // because they are all String
    // println!("name: {}, value: {}", field_name, field_value);

    let team_name = String::from("Blue");
    // score is Some(&10)
    // get returns Option<&V>
    let score = scores.get(&team_name);
    match score {
        Some(i) => println!("score is {}", i),
        _ => ()
    }

    for (k, v) in scores {
        println!("{}:{}", k, v);
    }

}
