use helpers::print_results;

use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input(include_str!("../input/actual.txt"));
    print_results(6, &[part_1(input.clone()), part_2(input)]);
    Ok(())
}

fn part_1(data: Vec<Vec<u8>>) -> i32 {
    let (grid, _) = mark_grid(data);
    grid.into_iter().flatten().filter(|&b| b == b'X').count() as _
}

fn part_2(data: Vec<Vec<u8>>) -> i32 {
}

fn mark_grid(mut grid: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, bool) {
    const UP: (i32, i32) = (-1, 0);
    const DOWN: (i32, i32) = (1, 0);
    const LEFT: (i32, i32) = (0, -1);
    const RIGHT: (i32, i32) = (0, 1);

    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let get_val = |(x, y): (i32, i32), grid: &[Vec<u8>]| {
        if (0 <= x && x < m) && (0 <= y && y < n) {
            return grid[x as usize][y as usize];
        }
        b'~'
    };
    let walk = |(x, y): (i32, i32), (dx, dy): (i32, i32)| (x + dx, y + dy);
    let mark_loc = |(x, y): (i32, i32), grid: &mut [Vec<u8>]| grid[x as usize][y as usize] = b'X';
    let change_dir = |guard_dir: (i32, i32)| match guard_dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => unreachable!(),
    };

    let guard_loc = get_guard_loc(&grid);
    let mut guard = (guard_loc, UP);

    let mut visited = HashSet::new();
    let mut nothing_new = 0;

    loop {
        if !visited.insert(guard.0) {
            nothing_new += 1;
            if nothing_new == m * n {
                return (grid, true);
            }
        }
        mark_loc(guard.0, &mut grid);
        let guard_temp = (walk(guard.0, guard.1), guard.1);
        match get_val(guard_temp.0, &grid) {
            b'.' | b'X' => {
                guard = guard_temp;
            }
            b'#' => {
                guard.1 = change_dir(guard.1);
            }
            b'~' => break,
            _ => unreachable!(),
        }
    }
    (grid, false)
}

fn get_guard_loc(grid: &[Vec<u8>]) -> (i32, i32) {
    (0..grid.len())
        .flat_map(|x| (0..grid[0].len()).map(move |y| (x, y)))
        .find(|&(x, y)| grid[x][y] == b'^')
        .map(|(x, y)| (x as i32, y as i32))
        .unwrap()
}

fn parse_input(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|line| line.bytes().collect()).collect()
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = parse_input(include_str!("../input/test.txt"));
        let actual: i32 = part_1(input);
        let expected: i32 = 41;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = parse_input(include_str!("../input/test.txt"));
        let actual: i32 = part_2(input);
        let expected: i32 = 6;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
