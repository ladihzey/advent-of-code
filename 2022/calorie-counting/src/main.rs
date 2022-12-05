use std::fs;

use calorie_counting::CalorieCounter;

const ELVES_FOOD_LIST: &str = "assets/elves-food.txt";

fn main() {
    let input = fs::read_to_string(ELVES_FOOD_LIST).unwrap();
    let calorie_counter = CalorieCounter::new(&input);

    println!("Max calories elf load: {}", calorie_counter.find_max_calories_elf_load());
    println!("Top elf load calories: {}", calorie_counter.sum_top_elf_load_calories(3));
}
