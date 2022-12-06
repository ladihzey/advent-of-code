use std::cmp::{Ord, Ordering};

#[derive(PartialEq, Eq, Clone)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn play(&self, other: &Self) -> u32 {
        match self.cmp(other) {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
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
            (HandShape::Rock, HandShape::Paper) => Ordering::Less,
            (HandShape::Rock, HandShape::Scissors) => Ordering::Greater,
            (HandShape::Paper, HandShape::Rock) => Ordering::Greater,
            (HandShape::Paper, HandShape::Scissors) => Ordering::Less,
            (HandShape::Scissors, HandShape::Rock) => Ordering::Less,
            (HandShape::Scissors, HandShape::Paper) => Ordering::Greater,
            _ => Ordering::Equal
        }
    }
}
impl From<&str> for HandShape {
    fn from(code: &str) -> Self {
        match code {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown handshape code {}", code)
        }
    }
}

pub struct Game {
    rounds: Vec<(HandShape, HandShape)>,
}

impl Game {
    pub fn new(round_codes: &str) -> Game {
        let rounds = round_codes
            .split('\n')
            .map(Self::parse_round_code)
            .collect();
        
        Game { rounds }
    }

    fn parse_round_code(round_code: &str) -> (HandShape, HandShape) {
        let shapes = round_code
            .split(' ')
            .map(|code| code.into())
            .collect::<Vec<HandShape>>();
        
        (shapes[0].clone(), shapes[1].clone())
    }

    pub fn calculate_game_score(&self) -> u32 {
        self.rounds
            .iter()
            .map(|(first, second)| second.play(first) + second.get_score())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_correctly_calculate_game_score() {
        let game = Game::new("A Y\nB X\nC Z");
        assert_eq!(game.calculate_game_score(), 15);
    }
}
