use std::ops::RangeInclusive;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *
};

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

type AssignmentRange = RangeInclusive<u32>;

trait Intersectionable {
    fn includes(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl Intersectionable for AssignmentRange {
    fn includes(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.includes(self) ||
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn assignment_range(input: &str) -> IResult<&str, AssignmentRange> {
    let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
    Ok((input, start..=end))
}

fn assignment_pair(input: &str) -> IResult<&str, (AssignmentRange, AssignmentRange)> {
    let (input, (left, right)) = separated_pair(assignment_range, tag(","), assignment_range)(input)?;
    Ok((input, (left, right)))
}

fn parse_assignments(input: &str) -> Vec<(AssignmentRange, AssignmentRange)> {
    let (_, ranges) = separated_list1(newline, assignment_pair)(input).unwrap();
    ranges
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