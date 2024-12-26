use helpers::print_results;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(8, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
    let parsed: Vec<Vec<u8>> = data.lines().map(|l| l.bytes().collect()).collect();
    let (rows, cols) = (parsed.len() as i32, parsed[0].len() as i32);

    let mut map: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    parsed
        .into_iter()
        .enumerate()
        .flat_map(|(row, arr)| {
            arr.into_iter()
                .enumerate()
                .filter(|(_, val)| *val != b'.')
                .map(move |(col, val)| (row as i32, col as i32, val))
        })
        .for_each(|(row, col, val)| {
            map.entry(val).or_default().push((row, col));
        });

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    map.into_values().for_each(|locs| {
        set.extend(locs.iter().enumerate().flat_map(|(i, &(r1, c1))| {
            locs[i + 1..]
                .iter()
                .flat_map(move |(r2, c2)| [(2 * r1 - r2, 2 * c1 - c2), (2 * r2 - r1, 2 * c2 - c1)])
                .filter(|&(r, c)| 0 <= r && r < rows && 0 <= c && c < cols)
        }));
    });

    set.len() as _
}

fn part_2(data: &str) -> i64 {
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_1(input);
        let expected: i64 = 14;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_2(input);
        let expected: i64 = 34;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
