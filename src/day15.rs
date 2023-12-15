use std::iter::repeat;

use indexmap::IndexMap;

const INPUT: &str = include_str!("../input/day15.txt");

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
    input
        .as_ref()
        .split(',')
        .map(|s| {
            let (label, value) = s.split_once(|c| matches!(c, '-' | '=')).unwrap();
            let operation = s.chars().find(|c| matches!(*c, '-' | '=')).unwrap();
            let op = (operation == '=').then(|| value.parse::<usize>().unwrap());

            (label, hash(label), op)
        })
        .fold(
            &mut Vec::from_iter(repeat(IndexMap::new()).take(256)),
            |boxes, action| {
                match action {
                    (label, box_id, None) => {
                        boxes[box_id].shift_remove(label);
                    }
                    (label, box_id, Some(value)) => {
                        *boxes[box_id].entry(label).or_insert(value) = value;
                    }
                }

                boxes
            },
        )
        .iter()
        .enumerate()
        .flat_map(|(box_id, lenses)| {
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
