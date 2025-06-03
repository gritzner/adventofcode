use std::collections::BTreeMap;

fn letter_to_index(c: char) -> usize {
    match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => panic!("invalid input"),
    }
}

enum Condition {
    True,
    LessThan(usize, usize),
    GreaterThan(usize, usize),
}

impl Condition {
    fn new(condition: &str) -> Self {
        let index = letter_to_index(condition.chars().next().expect("invalid puzzle input"));
        let threshold: usize = condition.get(2..).expect("invalid puzzle input").parse().expect("failed to parse threshold");
        match condition.chars().nth(1).expect("invalid puzzle input") {
            '<' => Self::LessThan(index, threshold),
            '>' => Self::GreaterThan(index, threshold),
            _ => panic!("invalid puzzle input"),
        }
    }

    fn accept(&self, v: &[usize; 4]) -> bool {
        match self {
            &Self::True => true,
            &Self::LessThan(index, threshold) => v[index] < threshold,
            &Self::GreaterThan(index, threshold) => v[index] > threshold,
        }
    }

    fn accepted_interval(&self, mut interval: ([usize; 4], [usize; 4])) -> Option<([usize; 4], [usize; 4])> {
        let index = match self {
            &Condition::True => return Some(interval),
            &Condition::LessThan(index, threshold) => {
                interval.1[index] = threshold - 1;
                index
            },
            &Condition::GreaterThan(index, threshold) => {
                interval.0[index] = threshold + 1;
                index
            },
        };
        if interval.0[index] <= interval.1[index] {
            Some(interval)
        } else {
            None
        }
    }
    
    fn rejected_interval(&self, mut interval: ([usize; 4], [usize; 4])) -> Option<([usize; 4], [usize; 4])> {
        let index = match self {
            &Condition::True => return None,
            &Condition::LessThan(index, threshold) => {
                interval.0[index] = threshold;
                index
            },
            &Condition::GreaterThan(index, threshold) => {
                interval.1[index] = threshold;
                index
            },
        };
        if interval.0[index] <= interval.1[index] {
            Some(interval)
        } else {
            None
        }
    }    
}

enum Target {
    Rule(String),
    Accept,
    Reject,
}

impl Target {
    fn new(target: &str) -> Self {
        if target == "A" {
            Self::Accept
        } else if target == "R" {
            Self::Reject
        } else {
            Self::Rule(target.to_string())
        }
    }
}

struct Rule {
    condition: Condition,
    target: Target,
}

impl Rule {
    fn new(rule: &str) -> Self {
        if let Some(i) = rule.find(":") {
            let (condition, target) = rule.split_at(i);
            let target = target.get(1..).unwrap();
            Self {
                condition: Condition::new(condition),
                target: Target::new(target),
            }
        } else {
            let rule = rule.get(..rule.len()-1).unwrap();
            Self {
                condition: Condition::True,
                target: Target::new(rule),
            }
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle: Vec<_> = puzzle.split("\n\n").collect();
    debug_assert!(puzzle.len() == 2);

    let mut rules = BTreeMap::new();
    for rule in puzzle[0].split("\n") {
        let rule: Vec<_> = rule.split("{").collect();
        debug_assert!(rule.len() == 2);
        rules.insert(rule[0].to_string(), rule[1]
            .split(",")
            .map(|rule| Rule::new(rule))
            .collect::<Vec<_>>()
        );
    }

    let mut sum = 0;
    for part in puzzle[1].split("\n") {
        let mut v: [usize; 4] = [0, 0, 0, 0];
        for value in part.get(1..part.len()-1).expect("invalid puzzle input").split(",") {
            let index = letter_to_index(value.chars().next().expect("invalid puzzle input"));
            let threshold: usize = value.get(2..).expect("invalid puzzle input").parse().expect("failed to parse value");
            v[index] = threshold;
        }

        let mut workflow = "in".to_string();
        'outer: loop {
            for rule in rules.get(&workflow).expect("invalid puzzle input: unknown rule") {
                if rule.condition.accept(&v) {
                    match &rule.target {
                        Target::Rule(next_workflow) => {
                            workflow = next_workflow.clone();
                            continue 'outer;
                        },
                        Target::Accept => {
                            sum += v.iter().copied().reduce(|acc, v| acc + v).unwrap();
                            break 'outer;
                        },
                        Target::Reject => break 'outer,
                    }
                }
            }
        }
    }
    println!("part one = {}", sum);

    sum = 0;
    let mut stack = vec![("in".to_string(), [1_usize; 4], [4000_usize; 4])];
    while let Some((workflow, min_v, max_v)) = stack.pop() {
        let mut interval = Some((min_v, max_v));
        for rule in rules.get(&workflow).expect("invalid puzzle input") {
            if let Some(subinterval) = rule.condition.accepted_interval(interval.unwrap()) {
                match &rule.target {
                    Target::Rule(next_workflow) => stack.push((next_workflow.clone(), subinterval.0, subinterval.1)),
                    Target::Accept => sum += subinterval.0
                        .into_iter()
                        .zip(subinterval.1.into_iter())
                        .map(|(a, b)| b - a + 1)
                        .reduce(|acc, v| acc * v)
                        .unwrap(),
                    Target::Reject => (),
                }
            }
            interval = rule.condition.rejected_interval(interval.unwrap());
            if interval.is_none() {
                break;
            }
        }
    }
    println!("part two = {}", sum);
}
