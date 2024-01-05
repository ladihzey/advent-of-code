use std::fs;
use day_02::parser::*;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let result = find_minimum_game_configs(&input);

    println!("The sum of the minimum game configs is {result}");
}

fn find_minimum_game_configs(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let g = game(line.trim()).unwrap().1;
            let gc = g.get_minimum_game_config();

            gc.red * gc.blue * gc.green
        })
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
    fn should_find_minimum_game_configs() {
        assert_eq!(find_minimum_game_configs(INPUT), 2286)
    }
}
