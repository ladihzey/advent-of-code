use std::fs;
use rock_paper_scissors::Game;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let game_score = Game::new(&input).calculate_game_score();

    println!("The game score is {game_score}");
}
