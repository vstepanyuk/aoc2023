use aoc::parse_nums;

const INPUT: &str = include_str!("../input/day6.txt");
fn main() {
    println!("Part 1: {}", solve(INPUT));
    println!("Part 2: {}", solve(INPUT.replace(' ', "")));
}

fn solve(input: impl AsRef<str>) -> u64 {
    let mut lines = input.as_ref().lines();
    let times: Vec<u64> = parse_nums(lines.next().unwrap());
    let distances: Vec<u64> = parse_nums(lines.next().unwrap());

    times.into_iter().zip(distances).map(calculate).product()
}

fn calculate((time, distance): (u64, u64)) -> u64 {
    let t = time as f32;
    let d = distance as f32;

    let mut x1 = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
    let mut x2 = (t + (t * t - 4.0 * d).sqrt()) / 2.0;

    if x1 == x1.ceil() {
        x1 += 1.0;
    }

    if x2 == x2.floor() {
        x2 -= 1.0;
    }

    x2.floor() as u64 - x1.ceil() as u64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT.replace(' ', "")), 71503);
    }
}
