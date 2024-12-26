use helpers::print_results;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(include_str!("../input/actual.txt"));
    print_results(9, &[part_1(input.clone()), part_2(input)]);
    Ok(())
}

fn part_1(mut input: Vec<i64>) -> i64 {
    loop {
        let l = input.iter().position(|&v| v == -1).unwrap();
        let r = input.iter().rposition(|&v| v != -1).unwrap();
        if l > r {
            break;
        }
        input.swap(l, r);
    }
    input
        .into_iter()
        .take_while(|&b| b != -1)
        .zip(0..)
        .map(|(b, i)| b * i)
        .sum()
}

fn part_2(input: Vec<i64>) -> i64 {
    let mut dir = File::from_vec(input);
    let mut r = dir.len();

    loop {
        r = match dir[..r].iter().rposition(|f| f.val != -1) {
            Some(j) => j,
            None => break,
        };
        if let Some(l) = dir
            .iter()
            .zip(0..)
            .position(|(f, l)| l < r && f.val == -1 && f.size >= dir[r].size)
        {
            dir[l].size -= dir[r].size;
            dir.insert(l, dir[r]);
            r = (r + 1).min(dir.len() - 1);
            dir[r].val = -1;
        }
    }

    dir.into_iter()
        .flat_map(|f| vec![f.val.max(0); f.size])
        .zip(0..)
        .map(|(b, i)| b * i)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct File {
    val: i64,
    size: usize,
}

impl File {
    fn from_vec(vec: Vec<i64>) -> Vec<File> {
        let mut file = File { val: -1, size: 0 };
        let mut dir = vec![];
        for n in vec {
            if file.val == n {
                file.inc();
                continue;
            }
            if file.size > 0 {
                dir.push(file)
            }
            file.reset(n);
        }
        dir.push(file);
        dir
    }

    fn inc(&mut self) {
        self.size += 1
    }

    fn reset(&mut self, val: i64) {
        self.val = val;
        self.size = 1;
    }
}

fn get_input(data: &str) -> Vec<i64> {
    data.lines()
        .flat_map(|line| line.bytes())
        .map(|b| (b - b'0') as usize)
        .scan((false, -1), |(fg, idx), n| {
            *fg = !*fg;
            let val = if *fg {
                *idx += 1;
                *idx
            } else {
                -1
            };
            Some(vec![val; n])
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = get_input(include_str!("../input/test.txt"));
        let actual: i64 = part_1(input);
        let expected: i64 = 1928;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = get_input(include_str!("../input/test.txt"));
        let actual: i64 = part_2(input);
        let expected: i64 = 2858;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
