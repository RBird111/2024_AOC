use helpers::print_results;

use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(11, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
    total_stones(data, 25)
}

fn part_2(data: &str) -> i64 {
    total_stones(data, 75)
}

fn total_stones(data: &str, blinks: usize) -> i64 {
    let mut stones: HashMap<Stone, i64> = HashMap::new();
    parse_data(data)
        .into_iter()
        .map(|val| Stone { val })
        .for_each(|s| *stones.entry(s).or_default() += 1);

    (0..blinks)
        .fold(stones, |stones, _| {
            let mut new_stones: HashMap<Stone, i64> = HashMap::new();
            stones.into_iter().for_each(|(s, n)| {
                s.replace()
                    .into_iter()
                    .for_each(|s| *new_stones.entry(s).or_default() += n);
            });
            new_stones
        })
        .into_values()
        .sum()
}

fn parse_data(data: &str) -> Vec<i64> {
    data.split_whitespace()
        .filter_map(|b| b.parse().ok())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone {
    val: i64,
}

impl Stone {
    fn replace(&self) -> Vec<Stone> {
        if self.val == 0 {
            return self.handle_zero();
        }
        if self.has_even_digits() {
            return self.handle_even_digits();
        }
        self.handle_default()
    }

    fn handle_zero(&self) -> Vec<Stone> {
        vec![Stone { val: 1 }]
    }

    fn handle_even_digits(&self) -> Vec<Stone> {
        let binding = self.val.to_string();
        let (left, right) = binding.split_at(self.count_digits() as usize / 2);
        [left, right]
            .into_iter()
            .filter_map(|s| s.parse().ok())
            .map(|val| Stone { val })
            .collect()
    }

    fn handle_default(&self) -> Vec<Stone> {
        vec![Stone {
            val: self.val * 2024,
        }]
    }

    fn count_digits(&self) -> i64 {
        let (mut digits, mut val) = (0, self.val);
        while val > 0 {
            digits += 1;
            val /= 10;
        }
        digits
    }

    fn has_even_digits(&self) -> bool {
        self.count_digits() % 2 == 0
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_1(input);
        let expected: i64 = 55312;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
