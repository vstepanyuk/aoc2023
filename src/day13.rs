use itertools::Itertools;
use num::Integer;
use pathfinding::matrix::Matrix;

use aoc::MatrixExt;

const INPUT: &str = include_str!("../input/day13.txt");

struct Frame(Matrix<char>);

impl Frame {
    fn new(input: impl AsRef<str>) -> Self {
        Self(Matrix::from_iter(
            input.as_ref().lines().map(|line| line.chars()),
        ))
    }

    fn find_mirror(&self) -> usize {
        self.find_mirror_inner(self.0.rows_iter(), 100)
            .or_else(|| self.find_mirror_inner(self.0.columns_iter(), 1))
            .unwrap_or_default()
    }

    #[inline]
    fn diff<T: PartialEq>(a: &[T], b: &[T]) -> usize {
        a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
    }

    #[inline]
    fn is_reflective<T: PartialEq>(s: &[T]) -> bool {
        s.len().is_even() && s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
    }

    fn find_other_mirror(&self) -> usize {
        self.find_mirror_other_inner(
            self.0.rows_iter(),
            self.find_mirror_inner(self.0.rows_iter(), 1)
                .map(|idx| idx - 1),
            100,
        )
        .or_else(|| {
            self.find_mirror_other_inner(
                self.0.columns_iter(),
                self.find_mirror_inner(self.0.columns_iter(), 1)
                    .map(|idx| idx - 1),
                1,
            )
        })
        .unwrap_or_default()
    }

    fn find_mirror_inner<'a, T: 'a + PartialEq>(
        &'a self,
        items: impl Iterator<Item = Vec<&'a T>>,
        factor: usize,
    ) -> Option<usize> {
        let items = items.collect::<Vec<_>>();

        items
            .iter()
            .tuple_windows()
            .enumerate()
            .find(|(pos, (a, b))| {
                a == b
                    && items[*pos + 2..]
                        .iter()
                        .zip(items[..*pos].iter().rev())
                        .all(|(a, b)| a == b)
            })
            .map(|(idx, _)| (idx + 1) * factor)
    }

    fn find_mirror_other_inner<'a, T: 'a + PartialEq>(
        &'a self,
        items: impl Iterator<Item = Vec<&'a T>>,
        ignore: Option<usize>,
        factor: usize,
    ) -> Option<usize> {
        let items = items.collect::<Vec<_>>();
        let items_len = items.len();

        (0..items_len)
            .flat_map(|i| (i + 1..items_len).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                Self::diff(&items[i], &items[j]) == 1
                    && Self::is_reflective(&items[i + 1..j])
                    && Some((j + i) / 2) != ignore
            })
            .find_map(|(i, j)| {
                let (mut ii, mut jj) = (i, j);

                loop {
                    if i != ii && jj != j && items[ii] != items[jj] {
                        return None;
                    }

                    if ii == 0 || jj == items_len - 1 {
                        return Some(((j + i) / 2 + 1) * factor);
                    }

                    ii -= 1;
                    jj += 1;
                }
            })
    }
}

fn solve<F>(input: impl AsRef<str>, f: F) -> usize
where
    F: Fn(&Frame) -> usize,
{
    input
        .as_ref()
        .split("\n\n")
        .map(Frame::new)
        .map(|frame| f(&frame))
        .sum()
}

fn main() {
    println!("Part 1: {}", solve(INPUT, Frame::find_mirror));
    println!("Part 2: {}", solve(INPUT, Frame::find_other_mirror));
}

#[cfg(test)]
mod tests {
    use crate::{solve, Frame};

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const INPUT2: &str = ".#..#.##.
##..####.
.#..#..#.
..##....#
.#..#..#.
.#..#.###
........#
......#.#
##..##.##
##..##.#.
##..##...
##..##.##
......#.#
........#
.#..#.###";

    const INPUT3: &str = "....###...#..##
#..####..##..##
.###..###.#.###
...####...#####
#..#..#..##..##
#.##..##.#.....
#........#.##..";

    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT, Frame::find_mirror), 405);
        assert_eq!(solve(INPUT2, Frame::find_mirror), 3)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT, Frame::find_other_mirror), 400);
        assert_eq!(solve(INPUT2, Frame::find_other_mirror), 1000);
        assert_eq!(solve(INPUT3, Frame::find_other_mirror), 5);
    }
}
