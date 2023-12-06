use std::{collections::HashSet, str::FromStr};

const INPUT: &str = include_str!("../input/day4.txt");

#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("Card ", "");
        let (id, nums) = s.split_once(':').unwrap();
        let (winning, yours) = nums.split_once(" | ").unwrap();

        let id = id.trim().parse().unwrap();
        let winning = winning
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let numbers = yours
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        Ok(Self {
            id,
            winning,
            numbers,
        })
    }
}

impl Card {
    fn get_winning_count(&self) -> usize {
        self.numbers.intersection(&self.winning).count()
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| l.parse::<Card>().ok())
        .map(|card| card.get_winning_count())
        .filter(|&winning_count| winning_count > 0)
        .map(|winning_count| 1 << (winning_count - 1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards_count = input.lines().count();
    let mut results = vec![0; cards_count];

    for line in input.lines() {
        let card = Card::from_str(line).unwrap();
        let winning_count = card.get_winning_count();

        results[card.id - 1] += 1;

        if winning_count > 0 {
            let count = results[card.id - 1];
            (0..winning_count).for_each(|id| results[card.id + id] += count);
        }
    }

    results.iter().sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        // Your code here
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), 30);
    }
}
