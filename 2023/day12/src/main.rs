use std::collections::BTreeMap;

fn solve(record: String, target: Vec<usize>, cache: &mut BTreeMap<(String, Vec<usize>), usize>) -> usize {
    if target.is_empty() {
        if record.chars().any(|c| c == '#') {
            return 0;
        } else {
            return 1;
        }
    }
    if let Some(result) = cache.get(&(record.clone(), target.clone())) {
        return *result;
    }
    let mut sum = 0;
    let new_target: Vec<_> = target.iter().skip(1).copied().collect();
    for i in 0..record.len() {
        if i > 0 && record.chars().nth(i-1) == Some('#') {
            break;
        } else if let Some(s) = record.get(i..i+target[0]+1) {
            if s.chars().last().unwrap() == '#' {
                continue;
            }
            if s.get(..s.len()-1).unwrap().chars().any(|c| c == '.') {
                continue;
            }
            if let Some(remainder) = record.get(i+target[0]+1..) {
                sum += solve(remainder.to_owned(), new_target.clone(), cache);
            } else {
                sum += 1;
            }
        } else {
            break;
        }
    }
    cache.insert((record, target), sum);
    sum
}

fn compute_num_arrangements(puzzle: &str, unfold: bool) -> usize {
    let mut sum = 0;
    let mut cache = BTreeMap::new();
    for record in puzzle.split("\n") {
        let mut record: Vec<String> = record.split(" ").map(|s| s.into()).collect();
        debug_assert!(record.len() == 2);
        if unfold {
            let temp: Vec<_> = (0..5).map(|_i| record[0].clone()).collect();
            record[0] = temp.join("?");
            let temp: Vec<_> = (0..5).map(|_i| record[1].clone()).collect();
            record[1] = temp.join(",");
        }
        let target: Vec<usize> = record[1]
            .split(",")
            .map(|v| v.parse().expect("failed to parse target"))
            .collect();
        let mut record = record[0].clone();
        record.push('.');
        sum += solve(record, target, &mut cache);
    }
    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    println!("part one = {}", compute_num_arrangements(&puzzle, false));
    println!("part two = {}", compute_num_arrangements(&puzzle, true));
}
