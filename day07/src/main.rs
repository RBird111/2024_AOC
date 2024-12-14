use helpers::print_results;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(7, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
}

fn part_2(data: &str) -> i64 {
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
