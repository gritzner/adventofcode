use std::collections::VecDeque;

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");

    let mut part_one = 0;
    let mut part_two = 0;
    for sequence in puzzle.split("\n") {
        let sequence: VecDeque<isize> = sequence.split(" ").map(|s| s.parse().expect("failed to parse sequence")).collect();
        let mut sequences = vec![sequence];
        while sequences.last().unwrap().iter().any(|v| *v != 0) {
            let sequence = sequences.last().unwrap();
            sequences.push(sequence
                .iter()
                .skip(1)
                .enumerate()
                .map(|(i, v)| v - sequence[i])
                .collect()
            );
        }
        let sequence = sequences.last_mut().unwrap();
        sequence.push_front(0);
        sequence.push_back(0);
        while sequences.len() > 1 {
            let delta = sequences.pop().unwrap();
            let mut sequence = sequences.pop().unwrap();
            sequence.push_front(sequence.front().unwrap() - delta.front().unwrap());
            sequence.push_back(sequence.back().unwrap() + delta.back().unwrap());
            sequences.push(sequence);
        }
        let sequence = sequences.last().unwrap();
        part_one += sequence.back().unwrap();
        part_two += sequence.front().unwrap();
    }
    println!("part one = {}", part_one);
    println!("part two = {}", part_two);
}
