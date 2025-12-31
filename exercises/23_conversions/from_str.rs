// This is similar to the previous `from_into` exercise. But this time, we'll
// implement `FromStr` and return errors instead of falling back to a default
// value. Additionally, upon implementing `FromStr`, you can use the `parse`
// method on strings to generate an object of the implementor type. You can read
// more about it in the documentation:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1. Split on commas
        let parts: Vec<&str> = s.split(',').collect();

        // 2. Must have exactly 2 parts
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 3. Name must not be empty
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 4. Parse age
        match age_str.parse::<u8>() {
            Ok(age) => Ok(Person {
                name: name.to_string(),
                age,
            }),
            Err(e) => Err(ParsePersonError::ParseInt(e)),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}
