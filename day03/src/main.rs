use helpers::print_results;

use regex::Regex;

use std::{error::Error, sync::OnceLock};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(3, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i32 {
    get_part_1_re()
        .captures_iter(data)
        .map(|c| c.extract())
        .filter_map(parse_captures)
        .sum()
}

fn part_2(data: &str) -> i32 {
    get_part_2_re()
        .captures_iter(data)
        .map(|c| c.extract())
        .scan(true, |should_mul, (_, [cap])| match cap {
            "do()" => {
                *should_mul = true;
                Some(0)
            }
            "don't()" => {
                *should_mul = false;
                Some(0)
            }
            _ => match *should_mul {
                true => Some(part_1(cap)),
                false => Some(0),
            },
        })
        .sum()
}

fn parse_captures((_, [left, right]): (&str, [&str; 2])) -> Option<i32> {
    Some(left.parse::<i32>().ok()? * right.parse::<i32>().ok()?)
}

fn get_part_1_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap())
}

fn get_part_2_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap())
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

    #[test]
    fn part_2_test() {
        let actual: i32 = 48;
        let expected: i32 = part_2(include_str!("../input/test2.txt"));
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
