use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

const INPUT: &str = include_str!("../input/day14.txt");

type StoneSet = BTreeSet<(isize, isize)>;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Platform {
    persistent: StoneSet,
    stones: StoneSet,
    height: usize,
}

impl Platform {
    fn new(input: impl AsRef<str>) -> Self {
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut persistent = StoneSet::new();
        let mut stones = StoneSet::new();

        for (y, line) in input.as_ref().lines().enumerate() {
            width = line.len();
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        persistent.insert((x as isize + 1, y as isize + 1));
                    }
                    'O' => {
                        stones.insert((x as isize + 1, y as isize + 1));
                    }
                    _ => {}
                }
            }

            height += 1;
        }

        (0..height + 2).for_each(|y| {
            persistent.insert((0, y as isize));
            persistent.insert((width as isize + 1, y as isize));
        });

        (0..width + 2).for_each(|x| {
            persistent.insert((x as isize, 0));
            persistent.insert((x as isize, height as isize + 1));
        });

        Self {
            persistent,
            stones,
            height: height + 2,
        }
    }
    fn count(&self) -> usize {
        self.stones
            .iter()
            .map(|pos| self.height - 1 - pos.1 as usize)
            .sum()
    }

    fn tilt(&mut self) {
        let mut new_stones = StoneSet::new();
        self.stones
            .iter()
            .sorted_by_key(|pos| pos.1)
            .for_each(|pos| {
                let mut pos = *pos;

                while !new_stones.contains(&(pos.0, pos.1 - 1))
                    && !self.persistent.contains(&(pos.0, pos.1 - 1))
                {
                    pos.1 -= 1;
                }

                new_stones.insert(pos);
            });

        self.stones = new_stones;
    }

    fn cycle(&mut self) {
        self.tilt();

        self.rotate(1);
        self.tilt();
        self.rotate(3);

        self.rotate(2);
        self.tilt();
        self.rotate(2);

        self.rotate(3);
        self.tilt();
        self.rotate(1);
    }

    fn rotate(&mut self, times: usize) {
        self.stones = Self::rotate_inner(&self.stones, times);
        self.persistent = Self::rotate_inner(&self.persistent, times);
    }

    fn rotate_inner(points: &StoneSet, times: usize) -> StoneSet {
        match times % 4 {
            3 => points.iter().map(|&(x, y)| (y, -x)).collect(),
            2 => points.iter().map(|&(x, y)| (-x, -y)).collect(),
            1 => points.iter().map(|&(x, y)| (-y, x)).collect(),
            _ => points.clone(),
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: impl AsRef<str>) -> usize {
    let mut platform = Platform::new(input);
    platform.tilt();
    platform.count()
}

fn part2(input: impl AsRef<str>) -> usize {
    let mut platform = Platform::new(input.as_ref());
    let mut cache = vec![];
    let mut visited = HashMap::new();

    let index = loop {
        platform.cycle();

        if let Some(&index) = visited.get(&platform) {
            break index;
        } else {
            visited.insert(platform.clone(), cache.len());
            cache.push(platform.count());
        }
    };

    let index = (1_000_000_000 - cache.len() - 1) % (cache.len() - index) + index;
    cache[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 64);
    }
}
