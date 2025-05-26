use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug)]
struct Hand {
    cards: [usize; 5],
    poker_type: usize,
    bid: usize,
}

impl Hand {
    fn new(s: &str) -> Self {
        let s: Vec<_> = s.split(" ").collect();
        let mut cards = [usize::MAX; 5];
        let mut counts = BTreeMap::new();
        for (i, c) in s[0].chars().enumerate() {
            cards[i] = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => c.to_string().parse().expect("failed parsing number card in hand")
            };
            let count = counts.entry(cards[i]).or_insert(0);
            *count += 1;
        }
        let poker_type = determine_poker_type(&counts);
        let bid = s[1].parse().expect("failed parsing bid");
        Self { cards, poker_type, bid }
    }

    fn compare(&self, other: &Self) -> Ordering {
        if self.poker_type == other.poker_type {
            self.cards.cmp(&other.cards).reverse()
        } else {
            self.poker_type.cmp(&other.poker_type)
        }
    }

    fn upgrade(&mut self) {
        let mut counts = BTreeMap::new();
        for card in self.cards.iter_mut() {
            if *card == 11 {
                *card = 1;
            }
            let count = counts.entry(*card).or_insert(0);
            *count += 1;
        }
        if let Some(&jokers) = counts.get(&1) {
            for &card in counts.keys() {
                if card == 1 {
                    continue;
                }
                let mut counts = counts.clone();
                counts.remove(&1);
                *counts.get_mut(&card).unwrap() += jokers;
                self.poker_type = self.poker_type.min(determine_poker_type(&counts));
            }
        }
    }
}

fn determine_poker_type(counts: &BTreeMap<usize, usize>) -> usize {
    let n = counts.len();
    match n {
        1 => 0, // five of a kind
        2 => {
            if counts.values().any(|&k| k == 4) {
                1 // four of a kind
            } else {
                2 // full house
            }
        },
        3 => {
            if counts.values().any(|&k| k == 3) {
                3 // three of a kind
            } else {
                4 // two pairs
            }
        },
        4 => 5, // one pair
        _ => {
            debug_assert!(n == 5);
            6 // high card
        },
    }
}

fn compute_winnings(hands: &Vec<Hand>) -> usize {
    let n = hands.len();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (n-i)*hand.bid)
        .fold(0, |acc, v| acc + v)
}

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let mut puzzle: Vec<_> = puzzle
        .split("\n")
        .map(|line| Hand::new(line))
        .collect();
    puzzle.sort_by(|a, b| a.compare(b));
    println!("part one = {}", compute_winnings(&puzzle));
    for hand in &mut puzzle {
        hand.upgrade();
    }
    puzzle.sort_by(|a, b| a.compare(b));
    println!("part two = {}", compute_winnings(&puzzle));
}
