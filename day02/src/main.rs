use helpers::print_results;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(2, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> u32 {
    data.lines()
        .map(str::split_whitespace)
        .map(Vec::from_iter)
        .map(check_log)
        .map(u32::from)
        .sum()
}

fn part_2(data: &str) -> u32 {
    data.lines()
        .map(str::split_whitespace)
        .map(Vec::from_iter)
        .map(|log| {
            (0..log.len())
                .map(|i| {
                    let mut log = log.to_vec();
                    log.remove(i);
                    log
                })
                .any(check_log)
        })
        .map(u32::from)
        .sum()
}

fn check_log(log: Vec<&str>) -> bool {
    log.windows(2)
        .filter_map(|win| Some(win[1].parse::<i32>().ok()? - win[0].parse::<i32>().ok()?))
        .scan(None, get_dir_diff)
        .all(is_safe)
}

fn get_dir_diff(dir: &mut Option<i32>, diff: i32) -> Option<(bool, i32)> {
    let same_dir = dir.is_none_or(|dir| dir == diff.signum());
    *dir = Some(diff.signum());
    Some((same_dir, diff.abs()))
}

fn is_safe((same_dir, diff): (bool, i32)) -> bool {
    same_dir && (1..=3).contains(&diff)
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    use std::sync::OnceLock;

    fn get_input() -> &'static str {
        static INPUT: OnceLock<&'static str> = OnceLock::new();
        INPUT.get_or_init(|| include_str!("../input/test.txt"))
    }

    #[test]
    fn part_1_test() {
        let expected: u32 = 2;
        let actual: u32 = part_1(get_input());
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        );
    }

    #[test]
    fn part_2_test() {
        let expected: u32 = 4;
        let actual: u32 = part_2(get_input());
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        );
    }
}
