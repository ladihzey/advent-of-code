use std::cmp;

#[derive(PartialEq, Debug)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn is_possible(&self, threshold_round: &GameConfig) -> bool {
        self.rounds.iter().all(|r| r.is_fit_into(threshold_round))
    }

    pub fn get_minimum_game_config(&self) -> GameConfig {
        self.rounds.iter().fold(
            GameConfig { blue: 1, red: 1, green: 1 },
            |mut gc, r| {
                gc.red = cmp::max(gc.red, r.red);
                gc.blue = cmp::max(gc.blue, r.blue);
                gc.green = cmp::max(gc.green, r.green);
                gc
            }
        )
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
pub type GameConfig = Round;

impl Round {
    pub fn add_cubes_amount(mut self, amount: u32, color: CubeColor) -> Self {
        match color {
            CubeColor::Green => self.green += amount,
            CubeColor::Red => self.red += amount,
            CubeColor::Blue => self.blue += amount,
        };
        self
    }

    pub fn is_fit_into(&self, other: &GameConfig) -> bool {
        self.green <= other.green
        && self.blue <= other.blue
        && self.red <= other.red
    }
}

#[derive(Clone)]
pub enum CubeColor {
    Green,
    Red,
    Blue,
}
