use aoc::parse_nums;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day9.txt");

struct Report(Vec<i64>);

impl Report {
    fn new(input: impl AsRef<str>) -> Self {
        Self(parse_nums(input.as_ref()))
    }

    fn extrapolate(&self, f: &impl Fn(&[(i64, i64)]) -> i64) -> i64 {
        let mut items = vec![];
        let mut current = self.0.clone();

        while !current.iter().all(|v| *v == 0) {
            items.push((*current.first().unwrap(), *current.last().unwrap()));
            current = current.iter().tuple_windows().map(|(a, b)| b - a).collect();
        }

        f(&items)
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: impl AsRef<str>) -> i64 {
    solve(input, |items| items.iter().map(|item| item.1).sum())
}

fn part2(input: impl AsRef<str>) -> i64 {
    solve(input, |items| {
        items
            .iter()
            .map(|item| item.0)
            .rev()
            .fold(0, |acc, value| value - acc)
    })
}

fn solve(input: impl AsRef<str>, f: impl Fn(&[(i64, i64)]) -> i64) -> i64 {
    input
        .as_ref()
        .lines()
        .map(Report::new)
        .map(|r| r.extrapolate(&f))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("10  13  16  21  30  45"), 5);
    }
}
