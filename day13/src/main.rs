use helpers::print_results;

use regex::Regex;

use std::{error::Error, sync::OnceLock};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(13, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
    ClawMachine::from_input(data)
        .into_iter()
        .filter_map(ClawMachine::find_intersection)
        .map(|(a, b)| 3 * a + b)
        .sum()
}

fn part_2(data: &str) -> i64 {
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    const PATTERN: &str = r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)
Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)
Prize: X=(?<px>\d+), Y=(?<py>\d+)";

    fn get_re() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new(Self::PATTERN).unwrap())
    }

    fn from_input(input: &str) -> Vec<Self> {
        Self::get_re()
            .captures_iter(input)
            .map(|cap| {
                let (_, vals): (&str, [&str; 6]) = cap.extract();
                vals.iter()
                    .filter_map(|val| val.parse::<i64>().ok())
                    .collect::<Vec<_>>()
            })
            .map(Self::new)
            .collect()
    }

    fn new(vals: Vec<i64>) -> Self {
        Self {
            button_a: (vals[0], vals[1]),
            button_b: (vals[2], vals[3]),
            prize: (vals[4], vals[5]),
        }
    }

    fn find_intersection(self) -> Option<(i64, i64)> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let ax_by = ax * by;
        let px_by = px * by;

        let ay_bx = ay * bx;
        let py_bx = py * bx;

        let a = (px_by - py_bx) / (ax_by - ay_bx);
        let b = (py - ay * a) / by;

        self.is_valid((a, b)).then_some((a, b))
    }

    fn is_valid(&self, (a, b): (i64, i64)) -> bool {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;
        a * ax + b * bx == px && a * ay + b * by == py
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_1(input);
        let expected: i64 = 480;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
