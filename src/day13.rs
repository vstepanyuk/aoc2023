use num::Integer;

const INPUT: &str = include_str!("../input/day13.txt");

struct Frame {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl Frame {
    fn new(input: impl AsRef<str>) -> Self {
        let rows = input
            .as_ref()
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let cols = (0..rows[0].len())
            .map(|col| rows.iter().map(|row| row[col]).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { rows, cols }
    }

    fn mirror_value(&self) -> usize {
        self.find(&self.rows, 0, None)
            .map(|idx| (idx + 1) * 100)
            .or_else(|| self.find(&self.cols, 0, None).map(|idx| idx + 1))
            .unwrap_or_default()
    }

    #[inline]
    fn diff<'a, T: PartialEq>(a: &'a [T], b: &'a [T]) -> usize {
        a.iter().zip(b.iter()).filter(|(a, b)| a != b).count()
    }

    #[inline]
    fn is_reflective<T: PartialEq>(s: &[T]) -> bool {
        s.len().is_even() && s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
    }

    fn mirror_value_with_smudge(&self) -> usize {
        self.find(&self.rows, 1, self.find(&self.rows, 0, None))
            .map(|idx| (idx + 1) * 100)
            .or_else(|| {
                self.find(&self.cols, 1, self.find(&self.cols, 0, None))
                    .map(|idx| idx + 1)
            })
            .unwrap_or_default()
    }

    fn find<'a, T: 'a + PartialEq>(
        &'a self,
        items: &[Vec<T>],
        allowed_diff: usize,
        ignore: Option<usize>,
    ) -> Option<usize> {
        let items_len = items.len();

        (0..items_len)
            .flat_map(|i| (i + 1..items_len).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                Self::diff(&items[i], &items[j]) == allowed_diff
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
                        return Some((j + i) / 2);
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
    println!("Part 1: {}", solve(INPUT, Frame::mirror_value));
    println!("Part 2: {}", solve(INPUT, Frame::mirror_value_with_smudge));
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
        assert_eq!(solve(INPUT, Frame::mirror_value), 405);
        assert_eq!(solve(INPUT2, Frame::mirror_value), 3)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT, Frame::mirror_value_with_smudge), 400);
        assert_eq!(solve(INPUT2, Frame::mirror_value_with_smudge), 1000);
        assert_eq!(solve(INPUT3, Frame::mirror_value_with_smudge), 5);
    }
}
