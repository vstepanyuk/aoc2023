use indexmap::IndexMap;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/day15.txt");

#[derive(Debug)]
struct Action<'a> {
    label: &'a str,
    operation: Operation,
    box_id: usize,
}

#[derive(Debug)]
enum Operation {
    GoTo,
    FocalLength(usize),
}

#[derive(Debug, Default, Clone)]
struct Box<'a> {
    lenses: IndexMap<&'a str, usize>,
}

impl<'a> Action<'a> {
    fn new(s: &'a str) -> Self {
        let (label, value) = s.split_once(|c| matches!(c, '-' | '=')).unwrap();
        let operation = s.chars().find(|c| matches!(*c, '-' | '=')).unwrap();

        let operation = match operation {
            '-' => Operation::GoTo,
            '=' => Operation::FocalLength(value.parse().unwrap()),
            _ => unreachable!("Invalid operation '{}'", operation),
        };

        Self {
            label,
            operation,
            box_id: hash(label),
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[inline]
fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |hash, c| ((hash + *c as usize) * 17) % 256)
}

fn part1(input: impl AsRef<str>) -> usize {
    input.as_ref().trim().split(',').map(hash).sum()
}

fn part2(input: impl AsRef<str>) -> usize {
    let actions = input
        .as_ref()
        .trim()
        .split(',')
        .map(Action::new)
        .collect_vec();

    let mut boxes = vec![Box::default(); 265];
    for action in actions {
        match action.operation {
            Operation::GoTo => {
                boxes[action.box_id].lenses.shift_remove(action.label);
            }
            Operation::FocalLength(value) => {
                *boxes[action.box_id]
                    .lenses
                    .entry(action.label)
                    .or_insert(value) = value;
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .filter(|(_, Box { lenses, .. })| !lenses.is_empty())
        .flat_map(|(box_id, Box { lenses, .. })| {
            lenses
                .iter()
                .enumerate()
                .map(move |(idx, (_, &fp))| (box_id + 1) * (idx + 1) * fp)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 145);
    }
}
