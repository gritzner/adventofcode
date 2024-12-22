use std::collections::{BTreeMap, BTreeSet};

const MASK: u64 = 2_u64.pow(24) - 1;

fn step(secret: u64, shift: isize) -> u64 {
    (if shift >= 0 {
        secret << shift
    } else {
        secret >> shift.abs()
    } ^ secret) & MASK
}

fn next(secret: u64) -> u64 {
    step(step(step(secret, 6), -5), 11)
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle = puzzle
        .split("\n")
        .filter(|puzzle| !puzzle.is_empty())
        .map(|puzzle| puzzle.parse::<u64>().expect("invalid puzzle input"));

    let mut sum = 0;
    let mut bananas = BTreeMap::new();
    for mut secret in puzzle {
        let mut prices = Vec::with_capacity(2001);
        prices.push((secret % 10) as isize);
        for _ in 0..2000 {
            secret = next(secret);
            prices.push((secret % 10) as isize);
        }
        sum += secret;
        let mut sequence = [0, prices[1]-prices[0], prices[2]-prices[1], prices[3]-prices[2]];
        let mut sequences = BTreeSet::new();
        for i in 4..2001 {
            for j in 0..3 {
                sequence[j] = sequence[j+1];
            }
            sequence[3] = prices[i] - prices[i-1];
            if sequences.insert(sequence) {
                let sum_of_bananas = bananas.entry(sequence).or_insert(0);
                *sum_of_bananas += prices[i] as usize;
            }
        }
    }
    println!("[part one] sum = {}", sum);
    println!("[part two] # of earnable bananas = {}", bananas.values().fold(0, |acc, &v| acc.max(v)));
}
