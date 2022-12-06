use std::fs;
use rock_paper_scissors::{Game, GameRound};

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();

    println!(
        "The game score with guess strategy is {}",
        Game::new(&input, &GameRound::parse_by_guess).calculate_game_score()
    );

    println!(
        "The game with correct strategy is {}",
        Game::new(&input, &GameRound::parse_correctly).calculate_game_score()
    );
}
