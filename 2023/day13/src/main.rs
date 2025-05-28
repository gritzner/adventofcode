use std::collections::{BTreeMap, BTreeSet};

fn mirrored_at_single(row: &Vec<char>, cache: &mut BTreeMap<Vec<char>, BTreeSet<usize>>) -> BTreeSet<usize> {
    if let Some(result) = cache.get(row) {
        result.clone()
    } else {
        let mut result = BTreeSet::new();
        for i in 1..row.len() {
            let mut left: Vec<_> = row.get(..i).unwrap().iter().copied().collect();
            let mut right: Vec<_> = row.get(i..).unwrap().iter().rev().copied().collect();
            if (0..left.len().min(right.len())).all(|_| left.pop().unwrap() == right.pop().unwrap()) {
                result.insert(i);
            }
        }
        cache.insert(row.clone(), result.clone());
        result
    }
}

fn mirrored_at(puzzle: &Vec<Vec<char>>, cache: &mut BTreeMap<Vec<char>, BTreeSet<usize>>) -> Option<usize> {
    puzzle
        .iter()
        .map(|row| mirrored_at_single(row, cache))
        .reduce(|acc, v| acc.intersection(&v).copied().collect())
        .unwrap()
        .iter()
        .copied()
        .next()
}

fn transpose(puzzle: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..puzzle[0].len())
        .map(|x| (0..puzzle.len()).map(|y| puzzle[y][x]).collect())
        .collect()
}

fn almost_mirrored_at(puzzle: &Vec<Vec<char>>, cache: &mut BTreeMap<Vec<char>, BTreeSet<usize>>) -> Option<usize> {
    let puzzle: Vec<_> = puzzle
        .iter()
        .map(|row| mirrored_at_single(row, cache))
        .collect();
    let mut map = BTreeMap::new();
    for row in &puzzle {
        for &n in row {
            let entry = map.entry(n).or_insert(0_usize);
            *entry += 1;
        }
    }
    for (k, v) in map {
        if v != puzzle.len() - 1 {
            continue;
        }
        return Some(k);
    }
    None
}

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
    let mut cache = BTreeMap::new();
    for puzzle in puzzle.split("\n\n") {
        let puzzle: Vec<Vec<_>> = puzzle
            .split("\n")
            .map(|s| s.chars().collect())
            .collect();
        
        if let Some(n) = mirrored_at(&puzzle, &mut cache) {
            part_one += n;
        } else {
            let puzzle = transpose(puzzle.clone());
            part_one += 100 * mirrored_at(&puzzle, &mut cache).unwrap();
        }

        if let Some(n) = almost_mirrored_at(&puzzle, &mut cache) {
            part_two += n;
        } else {
            let puzzle = transpose(puzzle);
            part_two += 100 * almost_mirrored_at(&puzzle, &mut cache).unwrap();
        }
    }
    println!("part one = {}", part_one);
    println!("part two = {}", part_two);
}
