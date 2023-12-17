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

fn solve(grid: &Matrix<usize>, max_steps: usize, min_steps: Option<usize>) -> usize {
    dijkstra(
        &((0, 0), (0, 1), 0),
        |curr| {
            let mut states = vec![];

            if curr.2 < max_steps {
                let ((row, col), dir) = (curr.0, curr.1);
                let (new_row, new_col) = (row as isize + dir.0, col as isize + dir.1);

                if new_row >= 0 && new_col >= 0 {
                    let new_pos = (new_row as usize, new_col as usize);
                    if let Some(&cost) = grid.get(new_pos) {
                        states.push(((new_pos, curr.1, curr.2 + 1), cost));
                    }
                }
            }

            if min_steps.is_some() && curr.2 < min_steps.unwrap() {
                // PART 2
                return states;
            }

            let directions = match curr.1 {
                (0, _) => [(1, 0), (-1, 0)],
                (_, 0) => [(0, 1), (0, -1)],
                _ => unreachable!(),
            };

            for (dr, dc) in directions {
                let (row, col) = curr.0;
                let (nr, nc) = (row as isize + dr, col as isize + dc);

                if nr < 0 || nc < 0 {
                    continue;
                }

                if let Some(&cost) = grid.get((nr as usize, nc as usize)) {
                    states.push((((nr as usize, nc as usize), (dr, dc), 1), cost));
                }
            }

            states
        },
        |state| {
            state.0 == (grid.rows - 1, grid.columns - 1)
                && (min_steps.is_none() || state.2 >= min_steps.unwrap()) // PART 2
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
