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
        .filter(|&loc| is_x_mas(loc, &grid))
        .count() as _
}

fn into_grid(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(str::as_bytes).map(Vec::from).collect()
}

fn count_xmas((x, y): (usize, usize), grid: &[Vec<u8>]) -> i32 {
    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|&diff| diff != (0, 0))
        .filter(|&diff| is_xmas((x, y), diff, grid))
        .count() as _
}

fn is_xmas((x, y): (usize, usize), (dx, dy): (i32, i32), grid: &[Vec<u8>]) -> bool {
    const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

    let get_mod = |diff: i32| (diff.abs() == 1) as usize;

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

fn is_x_mas((x, y): (usize, usize), grid: &[Vec<u8>]) -> bool {
    const DIFFS: [i32; 2] = [1, -1]; // [checked_add, checked_sub]

    let get_loc = |(x, y): (usize, usize)| grid.get(x).and_then(|row| row.get(y)).copied();
    let get_val = |(x_diff, y_diff): (i32, i32)| {
        get_check(x_diff)(x, 1)
            .and_then(|dx| Some((dx, get_check(y_diff)(y, 1)?)))
            .and_then(get_loc)
    };

    let [bot_right, bot_left, top_right, top_left] = DIFFS
        .into_iter()
        .flat_map(|x_diff| DIFFS.into_iter().map(move |y_diff| (x_diff, y_diff)))
        .filter_map(get_val)
        .collect::<Vec<_>>()[..]
    else {
        return false;
    };

    const PERMS: [(u8, u8); 2] = [(b'M', b'S'), (b'S', b'M')];
    PERMS
        .into_iter()
        .flat_map(|x| PERMS.into_iter().map(move |y| (x, y)))
        .any(|v| v == ((bot_left, top_right), (top_left, bot_right)))
}

fn get_check(diff: i32) -> fn(usize, usize) -> Option<usize> {
    if diff < 0 {
        usize::checked_sub
    } else {
        usize::checked_add
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
