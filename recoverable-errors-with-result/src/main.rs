use std::fs::{self, File};
use std::io::{ErrorKind, self, Read};

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let _f1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _f2 = File::open("hello.txt").expect("Failed to open the hello.txt file");
    // Note:
    // all unwrap calls that panic print the same message
    // so use expect will be more concise
    let _f3 = File::open("hello.txt").unwrap();

    match read_content_from_file() {
        Ok(c) => println!("file content is {}", c),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn read_content_from_file() -> Result<String, io::Error> {
    // let mut f = File::open("hello.txt")?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // f.read_to_string(&mut s)?;

    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
    fs::read_to_string("hello.txt")
}
