use helpers::print_results;

use regex::Regex;

use std::{collections::HashMap, error::Error, sync::OnceLock};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(1, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> u32 {
    let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|line| {
            let split: Vec<i32> = get_part_1_re()
                .splitn(line, 2)
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();
            (split[0], split[1])
        })
        .unzip();

    list_1.sort_unstable();
    list_2.sort_unstable();

    list_1
        .into_iter()
        .zip(list_2)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part_2(data: &str) -> u32 {
    let (list_1, list_2): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|line| {
            let split: Vec<i32> = get_part_1_re()
                .splitn(line, 2)
                .filter_map(|n| n.parse::<i32>().ok())
                .collect();
            (split[0], split[1])
        })
        .unzip();

    let mut freq: HashMap<i32, i32> = HashMap::new();
    list_2.into_iter().for_each(|n| {
        *freq.entry(n).or_insert(0) += 1;
    });

    list_1
        .into_iter()
        .map(|n| n * freq.get(&n).unwrap_or(&0))
        .map(|n| n as u32)
        .sum()
}

fn get_part_1_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\s+").unwrap())
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    fn get_input() -> &'static str {
        static INPUT: OnceLock<&'static str> = OnceLock::new();
        INPUT.get_or_init(|| include_str!("../input/test.txt"))
    }

    #[test]
    fn part_1_test() {
        let expected: u32 = 11;
        let actual: u32 = part_1(get_input());
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        );
    }

    #[test]
    fn part_2_test() {
        let expected: u32 = 31;
        let actual: u32 = part_2(get_input());
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        );
    }
}
