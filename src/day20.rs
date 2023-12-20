use std::collections::HashMap;
use std::collections::VecDeque;

use itertools::Itertools;
use num::integer::lcm;

const INPUT: &str = include_str!("../input/day20.txt");

type Modules<'a> = HashMap<&'a str, Module<'a>>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Module<'a> {
    FlipFlop {
        id: &'a str,
        state: bool,
        outputs: Vec<&'a str>,
    },
    Conjunction {
        id: &'a str,
        state: HashMap<&'a str, bool>,
        outputs: Vec<&'a str>,
    },
    Untyped {
        id: &'a str,
        outputs: Vec<&'a str>,
    },
}

impl<'a> Module<'a> {
    fn outputs(&self) -> &[&'a str] {
        match self {
            Self::FlipFlop { outputs, .. } => outputs,
            Self::Conjunction { outputs, .. } => outputs,
            Self::Untyped { outputs, .. } => outputs,
        }
    }
}

fn parse_modules(input: &str) -> Modules {
    let mut modules = Modules::new();

    for line in input.lines() {
        let (id, output) = line.split_once(" -> ").unwrap();
        let outputs = output.split(", ").collect();
        let mode = id.chars().next().unwrap();
        let id = id.trim_start_matches(|c| c == '%' || c == '&');

        let module = match mode {
            '%' => Module::FlipFlop {
                id,
                state: false,
                outputs,
            },
            '&' => Module::Conjunction {
                id,
                state: Default::default(),
                outputs,
            },
            _ => Module::Untyped { id, outputs },
        };

        modules.insert(id, module);
    }

    let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();
    for (id, module) in modules.iter() {
        module
            .outputs()
            .iter()
            .filter(|id| matches!(modules.get(*id), Some(Module::Conjunction { .. })))
            .for_each(|&conjunction_id| {
                deps.entry(conjunction_id).or_default().push(id);
            });
    }

    for (id, deps) in deps.iter() {
        let conjunction = modules.get_mut(id).unwrap();
        if let Module::Conjunction { state, .. } = conjunction {
            for dep in deps {
                state.insert(dep, false);
            }
        }
    }

    modules
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut modules = parse_modules(input);
    let mut counts = [0, 0];

    for _ in 0..1000 {
        push_button(&mut modules, &mut counts);
    }

    counts.iter().product()
}

fn part2(input: &str) -> usize {
    let mut modules = parse_modules(input);
    let mut counts = HashMap::new();

    for i in 1..10_000 {
        push_button2(&mut modules, i, &mut counts);
    }

    loop {
        let mut changed = false;
        for (id, module) in modules.iter() {
            if let Module::Conjunction { .. } = module {
                let deps = find_deps(id, &modules);
                if !counts.contains_key(&(id, true))
                    && deps
                        .iter()
                        .all(|dep_id| counts.contains_key(&(dep_id, false)))
                {
                    let steps = deps
                        .iter()
                        .map(|dep_id| *counts.get(&(dep_id, false)).unwrap())
                        .min()
                        .unwrap();

                    counts.insert((id, true), steps);
                    changed = true;
                }

                if !counts.contains_key(&(id, false))
                    && deps
                        .iter()
                        .all(|dep_id| counts.contains_key(&(dep_id, true)))
                {
                    let steps = deps
                        .iter()
                        .map(|dep_id| *counts.get(&(dep_id, true)).unwrap())
                        .collect_vec();

                    counts.insert((id, false), vec_lcm(&steps));
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let deps = find_deps("rx", &modules);
    let steps = deps
        .iter()
        .map(|dep_id| *counts.get(&(dep_id, false)).unwrap())
        .collect_vec();

    vec_lcm(&steps)
}

fn vec_lcm(arr: &[usize]) -> usize {
    let mut iter = arr.iter();
    let first = *iter.next().unwrap();
    iter.fold(first, |acc, curr| lcm(acc, *curr))
}

fn push_button(modules: &mut Modules, counts: &mut [usize; 2]) {
    let mut queue = VecDeque::new();
    queue.push_back(("broadcaster", false, "button"));

    while let Some((id, pulse, from)) = queue.pop_front() {
        counts[pulse as usize] += 1;

        let Some(module) = modules.get_mut(id) else {
            continue;
        };
        let outputs = module.outputs().to_vec();

        let next = match module {
            Module::FlipFlop { id, state, .. } if !pulse => {
                *state = !*state;

                outputs
                    .iter()
                    .map(|output_id| (*output_id, *state, *id))
                    .collect()
            }
            Module::Conjunction { id, state, .. } => {
                state.insert(from, pulse);
                let pulse = !state.values().all(|s| *s);

                outputs
                    .iter()
                    .map(|output_id| (*output_id, pulse, *id))
                    .collect()
            }
            Module::Untyped { id, .. } => outputs
                .iter()
                .map(|output_id| (*output_id, false, *id))
                .collect(),

            _ => {
                vec![]
            }
        };
        queue.extend(next);
    }
}

fn find_deps<'a>(id: &'a str, modules: &'a Modules) -> Vec<&'a str> {
    modules
        .iter()
        .filter(|(_, module)| module.outputs().iter().any(|output_id| *output_id == id))
        .map(|(id, _)| *id)
        .collect()
}

fn push_button2<'a>(
    modules: &mut Modules<'a>,
    index: usize,
    states: &mut HashMap<(&'a str, bool), usize>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(("broadcaster", false, "button"));

    while let Some((id, pulse, from)) = queue.pop_front() {
        let Some(module) = modules.get_mut(id) else {
            continue;
        };
        let outputs = module.outputs().to_vec();

        let next = match module {
            Module::FlipFlop { id, state, .. } if !pulse => {
                *state = !*state;

                states.entry((id, *state)).or_insert(index);

                outputs
                    .iter()
                    .map(|output_id| (*output_id, *state, *id))
                    .collect()
            }
            Module::Conjunction { id, state, .. } => {
                state.insert(from, pulse);
                let pulse = !state.values().all(|s| *s);

                states.entry((id, pulse)).or_insert(index);

                outputs
                    .iter()
                    .map(|output_id| (*output_id, pulse, *id))
                    .collect()
            }
            Module::Untyped { id, .. } => outputs
                .iter()
                .map(|output_id| (*output_id, false, *id))
                .collect(),

            _ => {
                vec![]
            }
        };
        queue.extend(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            ),
            32000000
        );

        assert_eq!(
            part1(
                "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            ),
            11687500
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 207652583562007);
    }
}
