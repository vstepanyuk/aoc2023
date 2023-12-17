use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use pathfinding::matrix::Matrix;

const INPUT: &str = include_str!("../input/day16.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    solve(
        &Matrix::from_iter(input.lines().map(|line| line.chars().collect::<Vec<_>>())),
        ((0, 0), (0, 1)),
    )
}

fn part2(input: &str) -> usize {
    let matrix = Matrix::from_iter(input.lines().map(|line| line.chars().collect::<Vec<_>>()));

    let mut initial = vec![];
    for r in 0..matrix.rows {
        initial.push(((r, 0), (0, 1)));
        initial.push(((r, matrix.columns - 1), (0, -1)));
    }

    for c in 0..matrix.columns {
        initial.push(((0, c), (1, 0)));
        initial.push(((matrix.rows - 1, c), (-1, 0)));
    }

    initial
        .into_iter()
        .map(|item| solve(&matrix, item))
        .max()
        .unwrap()
}

fn solve(matrix: &Matrix<char>, initial: ((usize, usize), (i8, i8))) -> usize {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(initial);

    while let Some(((r, c), d)) = queue.pop_front() {
        if !visited.insert(((r, c), d)) {
            continue;
        }

        let current = matrix.get((r, c)).unwrap();

        match (d, current) {
            ((0, 1), '.' | '-') | ((1, 0), '\\') | ((-1, 0), '/') => {
                if c + 1 < matrix.columns {
                    queue.push_back(((r, c + 1), (0, 1)));
                }
            }
            ((0, -1), '.' | '-') | ((1, 0), '/') | ((-1, 0), '\\') => {
                if c > 0 {
                    queue.push_back(((r, c - 1), (0, -1)));
                }
            }
            ((1, 0), '|' | '.') | ((0, -1), '/') | ((0, 1), '\\') => {
                if r + 1 < matrix.rows {
                    queue.push_back(((r + 1, c), (1, 0)));
                }
            }
            ((-1, 0), '|' | '.') | ((0, -1), '\\') | ((0, 1), '/') => {
                if r > 0 {
                    queue.push_back(((r - 1, c), (-1, 0)));
                }
            }
            // Split
            ((0, _), '|') => {
                if r + 1 < matrix.rows {
                    queue.push_back(((r + 1, c), (1, 0)));
                }
                if r > 0 {
                    queue.push_back(((r - 1, c), (-1, 0)));
                }
            }
            ((_, 0), '-') => {
                if c + 1 < matrix.columns {
                    queue.push_back(((r, c + 1), (0, 1)));
                }
                if c > 0 {
                    queue.push_back(((r, c - 1), (0, -1)));
                }
            }
            _ => {
                unreachable!("Invalid direction {:?} ({})", d, current);
            }
        }
    }

    visited.iter().unique_by(|(pos, _)| pos).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 51);
    }
}
