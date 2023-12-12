use std::collections::{HashSet, VecDeque};

use pathfinding::matrix::Matrix;

const INPUT: &str = include_str!("../input/day10.txt");

struct Grid(Matrix<char>);

impl Grid {
    fn new(input: impl AsRef<str>) -> Self {
        Self(Matrix::from_iter(
            input.as_ref().lines().map(|line| line.chars()),
        ))
    }

    fn find_path(&self) -> (char, Vec<(usize, usize)>) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(self.find_start());
        visited.insert(self.find_start());

        let mut potential_s = HashSet::from(['|', 'J', 'L', '7', 'F', '-']);

        while let Some(pos) = queue.pop_front() {
            let (row, col) = pos;
            let ch = *self.0.get(pos).unwrap();

            if row > 0
                && matches!(ch, 'S' | '|' | 'J' | 'L')
                && matches!(self.0[(row - 1, col)], '|' | '7' | 'F')
                && !visited.contains(&(row - 1, col))
            {
                queue.push_back((row - 1, col));
                visited.insert((row - 1, col));

                if ch == 'S' {
                    ['-', '7', 'F']
                        .iter()
                        .for_each(|&c| _ = potential_s.remove(&c));
                }
            }

            if row < self.0.rows - 1
                && matches!(ch, 'S' | '|' | '7' | 'F')
                && matches!(self.0[(row + 1, col)], '|' | 'J' | 'L')
                && !visited.contains(&(row + 1, col))
            {
                queue.push_back((row + 1, col));
                visited.insert((row + 1, col));

                if ch == 'S' {
                    ['-', 'J', 'L']
                        .iter()
                        .for_each(|&c| _ = potential_s.remove(&c));
                }
            }

            if col > 0
                && matches!(ch, 'S' | '-' | 'J' | '7')
                && matches!(self.0[(row, col - 1)], '-' | 'L' | 'F')
                && !visited.contains(&(row, col - 1))
            {
                queue.push_back((row, col - 1));
                visited.insert((row, col - 1));

                if ch == 'S' {
                    ['|', 'L', 'F']
                        .iter()
                        .for_each(|&c| _ = potential_s.remove(&c));
                }
            }

            if col < self.0.columns - 1
                && matches!(ch, 'S' | '-' | 'L' | 'F')
                && matches!(self.0[(row, col + 1)], '-' | 'J' | '7')
                && !visited.contains(&(row, col + 1))
            {
                queue.push_back((row, col + 1));
                visited.insert((row, col + 1));

                if ch == 'S' {
                    ['|', 'J', '7']
                        .iter()
                        .for_each(|&c| _ = potential_s.remove(&c));
                }
            }
        }

        let s = potential_s.into_iter().next().unwrap();
        (s, visited.into_iter().collect())
    }

    fn find_start(&self) -> (usize, usize) {
        self.0.items().find(|&(_, c)| *c == 'S').unwrap().0
    }

    #[allow(dead_code)]
    fn draw(&self, path: &[(usize, usize)]) -> Self {
        let mut new_matrix = self.0.clone();

        for p in path.iter() {
            let ch = new_matrix.get_mut(*p).unwrap();

            new_matrix[*p] = match *ch {
                'J' => '┘',
                'L' => '└',
                'F' => '┌',
                '7' => '┐',
                '-' => '─',
                '|' => '│',
                _ => *ch,
            };
        }

        new_matrix.items_mut().for_each(|(pos, ch)| {
            if !path.contains(&pos) {
                *ch = '.';
            }
        });

        Self(new_matrix)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.0.rows {
            for col in 0..self.0.columns {
                print!("{}", self.0[(row, col)]);
            }
            println!();
        }
    }

    fn outside(&mut self, path: &[(usize, usize)]) -> HashSet<(usize, usize)> {
        self.0.items_mut().for_each(|(pos, ch)| {
            if !path.contains(&pos) {
                *ch = '.';
            }
        });

        let mut outside = HashSet::new();

        for row in 0..self.0.rows {
            let mut within = false;
            let mut up = false;

            for col in 0..self.0.columns {
                let ch = self.0[(row, col)];

                match ch {
                    '|' => {
                        within = !within;
                    }
                    'L' | 'F' => {
                        up = ch == 'L';
                    }
                    '7' | 'J' => {
                        if matches!((ch, up), ('7', true) | ('J', false)) {
                            within = !within;
                        }
                        up = false;
                    }
                    '.' | '-' => {}
                    _ => {
                        unreachable!("Unknown character: {}", ch)
                    }
                }

                if !within {
                    outside.insert((row, col));
                }
            }
        }

        outside
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: impl AsRef<str>) -> usize {
    Grid::new(input).find_path().1.len() / 2
}

fn part2(input: impl AsRef<str>) -> usize {
    let mut grid = Grid::new(input);
    let (s, path) = grid.find_path();

    let start = grid.find_start();
    grid.0[start] = s;

    let outside = grid.outside(&path);

    grid.0.rows * grid.0.columns
        - outside.len()
        - path.iter().filter(|p| !outside.contains(p)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            8
        );

        assert_eq!(
            part1(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            4
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );

        assert_eq!(
            part2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );

        assert_eq!(
            part2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }
}
