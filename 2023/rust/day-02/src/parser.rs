use crate::game::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::{map, opt, value},
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn game(input: &str) -> IResult<&str, Game> {
    map(
        separated_pair(
            preceded(tag("Game "), u32),
            tag(": "),
            separated_list1(tag("; "), round),
        ),
        |(id, rounds)| Game { id, rounds },
    )(input)
}

fn round(input: &str) -> IResult<&str, Round> {
    let cube_color = alt((
        value(CubeColor::Green, tag("green")),
        value(CubeColor::Blue, tag("blue")),
        value(CubeColor::Red, tag("red")),
    ));
    let cubes_amount = separated_pair(u32, tag(" "), cube_color);

    fold_many1(
        terminated(cubes_amount, opt(tag(", "))),
        Round::default,
        |round, (amount, color)| round.add_cubes_amount(amount, color),
    )(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::*;

    #[case("1 green, 2 red, 6 blue" => Round { green: 1, red: 2, blue: 6 }; "when all colors are present")]
    #[case("2 red, 6 blue" => Round { green: 0, red: 2, blue: 6 }; "when two colors are present")]
    #[case("2 red" => Round { green: 0, red: 2, blue: 0 }; "when only one color is present")]
    fn should_parse_game_round(input: &str) -> Round {
        round(input).unwrap().1
    }

    #[case(
        "Game 6: 2 blue, 12 red, 2 green; 3 green, 2 red; 3 green, 3 blue, 10 red" =>
        Game {
            id: 6,
            rounds: vec![
                Round { blue: 2, red: 12, green: 2 },
                Round { blue: 0, red: 2, green: 3 },
                Round { blue: 3, red: 10, green: 3 },
            ]
        };
        "when passed with a multiple rounds input"
    )]
    #[case(
        "Game 11: 2 blue, 12 red, 2 green" =>
        Game {
            id: 11,
            rounds: vec![
                Round { blue: 2, red: 12, green: 2 },
            ]
        };
        "when passed with a single round input"
    )]
    fn should_parse_game(input: &str) -> Game {
        game(input).unwrap().1
    }
}
