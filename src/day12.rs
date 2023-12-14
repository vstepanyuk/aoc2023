use aoc::parse_nums;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day12.txt");

#[derive(Debug)]
struct Record {
    pattern: Vec<char>,
    rule: Vec<usize>,
}

impl Record {
    fn new(input: impl AsRef<str>) -> Self {
        let (pattern, rule) = input
            .as_ref()
            .split_once(' ')
            .map(|(pattern, rule)| (pattern.chars().collect(), parse_nums(rule)))
            .unwrap();

        Self { pattern, rule }
    }

    fn count(&self) -> usize {
        Self::count_inner(&self.pattern, &self.rule, &mut HashMap::new())
    }

    fn count_inner<'a>(
        pat: &'a [char],
        nums: &'a [usize],
        cache: &mut HashMap<(&'a [char], &'a [usize]), usize>,
    ) -> usize {
        if pat.is_empty() {
            return nums.is_empty() as usize;
        }

        if nums.is_empty() {
            return !pat.contains(&'#') as usize;
        }

        if let Some(&count) = cache.get(&(pat, nums)) {
            return count;
        }

        let mut result = 0;
        if matches!(pat[0], '.' | '?') {
            result += Self::count_inner(&pat[1..], nums, cache);
        }

        if matches!(pat[0], '#' | '?')
            && nums[0] <= pat.len()
            && !pat[..nums[0]].contains(&'.')
            && (nums[0] == pat.len() || pat[nums[0]] != '#')
        {
            let slice = if nums[0] + 1 >= pat.len() {
                &[]
            } else {
                &pat[nums[0] + 1..]
            };

            result += Self::count_inner(slice, &nums[1..], cache)
        }

        cache.insert((pat, nums), result);

        result
    }

    fn unfold(&self, times: usize) -> Self {
        let mut pattern = self.pattern.repeat(times);
        (1..times).for_each(|i| pattern.insert(i * self.pattern.len() + i - 1, '?'));
        let rule = self.rule.repeat(times);

        Self { pattern, rule }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: impl AsRef<str>) -> usize {
    input.as_ref().lines().map(|l| Record::new(l).count()).sum()
}

fn part2(input: impl AsRef<str>) -> usize {
    input
        .as_ref()
        .lines()
        .map(|l| Record::new(l).unfold(5).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, Record};

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(Record::new("???.### 1,1,3").count(), 1);
        assert_eq!(Record::new(".??..??...?##. 1,1,3").count(), 4);
        assert_eq!(Record::new("?#?#?#?#?#?#?#? 1,3,1,6").count(), 1);
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Record::new("???.### 1,1,3").unfold(5).count(), 1);
        assert_eq!(Record::new(".??..??...?##. 1,1,3").unfold(5).count(), 16384);
        assert_eq!(Record::new("?#?#?#?#?#?#?#? 1,3,1,6").unfold(5).count(), 1);
        assert_eq!(Record::new("????.#...#... 4,1,1").unfold(5).count(), 16);
        assert_eq!(
            Record::new("????.######..#####. 1,6,5").unfold(5).count(),
            2500
        );

        assert_eq!(Record::new("?###???????? 3,2,1").unfold(5).count(), 506250);
    }
}
