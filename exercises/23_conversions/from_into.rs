#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 1. Split on commas
        let parts: Vec<&str> = s.split(',').collect();

        // 2. Must have exactly 2 parts
        if parts.len() != 2 {
            return Person::default();
        }

        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // 3. Name must not be empty
        if name.is_empty() {
            return Person::default();
        }

        // 4. Parse age
        match age_str.parse::<u8>() {
            Ok(age) => Person {
                name: name.to_string(),
                age,
            },
            Err(_) => Person::default(),
        }
    }
}

fn main() {
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}
