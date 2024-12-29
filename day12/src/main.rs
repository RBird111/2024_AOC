use helpers::print_results;

use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input/actual.txt");
    print_results(12, &[part_1(input), part_2(input)]);
    Ok(())
}

fn part_1(data: &str) -> i64 {
    let (mut price, mut visited, garden) = (0, HashSet::new(), Garden::new(data));
    for loc in (0..garden.rows).flat_map(|row| (0..garden.cols).map(move |col| (row, col))) {
        if visited.insert(loc) {
            if let Some(region) = Region::map_region(loc, &garden) {
                price += region.price_per_perimiter();
                visited.extend(region.locations);
            };
        }
    }
    price
}

fn part_2(data: &str) -> i64 {
}

struct Region {
    area: i64,
    perimiter: i64,
    locations: HashSet<(i32, i32)>,
}

impl Region {
    fn new() -> Region {
        Region {
            area: 0,
            perimiter: 0,
            locations: HashSet::new(),
        }
    }

    fn map_region(loc: (i32, i32), garden: &Garden) -> Option<Region> {
        let crop = garden.get_val(loc)?;
        let (mut region, mut queue) = (Region::new(), VecDeque::from([loc]));
        region.locations.insert(loc);

        while let Some(loc) = queue.pop_front() {
            region.area += 1;
            region.perimiter += 4;
            garden
                .get_neighbors(loc)
                .into_iter()
                .filter(|&loc| garden.get_val(loc).is_some_and(|val| val == crop))
                .for_each(|loc| {
                    region.perimiter -= 1;
                    if region.locations.insert(loc) {
                        queue.push_back(loc);
                    }
                });
        }

        Some(region)
    }

    fn price_per_perimiter(&self) -> i64 {
        self.perimiter * self.area
    }
}

#[derive(Debug, Clone)]
struct Garden {
    rows: i32,
    cols: i32,
    buff: Vec<u8>,
}

impl Garden {
    fn new(data: &str) -> Self {
        let arr: Vec<Vec<_>> = data.lines().map(|line| line.bytes().collect()).collect();
        Garden {
            rows: arr.len() as _,
            cols: arr[0].len() as _,
            buff: arr.into_iter().flatten().collect(),
        }
    }

    fn get_neighbors(&self, (x, y): (i32, i32)) -> Vec<(i32, i32)> {
        const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        NEIGHBORS
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|&loc| self.get_val(loc).is_some())
            .collect()
    }

    fn get_val(&self, loc: (i32, i32)) -> Option<u8> {
        self.buff.get(self.idx(loc)?).copied()
    }

    fn idx(&self, (r, c): (i32, i32)) -> Option<usize> {
        (0 <= r && r < self.rows && 0 <= c && c < self.cols).then(|| (self.cols * r + c) as usize)
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_1(input);
        let expected: i64 = 1930;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }

    #[test]
    fn part_2_test() {
        let input = include_str!("../input/test.txt");
        let actual: i64 = part_2(input);
        let expected: i64 = 1206;
        assert_eq!(
            actual, expected,
            "actual ({actual}) != expected ({expected})",
        )
    }
}
