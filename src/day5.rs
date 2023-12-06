use std::str::FromStr;

const INPUT: &str = include_str!("../input/day5.txt");

type Element = u64;

fn main() {
    println!("Day 5, part 1: {}", part1(INPUT));
    println!("Day 5, part 2: {}", part2(INPUT));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    dst: Element,
    src: Element,
    len: Element,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let dst = iter.next().unwrap().parse::<Element>().unwrap();
        let src = iter.next().unwrap().parse::<Element>().unwrap();
        let len = iter.next().unwrap().parse::<Element>().unwrap();

        Ok(Map { dst, src, len })
    }
}

impl Map {
    fn to_dst(&self, input: Element) -> Option<Element> {
        if input < self.src || input >= self.src + self.len {
            return None;
        }

        Some(self.dst + (input - self.src))
    }
}

#[derive(Debug, Clone)]
struct Category(Vec<Map>);

impl Category {
    fn to_dst(&self, input: Element) -> Element {
        let found = self.0.iter().find_map(|map| map.to_dst(input));
        found.unwrap_or(input)
    }
}

fn parse(input: &str) -> (Vec<Element>, Vec<Category>) {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|item| item.parse::<Element>().ok())
        .collect::<Vec<_>>();

    let mut categories = vec![];
    let mut category = Category(vec![]);

    for line in lines {
        if line.contains(':') || line.is_empty() {
            if !category.0.is_empty() {
                categories.push(category.clone());
                category.0.clear();
            }
            continue;
        }

        let map = line.parse::<Map>().unwrap();
        category.0.push(map);
    }

    categories.push(category);

    (seeds, categories)
}

fn part1(input: &str) -> Element {
    let (seeds, categories) = parse(input);

    seeds
        .into_iter()
        .map(|seed| {
            categories
                .iter()
                .fold(seed, |input, category| category.to_dst(input))
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> Element {
    let (seeds, categories) = parse(input);

    seeds
        .chunks(2)
        .flat_map(|r| r[0]..r[0] + r[1])
        .map(|seed| {
            categories
                .iter()
                .fold(seed, |input, category| category.to_dst(input))
        })
        .min()
        .unwrap()
}
// Your code here

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
