use std::fs;
use rucksack_reorganization::{
    calculate_dupe_items_priority,
    calculate_badges_priority
};

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    let priority = calculate_dupe_items_priority(&input);
    let badges_priority = calculate_badges_priority(&input);

    println!("Total dupes priority is {priority}");
    println!("Total badges priority is {badges_priority}");
}
