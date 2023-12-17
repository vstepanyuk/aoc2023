use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use pathfinding::matrix::Matrix;

const INPUT: &str = include_str!("../input/day16.txt");

const LEFT: (i8, i8) = (0, -1);
const RIGHT: (i8, i8) = (0, 1);
const UP: (i8, i8) = (-1, 0);
const DOWN: (i8, i8) = (1, 0);

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    solve(
        &Matrix::from_iter(input.lines().map(|line| line.chars().collect::<Vec<_>>())),
        ((0, 0), RIGHT),
    )
}

fn part2(input: &str) -> usize {
    let matrix = Matrix::from_iter(input.lines().map(|line| line.chars().collect::<Vec<_>>()));

    (0..matrix.rows)
        .flat_map(|row| [((row, 0), LEFT), ((row, matrix.columns - 1), RIGHT)])
        .chain(
            (0..matrix.columns)
                .flat_map(|column| [((0, column), DOWN), ((matrix.rows - 1, column), UP)]),
        )
        .map(|item| solve(&matrix, item))
        .max()
        .unwrap()
}

fn solve(matrix: &Matrix<char>, initial: ((usize, usize), (i8, i8))) -> usize {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(initial);

    while let Some(((r, c), d)) = queue.pop_front() {
        let current = match matrix.get((r, c)) {
            Some(ch) if visited.insert(((r, c), d)) => ch,
            _ => continue,
        };

        match (d, current) {
            (LEFT, '.' | '-') | (DOWN, '/') | (UP, '\\') | ((_, 0), '-') if c > 0 => {
                queue.push_back(((r, c - 1), LEFT));
                if matches!((d, current), ((_, 0), '-')) {
                    queue.push_back(((r, c + 1), RIGHT));
                }
            }
            (UP, '|' | '.') | (LEFT, '\\') | (RIGHT, '/') | ((0, _), '|') if r > 0 => {
                queue.push_back(((r - 1, c), UP));

                if matches!((d, current), ((0, _), '|')) {
                    queue.push_back(((r + 1, c), DOWN));
                }
            }
            (DOWN, '|' | '.') | (LEFT, '/') | (RIGHT, '\\') | ((0, _), '|') => {
                queue.push_back(((r + 1, c), DOWN));
            }
            (RIGHT, '.' | '-') | (DOWN, '\\') | (UP, '/') | ((_, 0), '-') => {
                queue.push_back(((r, c + 1), RIGHT));
            }
            _ => {}
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
