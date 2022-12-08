use std::collections::HashSet;

pub fn calculate_dupe_items_priority(input: &str) -> u32 {
    input
        .split('\n')
        .map(|items| items.split_at(items.len() / 2))
        .filter_map(|(left, right)| {
            let left_set: HashSet<char> = left.chars().collect();
            right.chars().find(|c| left_set.contains(c))
        })
        .map(get_item_priority)
        .sum()
}

pub fn calculate_badges_priority(input: &str) -> u32 {
    0
}

fn get_item_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw\
    ";

    #[test]
    fn should_calculate_dupe_items_priority() {
        let result = calculate_dupe_items_priority(INPUT);
        assert_eq!(result, 157);
    }

    #[test]
    fn should_calculate_badges_priority() {
        let result = calculate_badges_priority(INPUT);
        assert_eq!(result, 70);
    }
}
