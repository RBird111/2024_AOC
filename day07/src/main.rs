use helpers::print_results;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(7, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
    const OPS: [Operation; 2] = [Operation::Add, Operation::Mul];
    data.lines()
        .map(Equation::new)
        .filter(|eq| eq.is_valid(&OPS))
        .map(|eq| eq.result)
        .sum()
}

fn part_2(data: &str) -> i64 {
    const OPS: [Operation; 3] = [Operation::Add, Operation::Mul, Operation::Con];
    data.lines()
        .map(Equation::new)
        .filter(|eq| eq.is_valid(&OPS))
        .map(|eq| eq.result)
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
    Con,
}

impl Operation {
    fn calc(self, a: i64, b: i64) -> i64 {
        use Operation::*;
        match self {
            Add => a + b,
            Mul => a * b,
            Con => {
                let (mut v, mut len) = (b, 0);
                while v > 0 {
                    len += 1;
                    v /= 10;
                }
                a * 10i64.pow(len) + b
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    values: Vec<i64>,
    result: i64,
}

impl Equation {
    fn new(line: &str) -> Self {
        let mut values: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.trim_matches(':'))
            .filter_map(|v| v.parse().ok())
            .collect();
        let result = values.remove(0);
        Self { values, result }
    }

    fn is_valid(&self, ops: &[Operation]) -> bool {
        self.get_ops(ops)
            .iter()
            .map(|ops| self.calc_result(ops))
            .any(|res| res == self.result)
    }

    fn get_ops(&self, ops: &[Operation]) -> Vec<Vec<Operation>> {
        let it = || ops.iter().copied().map(|op| vec![op]);
        (0..self.values.len() - 1).fold(it().collect::<Vec<_>>(), |arr, _| {
            arr.into_iter()
                .flat_map(|ops| it().map(move |op| [ops.clone(), op].concat()))
                .collect()
        })
    }

    fn calc_result(&self, ops: &[Operation]) -> i64 {
        let mut ops = ops.iter();
        self.values
            .iter()
            .copied()
            .reduce(|res, val| ops.next().unwrap().calc(res, val))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_1(input);
        let expected: i64 = 3749;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_2(input);
        let expected: i64 = 11387;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
