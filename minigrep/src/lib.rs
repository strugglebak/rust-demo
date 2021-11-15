use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str>  {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
        // Remove-Item Env:CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait
// dyn means dynamic
// This gives us flexibility to return error values
// that may be of different types in different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
      search(&config.query, &contents)
    } else {
      search_case_insensitive(&config.query, &contents)
    };

    for line in results {
      println!("{}", line);
    }

    // using () like this is the idiomatic way to indicate
    // that we’re calling run for its side effects only
    // it doesn’t return a value we need
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
