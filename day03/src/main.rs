use helpers::print_results;

use regex::Regex;

use std::{error::Error, sync::OnceLock};

fn main() -> Result<(), Box<dyn Error>> {
fn part_1(data: &str) -> i32 {
    get_part_1_re()
        .captures_iter(data)
        .map(|c| c.extract())
        .filter_map(parse_captures)
        .sum()
}
fn parse_captures((_, [left, right]): (&str, [&str; 2])) -> Option<i32> {
    Some(left.parse::<i32>().ok()? * right.parse::<i32>().ok()?)
}

fn get_part_1_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap())
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let actual: i32 = 161;
        let expected: i32 = part_1(include_str!("../input/test.txt"));
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
