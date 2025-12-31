use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// Iterator-based implementation
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values()
        .filter(|&&progress| progress == value)
        .count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

// Iterator-based implementation for collection of maps
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection
        .iter()
        .flat_map(|map| map.values()) // flatten all values from all maps
        .filter(|&&progress| progress == value)
        .count()
}

fn main() {
    // You can optionally experiment here.
}
