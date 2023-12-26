use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::{many1, many_till},
    branch::alt,
    character::complete::{one_of, anychar},
    IResult,
};

pub fn parser(input: &str) -> IResult<&str, Vec<u32>> {
    many1(
        map(many_till(
            anychar,
            alt((single_digit, written_digit))
        ), |(_, digit)| digit)
    )(input)
}

fn single_digit(input: &str) -> IResult<&str, u32> {
    map(one_of("123456789"), |ch| ch.to_digit(10).unwrap())(input)
}

fn written_digit(input: &str) -> IResult<&str, u32> {
    alt((
        map(tag("one"), |_| 1),
        map(tag("two"), |_| 2),
        map(tag("three"), |_| 3),
        map(tag("four"), |_| 4),
        map(tag("five"), |_| 5),
        map(tag("six"), |_| 6),
        map(tag("seven"), |_| 7),
        map(tag("eight"), |_| 8),
        map(tag("nine"), |_| 9),
    ))(input)
}

#[cfg(test)]
mod test {
    use test_case::case;
    use super::*;

    #[case("one" => 1)]
    #[case("two" => 2)]
    #[case("three" => 3)]
    #[case("four" => 4)]
    #[case("five" => 5)]
    #[case("six" => 6)]
    #[case("seven" => 7)]
    #[case("eight" => 8)]
    #[case("nine" => 9)]
    fn should_parse_written_digit(input: &str) -> u32 {
        written_digit(input).unwrap().1
    }

    #[case("1" => 1)]
    #[case("2" => 2)]
    #[case("3" => 3)]
    #[case("4" => 4)]
    #[case("5" => 5)]
    #[case("6" => 6)]
    #[case("7" => 7)]
    #[case("8" => 8)]
    #[case("9" => 9)]
    fn should_parse_single_digit(input: &str) -> u32 {
        single_digit(input).unwrap().1
    }

    #[case("one1" => vec![1, 1])]
    #[case("boonebo" => vec![1])]
    #[case("53eightfourlseven5bvtmzfkqc6" => vec![5, 3, 8, 4, 7, 5, 6])]
    fn should_parse_input(input: &str) -> Vec<u32> {
        parser(input).unwrap().1
    }
}
