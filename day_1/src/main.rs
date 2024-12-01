use helpers::print_results;

use regex::Regex;

use std::{collections::HashMap, error::Error, sync::OnceLock};

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
fn get_part_1_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\s+").unwrap())
}
#[cfg(test)]
mod day_1_tests {
    use super::*;

    fn get_input() -> &'static str {
        static INPUT: OnceLock<&'static str> = OnceLock::new();
        INPUT.get_or_init(|| include_str!("../input/day_1_test.txt"))
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
}
