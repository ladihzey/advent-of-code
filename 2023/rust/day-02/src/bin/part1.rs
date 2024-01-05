use std::fs;
use day_02::game::*;
use day_02::parser::*;

const THRESHOLD_ROUND: GameConfig = GameConfig {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let result = find_possible_games(&input);

    println!("The sum of the possible games ids is {result}");
}

fn find_possible_games(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| game(line.trim()).unwrap().1)
        .filter(|g| g.is_possible(&THRESHOLD_ROUND))
        .map(|g| g.id)
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    #[test]
    fn should_find_impossible_games() {
        assert_eq!(find_possible_games(INPUT), 8);
    }
}
