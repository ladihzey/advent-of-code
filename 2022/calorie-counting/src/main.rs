use std::fs;

use calorie_counting::CalorieCounter;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let calorie_counter = CalorieCounter::new(&input);

    println!("Max calories elf load: {}", calorie_counter.find_max_calories_elf_load());
    println!("Top elf load calories: {}", calorie_counter.sum_top_elf_load_calories(3));
}
