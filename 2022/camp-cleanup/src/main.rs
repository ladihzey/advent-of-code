use std::fs;

use camp_cleanup::{count_duplicating_assignments, count_overlapping_assignments};

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    let duplicating_count = count_duplicating_assignments(&input);
    let overlapping_count = count_overlapping_assignments(&input);

    println!("The count of duplicating assignments is {duplicating_count}");
    println!("The count of overlapping assignments is {overlapping_count}");
}
