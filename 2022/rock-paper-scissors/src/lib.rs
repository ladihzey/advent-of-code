use std::cmp::{Ord, Ordering};
use Ordering::{Less, Greater, Equal};
use HandShape::{Rock, Paper, Scissors};

#[derive(PartialEq, Eq, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn get_score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play(&self, other: &Self) -> u32 {
        match self.cmp(other) {
            Greater => 6,
            Equal => 3,
            Less => 0,
        }
    }
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Rock, Paper) => Less,
            (Rock, Scissors) => Greater,
            (Paper, Rock) => Greater,
            (Paper, Scissors) => Less,
            (Scissors, Rock) => Less,
            (Scissors, Paper) => Greater,
            _ => Equal
        }
    }
}

pub struct GameRound(HandShape, HandShape);

impl GameRound {
    pub fn parse_by_guess(code_pair: &str) -> Self {
        let shapes = code_pair
            .split(' ')
            .map(|code| match code {
                "A" | "X" => Rock,
                "B" | "Y" => Paper,
                "C" | "Z" => Scissors,
                _ => panic!("Unknown hand shape code {code}")
            })
            .collect::<Vec<HandShape>>();

        Self(shapes[0].clone(), shapes[1].clone())
    }

    pub fn parse_correctly(code_pair: &str) -> Self {
        match code_pair {
            "A X" => Self(Rock, Scissors),
            "A Y" => Self(Rock, Rock),
            "A Z" => Self(Rock, Paper),
            "B X" => Self(Paper, Rock),
            "B Y" => Self(Paper, Paper),
            "B Z" => Self(Paper, Scissors),
            "C X" => Self(Scissors, Paper),
            "C Y" => Self(Scissors, Scissors),
            "C Z" => Self(Scissors, Rock),
            _ => panic!("Unknown hand shape code pair {code_pair}")
        }
    }

    fn calculate_round_score(&self) -> u32 {
        let GameRound(first, second) = self;
        second.get_score() + second.play(first)
    }
}

pub struct Game {
    rounds: Vec<GameRound>,
}

impl Game {
    pub fn new(round_codes: &str, parse: &dyn Fn(&str) -> GameRound) -> Self {
        let rounds = round_codes
            .lines()
            .map(parse)
            .collect();
        
        Self { rounds }
    }

    pub fn calculate_game_score(&self) -> u32 {
        self.rounds
            .iter()
            .map(|round| round.calculate_round_score())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        A Y\n\
        B X\n\
        C Z\
    ";

    #[test]
    fn calculate_game_score_by_guess_strategy() {
        let game = Game::new(INPUT, &GameRound::parse_by_guess);
        assert_eq!(game.calculate_game_score(), 15);
    }

    #[test]
    fn calculate_game_score_by_correct_strategy() {
        let game = Game::new(INPUT, &GameRound::parse_correctly);
        assert_eq!(game.calculate_game_score(), 12);
    }
}
