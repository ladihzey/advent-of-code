use std::collections::{HashSet, HashMap};

pub fn calculate_dupe_items_priority(input: &str) -> u32 {
    input
        .lines()
        .map(|items| items.split_at(items.len() / 2))
        .filter_map(|(left, right)| left.chars().find(|item| right.contains(*item)))
        .map(get_item_priority)
        .sum()
}

pub fn calculate_badges_priority(input: &str) -> u32 {
    let rucksack_groups = input
        .lines()
        .map(|rucksack| {
            let unique_items = rucksack.chars().collect::<HashSet<char>>();
            Vec::from_iter(unique_items).iter().collect::<String>()
        })
        .enumerate()
        .fold(Vec::<String>::new(), |mut groups, (index, rucksack)| {
            if let Some(group) = groups.get_mut(index / 3) {
                group.push_str(&rucksack);
            } else {
                groups.push(rucksack.to_owned());
            }

            groups
        });

    rucksack_groups
        .iter()
        .filter_map(|group| {
            group.chars().fold(HashMap::new(), |mut items_count, item| {
                *items_count.entry(item).or_insert(0) += 1;
                items_count
            })
            .iter()
            .find(|(_, &count)| count == 3)
            .map(|(key, _)| *key)
        })
        .map(get_item_priority)
        .sum()
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
