#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>), // ✅ wrap recursive type in Box
    Nil,
}

// Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// Create a non-empty cons list.
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
