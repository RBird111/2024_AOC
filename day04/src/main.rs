use helpers::print_results;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(2, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i32 {
    let grid = into_grid(data);
    (0..grid.len())
        .flat_map(|x| (0..grid[0].len()).map(move |y| (x, y)))
        .filter(|&(x, y)| grid[x][y] == b'X')
        .map(|(x, y)| count_xmas((x, y), &grid))
        .sum()
}

fn part_2(data: &str) -> i32 {
    let grid = into_grid(data);
    (0..grid.len())
        .flat_map(|x| (0..grid[0].len()).map(move |y| (x, y)))
        .filter(|&(x, y)| grid[x][y] == b'A')
        .filter_map(|(x, y)| is_x_mas((x, y), &grid))
        .sum()
}

fn into_grid(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(str::as_bytes).map(Vec::from).collect()
}

const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

fn count_xmas((x, y): (usize, usize), grid: &[Vec<u8>]) -> i32 {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|&diff| diff != (0, 0))
        .filter(|&diff| is_xmas((x, y), diff, grid))
        .count() as _
}

fn is_xmas((x, y): (usize, usize), (dx, dy): (i32, i32), grid: &[Vec<u8>]) -> bool {
    let get_mod = |diff: i32| (diff.abs() == 1) as usize;
    let get_check = |diff: i32| {
        if diff < 0 {
            usize::checked_sub
        } else {
            usize::checked_add
        }
    };

    let mut iter = XMAS.iter().skip(1).peekable();
    let (xmod, ymod) = (get_mod(dx), get_mod(dy));
    let (xcheck, ycheck) = (get_check(dx), get_check(dy));

    let (mut dx, mut dy) = (x, y);
    while let Some((x, y)) = xcheck(dx, xmod).and_then(|x| Some((x, ycheck(dy, ymod)?))) {
        (dx, dy) = (x, y);
        if grid.get(x).is_none_or(|row| row.get(y) != iter.next()) {
            return false;
        }
        if iter.peek().is_none() {
            return true;
        }
    }

    false
}

fn is_x_mas((x, y): (usize, usize), grid: &[Vec<u8>]) -> Option<i32> {
    // TODO: Refactor

    let get_loc = |(x, y): (usize, usize)| grid.get(x).and_then(|row| row.get(y));

    // get bot-left
    let bot_left = x
        .checked_add(1)
        .and_then(|dx| Some((dx, y.checked_sub(1)?)))
        .and_then(get_loc)?;

    // get top-left
    let top_left = x
        .checked_sub(1)
        .and_then(|dx| Some((dx, y.checked_sub(1)?)))
        .and_then(get_loc)?;

    // get bot-right
    let bot_right = x
        .checked_add(1)
        .and_then(|dx| Some((dx, y.checked_add(1)?)))
        .and_then(get_loc)?;

    // get top-right
    let top_right = x
        .checked_sub(1)
        .and_then(|dx| Some((dx, y.checked_add(1)?)))
        .and_then(get_loc)?;

    match ((&bot_left, &top_right), (&top_left, &bot_right)) {
        ((b'M', b'S'), (b'M', b'S'))
        | ((b'S', b'M'), (b'M', b'S'))
        | ((b'S', b'M'), (b'S', b'M'))
        | ((b'M', b'S'), (b'S', b'M')) => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let expected: i32 = 18;
        let actual: i32 = part_1(include_str!("../input/test.txt"));
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let expected: i32 = 9;
        let actual: i32 = part_2(include_str!("../input/test.txt"));
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
