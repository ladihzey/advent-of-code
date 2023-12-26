use std::fs;
use day_01::parser::*;

fn main() {
    let input = fs::read_to_string("assets/input.txt").unwrap();
    let answer = get_calibration_value(&input);
    println!("{answer}");
}

fn get_calibration_value(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let digits = parser(line).unwrap().1;
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_calibration_value() {
        let input = "\
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        ";
        let value = get_calibration_value(input);
        assert_eq!(value, 281);
    }
}
