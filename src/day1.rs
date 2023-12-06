const INPUT: &str = include_str!("../input/day1.txt");
const DIGITS_NAMED: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!("Part 1: {}", solve(INPUT, &[]));
    println!("Part 2: {}", solve(INPUT, &DIGITS_NAMED));
}

trait DigitContainer {
    type Owned;

    fn get_first_digit<T>(&self, replacement: &[T]) -> Option<u32>
    where
        T: AsRef<Self>;

    fn reverse(&self) -> Self::Owned;
}

impl DigitContainer for str {
    type Owned = String;

    fn get_first_digit<T>(&self, replacement: &[T]) -> Option<u32>
    where
        T: AsRef<Self>,
    {
        self.chars().enumerate().find_map(|(idx, c)| {
            if c.is_ascii_digit() {
                c.to_digit(10)
            } else {
                replacement
                    .iter()
                    .enumerate()
                    .find(|(_, ref word)| self[idx..].starts_with(word.as_ref()))
                    .map(|(idx, _)| (idx + 1) as u32)
            }
        })
    }

    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }
}

fn solve(input: &str, replacement: &[&str]) -> u32 {
    let reversed = replacement.iter().map(|&s| s.reverse()).collect::<Vec<_>>();

    input
        .lines()
        .map(|line| {
            line.get_first_digit::<&str>(replacement)
                .unwrap_or_default()
                * 10
                + line
                    .reverse()
                    .get_first_digit(&reversed[..])
                    .unwrap_or_default()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{solve, DIGITS_NAMED};

    #[test]
    fn test_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        assert_eq!(solve(input, &[]), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        assert_eq!(solve(input, &DIGITS_NAMED), 281);
    }
}
