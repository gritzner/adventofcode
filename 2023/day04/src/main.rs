use std::collections::BTreeSet;
use regex::Regex;

struct Card {
    winning: BTreeSet<usize>,
    numbers: BTreeSet<usize>,
}

impl Card {
    fn new(s: &str, pattern: &Regex) -> Self {
        let i = s.find("|").expect("malformatted card");
        let mut winning = BTreeSet::new();
        let mut numbers = BTreeSet::new();
        for m in pattern.find_iter(s).skip(1) {
            let number = m.as_str().parse().expect("failed to parse number into usize");
            if m.start() < i {
                winning.insert(number);
            } else {
                numbers.insert(number);
            }
        }
        Self { winning, numbers }
    }

    fn num_matches(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
    }

    fn score(&self) -> usize {
        let n = self.num_matches();
        if n > 0 {
            2_usize.pow((n-1) as u32)
        } else {
            0
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

    let pattern = Regex::new(r"[0-9]+").unwrap();
    let cards: Vec<_> = puzzle.split("\n").map(|line| Card::new(line, &pattern)).collect();
    println!("part one = {}", cards.iter().map(|card| card.score()).fold(0, |acc, v| acc + v));
    
    let mut num_cards = vec![1_usize; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let count = num_cards[i];
        for j in (i+1)..(i+1+card.num_matches()) {
            num_cards[j] += count;
        }
    }
    println!("part two = {}", num_cards.into_iter().fold(0, |acc, v| acc + v));
}
