use aoc::Point2D;
use itertools::Itertools;
use pathfinding::matrix::Matrix;

const INPUT: &str = include_str!("../input/day11.txt");

fn main() {
    println!("Part 1: {}", solve(INPUT, 1));
    println!("Part 2: {}", solve(INPUT, 1_000_000 - 1));
}

fn solve(input: impl AsRef<str>, gap_size: usize) -> usize {
    let matrix = Matrix::from_iter(input.as_ref().lines().map(|line| line.chars()));

    let galaxies = matrix
        .items()
        .filter(|&(_, &c)| (c == '#'))
        .map(|(pos, _)| Point2D::new(pos.1, pos.0))
        .collect::<Vec<_>>();

    let empty_rows = (0..matrix.rows)
        .filter(|&row| (0..matrix.columns).all(|col| matrix.get((row, col)) == Some(&'.')))
        .collect::<Vec<_>>();

    let empty_cols = (0..matrix.columns)
        .filter(|&col| (0..matrix.rows).all(|row| matrix.get((row, col)) == Some(&'.')))
        .collect::<Vec<_>>();

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            pair[0].manhattan(pair[1])
                + gap_size
                    * (gaps(pair[0].x, pair[1].x, &empty_cols)
                        + gaps(pair[0].y, pair[1].y, &empty_rows))
        })
        .sum()
}

#[inline]
fn gaps(a: usize, b: usize, empty: &[usize]) -> usize {
    let (min, max) = (a.min(b), a.max(b));
    empty.iter().filter(|&&pos| min < pos && pos < max).count()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT, 1), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT, 10 - 1), 1030);
        assert_eq!(solve(INPUT, 100 - 1), 8410);
    }
}
