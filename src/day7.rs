use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

const INPUT: &str = include_str!("../input/day7.txt");

#[derive(PartialEq, Eq)]
struct Part1;

#[derive(PartialEq, Eq)]
struct Part2;

#[derive(Debug, Clone)]
struct Card(char, Option<u32>);

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Card {}

impl Card {
    fn new(c: char) -> Self {
        Self(c, None)
    }

    fn joker_value(&self) -> u32 {
        self.1.unwrap_or_else(|| self.value())
    }

    fn value(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            c => c.to_digit(10).unwrap(),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn hashmap(&self) -> HashMap<&Card, u64> {
        self.cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        })
    }

    fn is32(&self) -> bool {
        let hm = self.hashmap();
        hm.values().contains(&3) && hm.values().contains(&2)
    }

    fn is5(&self) -> bool {
        self.cards.iter().all_equal()
    }

    fn is4(&self) -> bool {
        self.hashmap().values().contains(&4)
    }

    fn is3(&self) -> bool {
        !self.is32() && self.hashmap().values().contains(&3)
    }

    fn is22(&self) -> bool {
        self.hashmap().values().filter(|&&v| v == 2).count() == 2
    }

    fn is2(&self) -> bool {
        !self.is32() && !self.is22() && self.hashmap().values().contains(&2)
    }

    fn compare_cards(&self, other: &Self) -> Ordering {
        self.cards
            .iter()
            .zip(other.cards.iter())
            .map(|(a, b)| a.joker_value().cmp(&b.joker_value()))
            .find(|&o| o != Ordering::Equal)
            .unwrap()
    }

    fn replace_joker(&mut self) {
        let hm = self
            .hashmap()
            .into_iter()
            .map(|(k, v)| (k.0, v))
            .collect::<HashMap<_, _>>();

        for card in self.cards.iter_mut() {
            if card.0 == 'J' {
                let found = hm
                    .iter()
                    .filter(|(&card, _)| card != 'J')
                    .max_by_key(|(_, &count)| count);

                if let Some((c, _)) = found {
                    card.0 = *c;
                } else {
                    card.0 = 'A';
                }

                card.1 = Some(1);
            }
        }
    }
}

impl FromStr for Hand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: [Card; 5] = cards
            .chars()
            .take(5)
            .map(Card::new)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Hand {
            cards,
            bid: bid.parse()?,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is5() && !other.is5() {
            return Ordering::Greater;
        }

        if other.is5() && !self.is5() {
            return Ordering::Less;
        }

        if self.is4() && !other.is4() {
            return Ordering::Greater;
        }

        if other.is4() && !self.is4() {
            return Ordering::Less;
        }

        if self.is32() && !other.is32() {
            return Ordering::Greater;
        }

        if other.is32() && !self.is32() {
            return Ordering::Less;
        }

        if self.is3() && !other.is3() {
            return Ordering::Greater;
        }

        if other.is3() && !self.is3() {
            return Ordering::Less;
        }

        if self.is22() && !other.is22() {
            return Ordering::Greater;
        }

        if other.is22() && !self.is22() {
            return Ordering::Less;
        }

        if self.is2() && !other.is2() {
            return Ordering::Greater;
        }

        if other.is2() && !self.is2() {
            return Ordering::Less;
        }

        self.compare_cards(other)
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: impl AsRef<str>) -> u64 {
    input
        .as_ref()
        .lines()
        .map(|l| l.parse::<Hand>().unwrap())
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

fn part2(input: impl AsRef<str>) -> u64 {
    input
        .as_ref()
        .lines()
        .map(|l| {
            let mut hand = l.parse::<Hand>().unwrap();
            hand.replace_joker();
            hand
        })
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5905);
    }
}
