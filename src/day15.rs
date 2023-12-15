use itertools::Itertools;

const INPUT: &str = include_str!("../input/day15.txt");

#[derive(Debug)]
struct Action<'a> {
    label: &'a str,
    operation: Operation,
    box_id: u8,
}

#[derive(Debug)]
enum Operation {
    GoTo,
    FocalLength(u8),
}

#[derive(Debug, Default, Clone)]
struct Box<'a> {
    lenses: Vec<(&'a str, u8)>,
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
            box_id: hash(label) as u8,
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[inline]
fn hash(s: &str) -> u64 {
    s.as_bytes()
        .iter()
        .fold(0, |hash, c| ((hash + *c as u64) * 17) % 256)
}

fn part1(input: impl AsRef<str>) -> u64 {
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
                if let Some((idx, _)) = boxes[action.box_id as usize]
                    .lenses
                    .iter()
                    .find_position(|&&(l, _)| l == action.label)
                {
                    boxes[action.box_id as usize].lenses.remove(idx);
                }
            }
            Operation::FocalLength(value) => {
                if let Some((idx, _)) = boxes[action.box_id as usize]
                    .lenses
                    .iter()
                    .find_position(|&&(l, _)| l == action.label)
                {
                    boxes[action.box_id as usize].lenses[idx].1 = value;
                } else {
                    boxes[action.box_id as usize]
                        .lenses
                        .push((action.label, value))
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .filter(|b| !b.1.lenses.is_empty())
        .map(|(idx, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(|(i, (_, v))| *v as usize * (i + 1) * (idx + 1))
                .sum::<usize>()
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
