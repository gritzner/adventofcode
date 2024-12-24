use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operations {
    AND, OR, XOR
}

impl Operations {
    fn from_str(s: &str) -> Self {
        match s {
            "AND" => Operations::AND,
            "OR" => Operations::OR,
            "XOR" => Operations::XOR,
            _ => panic!("invalid puzzle input")
        }
    }

    fn apply(&self, left: bool, right: bool) -> bool {
        match self {
            Operations::AND => left && right,
            Operations::OR => left || right,
            Operations::XOR => left != right,
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
    let mut puzzle = puzzle.split("\n");
    let fixes_str = std::fs::read_to_string("day24/fixes.txt").expect("cannot read file with fixes for puzzle input");
    let mut fixes = BTreeMap::new();
    for line in fixes_str.split("\n").filter(|line| !line.is_empty()) {
        let fix = line.split(",").collect::<Vec<_>>();
        fixes.insert(fix[0], fix[1]);
        fixes.insert(fix[1], fix[0]);
    }


    let mut inputs = BTreeMap::new();
    let mut max_index = 0;
    while let Some(line) = puzzle.next() {
        if line.is_empty() {
            break;
        }
        let line = line.split(": ").collect::<Vec<_>>();
        let value = line[1].parse::<u8>().expect("invalid puzzle input");
        inputs.insert(line[0], value == 1);
        max_index = max_index.max(line[0].split_at(1).1.parse::<usize>().expect("invalud puzzle input"));
    }

    let mut outputs = BTreeMap::new();
    let mut half_adders = BTreeMap::new();
    let mut or_gates = BTreeMap::new();
    while let Some(line) = puzzle.next() {
        if line.is_empty() {
            break;
        }
        let mut line = line.split(" ").collect::<Vec<_>>();
        let op = Operations::from_str(line[1]);
        outputs.insert(line[4], (line[0], op, line[2]));
        if let Some(fix) = fixes.get(&line[4]) {
            line[4] = fix;
        }
        let key = BTreeSet::from([line[0], line[2]]);
        if op == Operations::AND || op == Operations::XOR {
            let entry = half_adders.entry(key).or_insert_with(BTreeMap::new);
            entry.insert(op, line[4]);
        } else {
            if or_gates.insert(key, line[4]).is_some() {
                panic!("invalid puzzle input");
            }
        }
    }

    loop {
        let mut new_inputs = BTreeSet::new();
        for (&out, &(in0, op, in1)) in &outputs {
            if let Some(&in0) = inputs.get(in0) {
                if let Some(&in1) = inputs.get(in1) {
                    let value = op.apply(in0, in1);
                    inputs.insert(out, value);
                    new_inputs.insert(out);
                }
            }
        }
        for &out in &new_inputs {
            outputs.remove(out);
        }
        if new_inputs.is_empty() || outputs.is_empty() {
            break;
        }
    }

    let mut number = 0_u64;
    for (var, value) in inputs {
        if !var.starts_with('z') || !value {
            continue;
        }
        let shift = var.split_at(1).1.parse::<usize>().expect("invalid puzzle input");
        number = number | (1 << shift);
    }
    println!("[part one] number = {}", number);

    // AND and XOR together are a half adder
    // a full adder consists of two half adders and an OR
    // the first half adder takes x and y: its carry (AND) goes into the OR gate while its sum (XOR) goes into the other half adder
    // the second half adder additionally takes the carry of the previous full adder as input
    // the second half adders sum is the actual sum of the full adder while its carry goes into the OR gate
    // the OR gate's output is the actual carry of the full adder
    // least significant bit only needs a half adder
    // most significant bit is the actual carry of the previous full adder
    let mut carries = vec![*half_adders.get(&BTreeSet::from(["x00", "y00"])).unwrap().get(&Operations::AND).unwrap()]; // assumes that the first half adder is defined correctly
    for index in 1..max_index+1 {
        let key = (format!("x{:02}", index), format!("y{:02}", index));
        let key = BTreeSet::from([key.0.as_str(), key.1.as_str()]);
        let first_ha = half_adders.get(&key).unwrap();
        let second_key = BTreeSet::from([*first_ha.get(&Operations::XOR).unwrap(), carries[index-1]]);
        if let Some(second_ha) = half_adders.get(&second_key) {
            if *second_ha.get(&Operations::XOR).unwrap() != format!("z{:02}", index).as_str() {
                panic!("sum of second half adder has wrong output: {} vs. z{:02}", second_ha.get(&Operations::XOR).unwrap(), index);
            }
            let or_key = BTreeSet::from([*first_ha.get(&Operations::AND).unwrap(), *second_ha.get(&Operations::AND).unwrap()]);
            if let Some(&or_gate) = or_gates.get(&or_key) {
                carries.push(or_gate);
            } else {
                panic!("missing OR gate: {:?} ({:?}, {:?})", or_key, key, second_key);
            }
        } else {
            panic!("second half adder missing: {:?} -> {:?}", key, second_key);
        }
    }
    
    let mut fixes = fixes.values().into_iter().map(|&s| s).collect::<Vec<_>>();
    fixes.sort();
    let fixes = fixes.iter().skip(1).fold(fixes[0].to_owned(), |acc, &s| format!("{},{}", acc, s));
    println!("[part two] swapped outputs = {}", fixes);
}
