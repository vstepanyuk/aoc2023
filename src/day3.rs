use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("../input/day3.txt");

struct Schematic {
    data: Vec<char>,
    width: usize,
    height: usize,
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];

impl FromStr for Schematic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.chars().filter(|&c| c != '\n').collect::<Vec<_>>();
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        Ok(Schematic {
            data,
            width,
            height,
        })
    }
}

impl Schematic {
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        (x < self.width && y < self.height)
            .then(|| self.data.get(y * self.width + x))
            .flatten()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(&char, (usize, usize))> {
        NEIGHBORS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                Some((
                    self.get(nx as usize, ny as usize)?,
                    (nx as usize, ny as usize),
                ))
            })
            .collect()
    }

    fn num_at(&self, x: usize, y: usize) -> u32 {
        let mut digits = (0..=x)
            .rev()
            .map(|x| *self.get(x, y).unwrap())
            .take_while(char::is_ascii_digit)
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        digits.reverse();

        digits.extend(
            (x + 1..self.width)
                .map(|x| *self.get(x, y).unwrap())
                .take_while(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );

        digits
            .iter()
            .rev()
            .fold((1, 0), |(mul, sum), &d| (mul * 10, sum + d * mul))
            .1
    }
}

fn main() {
    let s = Schematic::from_str(INPUT).unwrap();
    println!("Day 3, part 1: {}", part1(&s));
    println!("Day 3, part 2: {}", part2(&s));
}

fn part1(s: &Schematic) -> i32 {
    let mut visited = HashSet::new();
    let mut total = 0;

    for x in 0..s.width {
        for y in 0..s.height {
            let c = s.get(x, y).unwrap();
            if !c.is_ascii_digit() || visited.contains(&(x, y)) {
                continue;
            }

            let mut positions = HashSet::new();
            positions.insert((x, y));

            let num = (x..s.width)
                .map(|x| (s.get(x, y).unwrap(), (x, y)))
                .take_while(|(c, _)| c.is_ascii_digit())
                .fold(0, |v, c| {
                    positions.insert(c.1);
                    v * 10 + c.0.to_digit(10).unwrap()
                });

            if positions
                .iter()
                .flat_map(|p| s.get_neighbors(p.0, p.1))
                .any(|(c, _)| c != &'.' && !c.is_ascii_digit())
            {
                total += num as i32;
            }

            visited.extend(positions);
        }
    }

    total
}

fn part2(s: &Schematic) -> u32 {
    let mut total = 0;
    for x in 0..s.width {
        for y in 0..s.height {
            if s.get(x, y) != Some(&'*') {
                continue;
            }

            let nums = s
                .get_neighbors(x, y)
                .iter()
                .filter_map(|(c, pos)| {
                    if !c.is_ascii_digit() {
                        return None;
                    }

                    Some(s.num_at(pos.0, pos.1))
                })
                .unique()
                .collect::<Vec<_>>();

            if nums.len() != 2 {
                continue;
            }

            total += nums.iter().product::<u32>();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{part1, part2, Schematic};

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&Schematic::from_str(INPUT).unwrap()), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Schematic::from_str(INPUT).unwrap()), 467835);
    }
}
