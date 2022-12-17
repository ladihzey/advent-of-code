use regex::Regex;
use std::ops::RangeInclusive;

pub fn count_duplicating_assignments(input: &str) -> usize {
    parse_assignments(input)
        .iter()
        .filter(|(a, b)| a.includes(b) || b.includes(a))
        .count()
}

pub fn count_overlapping_assignments(input: &str) -> usize {
    parse_assignments(input)
        .iter()
        .filter(|(a, b)| a.overlaps(b))
        .count()
}

trait Intersectionable {
    fn includes(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl Intersectionable for RangeInclusive<u32> {
    fn includes(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.includes(self) ||
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn parse_assignments(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let re = Regex::new(r"(\d+)").unwrap();

    input
        .lines()
        .map(|assignments_str| re
            .captures_iter(assignments_str)
            .map(|c| c[0].parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        )
        .map(|assignments| {
            if let [a_start, a_end, b_start, b_end] = assignments.as_slice() {
                (*a_start..=*a_end, *b_start..=*b_end)
            } else {
                panic!("Can't parse assignments");
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8\n\
    ";

    #[test]
    fn should_count_duplicating_assignments() {
        let result = count_duplicating_assignments(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn should_count_overlapping_assignments() {
        let result = count_overlapping_assignments(INPUT);
        assert_eq!(result, 4);
    }
}