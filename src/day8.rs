use std::collections::HashMap;

use num::integer::lcm;

const INPUT: &str = include_str!("../input/day8.txt");

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (instructions, rules) = parse(input);
    calc_steps("AAA", &instructions, &rules, |c| c != "ZZZ")
}

fn part2(input: &str) -> usize {
    let (instructions, rules) = parse(input);

    let mut result = rules
        .keys()
        .filter(|&k| k.ends_with('A'))
        .map(|&k| calc_steps(k, &instructions, &rules, |c| !c.ends_with('Z')));

    vec_lcm(&mut result)
}

fn calc_steps<F>(
    current: &str,
    instructions: &[char],
    rules: &HashMap<&str, [&str; 2]>,
    f: F,
) -> usize
where
    F: Fn(&str) -> bool,
{
    (0..)
        .zip(instructions.iter().cycle())
        .scan(current, |curr, (_, c)| {
            *curr = rules.get(curr).unwrap()[(c == &'R') as usize];
            f(curr).then_some(*curr)
        })
        .count()
        + 1
}

fn vec_lcm(iter: &mut impl Iterator<Item = usize>) -> usize {
    let first = iter.next().unwrap();
    iter.fold(first, lcm)
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();

    let rules = lines
        .map(|line| {
            let (key, parts) = line.split_once(" = ").unwrap();
            let (left, right) = parts
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();

            (key, [left, right])
        })
        .collect();

    (instructions, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
