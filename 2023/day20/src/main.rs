use std::collections::{BTreeMap, BTreeSet, VecDeque};
use num::integer::lcm;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low, High
}

impl Pulse {
    fn index(&self) -> usize {
        match self {
            Pulse::Low => 0,
            Pulse::High => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    Broadcaster, FlipFlop, Conjunction
}

impl ModuleType {
    fn new(s: &str) -> (Self, String) {
        match s.chars().next().expect("invalid module name") {
            '%' => (Self::FlipFlop, s.get(1..).expect("invalid module name").to_string()),
            '&' => (Self::Conjunction, s.get(1..).expect("invalid module name").to_string()),
            _ => (Self::Broadcaster, s.to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Module {
    name: String,
    module_type: ModuleType,
    inputs: BTreeMap<String, Pulse>,
    outputs: Vec<String>,
}

impl From<&str> for Module {
    fn from(s: &str) -> Self {
        let s: Vec<_> = s.split(" -> ").collect();
        debug_assert!(s.len() == 2);
        let (module_type, name) = ModuleType::new(s[0]);
        let mut inputs = BTreeMap::new();
        if module_type == ModuleType::FlipFlop {
            inputs.insert("state".to_string(), Pulse::Low);
        }
        let outputs: Vec<String> = s[1].split(", ").map(|s| s.into()).collect();
        Self { name, module_type, inputs, outputs }
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
    let puzzle: Vec<Module> = puzzle.split("\n").map(|module| module.into()).collect();

    let mut modules = BTreeMap::new();
    for module in puzzle {
        modules.insert(module.name.clone(), module);
    }
    let mut conjunctions = BTreeMap::new();
    for module in modules.values() {
        if module.module_type != ModuleType::Conjunction {
            continue;
        }
        conjunctions.insert(module.name.clone(), Vec::new());
    }
    for module in modules.values() {
        for output in &module.outputs {
            if let Some(target) = conjunctions.get_mut(output) {
                target.push(module.name.clone());
            }
        }
    }
    for (name, inputs) in conjunctions {
        let module = modules.get_mut(&name).unwrap();
        for name in inputs {
            module.inputs.insert(name, Pulse::Low);
        }
    }

    let target_invs: BTreeSet<_> = if let Some(inv_before_rx) = modules
        .values()
        .filter(|module| module.outputs[0] == "rx")
        .next() {
        modules
            .values()
            .filter(|module| module.outputs[0] == inv_before_rx.name)
            .map(|module| module.name.clone())
            .collect()
    } else {
        BTreeSet::new()
    };
    let mut button_presses_until_target = BTreeMap::new();

    let mut pulses = [0_usize; 2];
    let mut part_one = 0;
    for button_press in 0..10000_usize {
        if button_press == 1000 {
            part_one = pulses[0] * pulses[1];
        }
        let mut sent_pulses = VecDeque::new();
        sent_pulses.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        'the_loop: while let Some((source, pulse, target)) = sent_pulses.pop_front() {
            pulses[pulse.index()] += 1;
            if pulse == Pulse::Low && target_invs.contains(&target) && !button_presses_until_target.contains_key(&target) {
                button_presses_until_target.insert(target.clone(), button_press + 1);
            }
            let module = modules.get_mut(&target);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let pulse = match module.module_type {
                ModuleType::Broadcaster => {
                    pulse
                },
                ModuleType::FlipFlop => {
                    if pulse == Pulse::Low {
                        let state = module.inputs.values_mut().next().unwrap();
                        *state = match state {
                            Pulse::Low => Pulse::High,
                            Pulse::High => Pulse::Low,
                        };
                        *state
                    } else {
                        continue 'the_loop;
                    }
                },
                ModuleType::Conjunction => {
                    module.inputs.insert(source, pulse);
                    if module.inputs.values().any(|&pulse| pulse == Pulse::Low) { Pulse::High } else { Pulse::Low }
                },
            };
            for output in &module.outputs {
                sent_pulses.push_back((module.name.clone(), pulse, output.clone()));
            }
        }
    }
    println!("part one = {}", part_one);
    println!("part two = {}", button_presses_until_target.values().fold(1, |acc, &v| lcm(acc, v)));
}
