---
minutes: 10
---

# Exercise: Rewriting with Result

Use the try operator (?) to simplify the error handling in this code:

use std::fs;
use std::io::{self, Read};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}

