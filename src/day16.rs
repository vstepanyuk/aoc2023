use std::collections::HashSet;

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
    let grid = Matrix::from_iter(input.lines().map(|line| line.chars().collect::<Vec<_>>()));

    (0..grid.rows)
        .flat_map(|row| [((row, 0), LEFT), ((row, grid.columns - 1), RIGHT)])
        .chain(
            (0..grid.columns)
                .flat_map(|column| [((0, column), DOWN), ((grid.rows - 1, column), UP)]),
        )
        .map(|item| solve(&grid, item))
        .max()
        .unwrap()
}

fn solve(grid: &Matrix<char>, initial: ((usize, usize), (i8, i8))) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![initial];

    while let Some(((row, col), dir)) = queue.pop() {
        let current = match grid.get((row, col)) {
            Some(ch) if visited.insert(((row, col), dir)) => ch,
            _ => continue,
        };

        match (dir, current) {
            (LEFT, '.' | '-') | (DOWN, '/') | (UP, '\\') | ((_, 0), '-') if col > 0 => {
                queue.push(((row, col - 1), LEFT));
                if dir.1 == 0 && current == &'-' {
                    queue.push(((row, col + 1), RIGHT));
                }
            }
            (UP, '|' | '.') | (LEFT, '\\') | (RIGHT, '/') | ((0, _), '|') if row > 0 => {
                queue.push(((row - 1, col), UP));

                if dir.0 == 0 && current == &'|' {
                    queue.push(((row + 1, col), DOWN));
                }
            }
            (DOWN, '|' | '.') | (LEFT, '/') | (RIGHT, '\\') | ((0, _), '|') => {
                queue.push(((row + 1, col), DOWN));
            }
            (RIGHT, '.' | '-') | (DOWN, '\\') | (UP, '/') | ((_, 0), '-') => {
                queue.push(((row, col + 1), RIGHT));
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
