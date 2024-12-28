use helpers::print_results;

use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(10, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
    let map = parse_data(data);
    map.iter()
        .zip(0..)
        // get zeros
        .flat_map(|(row, i)| {
            row.iter()
                .zip(0..)
                .filter(|&(&b, _)| b == 0)
                .map(move |(_, j)| (i, j))
        })
        // get score
        .map(|loc| score_trailhead(loc, &map))
        .sum()
}

fn part_2(data: &str) -> i64 {
    let map = parse_data(data);
    map.iter()
        .zip(0..)
        // get zeros
        .flat_map(|(row, i)| {
            row.iter()
                .zip(0..)
                .filter(|&(&b, _)| b == 0)
                .map(move |(_, j)| (i, j))
        })
        // get rating
        .map(|loc| rate_trailhead(loc, &map))
        .sum()
}

fn score_trailhead(loc: (i32, i32), map: &[Vec<i32>]) -> i64 {
    const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let is_valid =
        |&(x, y): &(i32, i32)| 0 <= x && x < map.len() as i32 && 0 <= y && y < map[0].len() as i32;

    let (mut stack, mut set) = (vec![loc], HashSet::new());
    while let Some((x, y)) = stack.pop() {
        let val = map[x as usize][y as usize];
        if val == 9 {
            set.insert((x, y));
            continue;
        }
        stack.extend(
            NEIGHBORS
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(is_valid)
                .filter(|&(x, y)| map[x as usize][y as usize] == val + 1),
        );
    }
    set.len() as _
}

fn rate_trailhead(loc: (i32, i32), map: &[Vec<i32>]) -> i64 {
    const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let is_valid =
        |&(x, y): &(i32, i32)| 0 <= x && x < map.len() as i32 && 0 <= y && y < map[0].len() as i32;

    let (mut stack, mut set) = (vec![Trail::new().insert(loc)], HashSet::new());
    while let Some(trail) = stack.pop() {
        if trail.is_complete(map) {
            set.insert(trail);
            continue;
        }
        let (x, y) = trail.cur_loc();
        stack.extend(
            NEIGHBORS
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(is_valid)
                .filter(|&(x, y)| map[x as usize][y as usize] == trail.get_val(map) + 1)
                .map(|loc| trail.insert(loc)),
        );
    }
    set.len() as _
}

fn parse_data(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Trail {
    trail: [(i32, i32); 10],
    next: i32,
}

impl Trail {
    fn new() -> Self {
        Trail {
            trail: [(0, 0); 10],
            next: 0,
        }
    }

    fn insert(&self, loc: (i32, i32)) -> Self {
        let mut t = *self;
        t.trail[self.next as usize] = loc;
        t.next += 1;
        t
    }

    fn is_complete(&self, map: &[Vec<i32>]) -> bool {
        let (x, y) = self.trail[9];
        map[x as usize][y as usize] == 9
    }

    fn cur_loc(&self) -> (i32, i32) {
        self.trail[(self.next - 1) as usize]
    }

    fn get_val(&self, map: &[Vec<i32>]) -> i32 {
        let (x, y) = self.cur_loc();
        map[x as usize][y as usize]
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_1(input);
        let expected: i64 = 36;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_2(input);
        let expected: i64 = 81;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
