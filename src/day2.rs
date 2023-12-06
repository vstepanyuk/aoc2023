use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input/day2.txt");

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

#[derive(Debug)]
struct GameSet(i32, i32, i32);

impl FromStr for GameSet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut r, mut g, mut b) = (0, 0, 0);

        for item in s.split(", ") {
            let (num, color) = item.split_once(' ').unwrap();
            let num = num.parse::<i32>()?;

            match color {
                "red" => r = num,
                "green" => g = num,
                "blue" => b = num,
                _ => unreachable!(),
            }
        }

        Ok(GameSet(r, g, b))
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");

        let id = iter
            .next()
            .unwrap()
            .split_once(' ')
            .ok_or(())?
            .1
            .parse::<i32>()
            .unwrap();

        let sets = iter
            .next()
            .unwrap()
            .split("; ")
            .filter_map(|set| set.parse::<GameSet>().ok())
            .collect::<Vec<_>>();

        Ok(Game { id, sets })
    }
}

fn main() {
    println!("Day 2, Part 1: {}", part1(INPUT));
    println!("Day 2, Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input
        .split('\n')
        .filter_map(|game| game.parse::<Game>().ok())
        .filter_map(|game| {
            if game
                .sets
                .iter()
                .all(|set| set.0 <= 12 && set.1 <= 13 && set.2 <= 14)
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .split('\n')
        .filter_map(|game| game.parse::<Game>().ok())
        .filter_map(|game| {
            let r = game.sets.iter().map(|set| set.0).max().unwrap();
            let g = game.sets.iter().map(|set| set.1).max().unwrap();
            let b = game.sets.iter().map(|set| set.2).max().unwrap();

            if r > 0 && g > 0 && b > 0 {
                Some(r * g * b)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
