use std::isize;

const INPUT: &str = include_str!("../input/day18.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    solve(input, |line| {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap().chars().last().unwrap();
        let steps = parts.next().unwrap().parse().unwrap();

        (direction, steps)
    })
}

fn part2(input: &str) -> usize {
    solve(input, |line| {
        let (_, hex) = line.trim_matches(')').split_once('#').unwrap();
        let direction = hex.chars().last().unwrap();
        let steps = isize::from_str_radix(&hex[0..hex.len() - 1], 16).unwrap();

        (direction, steps)
    })
}

fn solve<F>(input: &str, parser: F) -> usize
where
    F: Fn(&str) -> (char, isize),
{
    let items = input.lines().map(parser).collect::<Vec<(char, isize)>>();
    let mut map = vec![(0, 0)];

    for (direction, steps) in items.iter() {
        let (dx, dy) = match direction {
            'R' | '0' => (1, 0),
            'D' | '1' => (0, 1),
            'L' | '2' => (-1, 0),
            'U' | '3' => (0, -1),
            _ => unreachable!(),
        };

        let (x, y) = *map.last().unwrap();
        map.push((x + steps * dx, y + steps * dy));
    }

    let area = gauss_area(&map);
    let perimeter: usize = items.iter().map(|&(_, length)| length as usize).sum();

    area + perimeter / 2 + 1
}

fn gauss_area(vertices: &[(isize, isize)]) -> usize {
    let mut area = 0isize;
    let n = vertices.len();

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];

        area += (x1 + x2) * (y2 - y1);
    }

    area.unsigned_abs() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 952408144115);
    }
}
