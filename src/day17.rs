use pathfinding::matrix::Matrix;
use pathfinding::prelude::*;

const INPUT: &str = include_str!("../input/day17.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn parse<T: TryFrom<u32>>(input: &str) -> Matrix<T> {
    Matrix::from_iter(input.lines().map(|line| {
        line.chars()
            .flat_map(|c| c.to_digit(10).unwrap().try_into().ok())
    }))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    steps: usize,
}

impl State {
    fn new(position: (usize, usize), direction: (isize, isize), steps: usize) -> Self {
        Self {
            position,
            direction,
            steps,
        }
    }
}

fn solve(grid: &Matrix<usize>, max_steps: usize, min_steps: Option<usize>) -> usize {
    dijkstra(
        &State::new((0, 0), (0, 1), 0),
        |s| {
            let mut states = vec![];

            if s.steps < max_steps {
                let next_row = s.position.0 as isize + s.direction.0;
                let next_column = s.position.1 as isize + s.direction.1;

                if next_row >= 0 && next_column >= 0 {
                    let next = (next_row as usize, next_column as usize);
                    if let Some(&cost) = grid.get(next) {
                        states.push((State::new(next, s.direction, s.steps + 1), cost));
                    }
                }
            }

            if min_steps.is_some() && s.steps < min_steps.unwrap() {
                return states;
            }

            let directions = match s.direction {
                (0, _) => [(1, 0), (-1, 0)],
                (_, 0) => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for dir in directions {
                let next_row = s.position.0 as isize + dir.0;
                let next_column = s.position.1 as isize + dir.1;

                if next_row < 0 || next_column < 0 {
                    continue;
                }

                if let Some(&cost) = grid.get((next_row as usize, next_column as usize)) {
                    states.push((
                        State::new((next_row as usize, next_column as usize), dir, 1),
                        cost,
                    ));
                }
            }

            states
        },
        |state| {
            state.position == (grid.rows - 1, grid.columns - 1)
                && (min_steps.is_none() || state.steps >= min_steps.unwrap())
        },
    )
    .unwrap()
    .1
}

fn part1(input: &str) -> usize {
    solve(&parse::<usize>(input), 3, None)
}

fn part2(input: &str) -> usize {
    solve(&parse::<usize>(input), 10, Some(4))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 94);
        assert_eq!(
            part2(
                "111111111111
999999999991
999999999991
999999999991
999999999991"
            ),
            71
        );
    }
}
