use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::value,
    branch::alt,
    multi::many1,
};

pub fn parser(input: &str) -> Vec<u32> {
    let (_, numbers) = many1(number)(input).unwrap();
    numbers
        .iter()
        .copied()
        .flatten()
        .collect()
}

fn number(input: &str) -> IResult<&str, Option<u32>> {
    let res : IResult<&str, u32> = alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input);

    let (input, digit) = anychar(input)?;

    match res {
        Ok((_, digit)) => Ok((input, Some(digit))),
        Err(_) => Ok((input, digit.to_digit(10))),
    }
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
    #[case("1" => 1)]
    #[case("2" => 2)]
    #[case("3" => 3)]
    #[case("4" => 4)]
    #[case("5" => 5)]
    #[case("6" => 6)]
    #[case("7" => 7)]
    #[case("8" => 8)]
    #[case("9" => 9)]
    fn should_parse_number(input: &str) -> u32 {
        number(input).unwrap().1.unwrap()
    }

    #[case("one2o" => vec![1, 2])]
    #[case("boonebo" => vec![1])]
    #[case("1oneight" => vec![1, 1, 8])]
    fn should_parse_input(input: &str) -> Vec<u32> {
        parser(input)
    }
}
