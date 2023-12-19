use std::str::FromStr;

use indexmap::IndexMap;

use aoc::parse_nums;

const INPUT: &str = include_str!("../input/day19.txt");

#[derive(Debug, Clone)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rules) = s.split_once('{').unwrap();
        let rules = rules[..rules.len() - 1]
            .split(',')
            .map(|line| line.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            id: id.to_string(),
            rules,
        })
    }
}

impl Workflow {
    fn count(&self, workflow: &IndexMap<String, Workflow>, counts: [(usize, usize); 4]) -> usize {
        self.rules
            .iter()
            .scan(counts, |counts, rule| {
                let result = match (rule.condition, &rule.action) {
                    (None, Action::Accept) => {
                        counts.iter().map(|(min, max)| max - min + 1).product()
                    }
                    (None, Action::Move(id)) => workflow.get(id).unwrap().count(workflow, *counts),
                    (None, Action::Reject) => 0,
                    (Some((category, op, value)), action) => {
                        let mut rule_counts = *counts;
                        let current = counts[category];

                        rule_counts[category] = match op {
                            '<' => (current.0, current.1.min(value - 1)),
                            '>' => (current.0.max(value + 1), current.1),
                            _ => unreachable!(),
                        };

                        let result = match action {
                            Action::Accept => {
                                rule_counts.iter().map(|(min, max)| max - min + 1).product()
                            }
                            Action::Move(id) => {
                                workflow.get(id).unwrap().count(workflow, rule_counts)
                            }
                            Action::Reject => 0,
                        };

                        counts[category] = match op {
                            '>' => (current.0, current.1.min(value)),
                            '<' => (current.0.max(value), current.1),
                            _ => unreachable!(),
                        };

                        result
                    }
                };

                Some(result)
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Rule {
    action: Action,
    condition: Option<(usize, char, usize)>,
}

impl Rule {
    fn check(&self, parts: &[usize; 4]) -> bool {
        match &self.condition {
            Some((category, op, value)) => match op {
                '<' => parts[*category] < *value,
                '>' => parts[*category] > *value,
                _ => unreachable!(),
            },
            None => true,
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, condition) = match s.split_once(':') {
            Some((condition, action)) => {
                let mut chars = condition.chars();
                let category = Part::from_index(chars.next().unwrap());
                let op = chars.next().unwrap();
                let value = chars.as_str().parse::<usize>().unwrap();

                (action, Some((category, op, value)))
            }
            None => (s, None),
        };

        let action = match action {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::Move(action.to_string()),
        };

        Ok(Self { action, condition })
    }
}

#[derive(Debug)]
struct Part([usize; 4]);

impl Part {
    fn from_index(id: char) -> usize {
        match id {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        }
    }

    fn process(&self, workflows: &IndexMap<String, Workflow>) -> bool {
        let mut workflow = workflows.get("in").unwrap();

        loop {
            let rule = workflow
                .rules
                .iter()
                .find(|rule| rule.check(&self.0))
                .unwrap();

            match &rule.action {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Move(id) => {
                    workflow = workflows.get(id).unwrap();
                }
            }
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = parse_nums::<Vec<usize>, _>(s);
        Ok(Self([nums[0], nums[1], nums[2], nums[3]]))
    }
}

#[derive(Debug, Clone)]
enum Action {
    Accept,
    Reject,
    Move(String),
}

fn parse(input: &str) -> (IndexMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let workflow: Workflow = line.parse().unwrap();
            (workflow.id.clone(), workflow)
        })
        .collect();

    let parts = parts.lines().map(|line| line.parse().unwrap()).collect();

    (workflows, parts)
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|part| part.process(&workflows))
        .map(|part| part.0.iter().sum::<usize>())
        .sum()
}

fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input);

    workflows["in"].count(&workflows, [(1, 4000); 4])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 167409079868000);
    }
}
