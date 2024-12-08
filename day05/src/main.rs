use helpers::print_results;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(include_str!("../input/actual.txt"));
    print_results(5, &[part_1(&input), part_2(&input)]);
    Ok(())
}

fn part_1((map, updates): &(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)) -> i32 {
    updates
        .iter()
        .filter(|update| is_valid_update(update, map))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_2((map, updates): &(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>)) -> i32 {
    let make_valid = |update: &Vec<i32>| {
        let mut update = update.to_owned();
        while let Some(i) = update[..update.len() - 1]
            .iter()
            .enumerate()
            .position(|(i, b)| map.get(b).is_some_and(|set| set.contains(&update[i + 1])))
        {
            update.swap(i, i + 1);
        }
        update
    };

    updates
        .iter()
        .filter(|update| !is_valid_update(update, map))
        .map(make_valid)
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_valid_update(update: &[i32], map: &HashMap<i32, HashSet<i32>>) -> bool {
    let is_wrong_order = |(i, before): (usize, &i32)| {
        update[i + 1..]
            .iter()
            .any(|after| map.get(after).is_some_and(|set| set.contains(before)))
    };
    !update.iter().enumerate().any(is_wrong_order)
}

fn parse_input(data: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (ordering, updates): (Vec<_>, Vec<_>) = data.lines().partition(|s| s.contains("|"));

    let mut before_after_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    ordering
        .into_iter()
        .map(|s| {
            let arr: Vec<i32> = s.split('|').filter_map(|v| v.parse().ok()).collect();
            (arr[0], arr[1])
        })
        .for_each(|(before, after)| {
            before_after_map.entry(before).or_default().insert(after);
        });

    let updates: Vec<Vec<i32>> = updates
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').filter_map(|v| v.parse().ok()).collect())
        .collect();

    (before_after_map, updates)
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = parse_input(include_str!("../input/test.txt"));
        let actual: i32 = part_1(&input);
        let expected: i32 = 143;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(include_str!("../input/test.txt"));
        let actual: i32 = part_2(&input);
        let expected: i32 = 123;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
