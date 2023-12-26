use std::fs;

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
            let digits: Vec<char> = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .collect();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();

            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_calibration_value() {
        let input = "\
            1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet\n\
        ";
        let value = get_calibration_value(input);
        assert_eq!(value, 142);
    }

}
