use std::collections::{BTreeMap, BinaryHeap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North, East, South, West
}

impl Direction {
    fn rotate(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Path {
    y: usize,
    x: usize,
    d: Direction,
    loss: usize,
    min_remainder: usize,
}

impl Path {
    fn next(&self, puzzle: &Vec<Vec<usize>>) -> Option<Self> {
        let mut candidate = self.clone();
        let candidate = match self.d {
            Direction::North => {
                if candidate.y > 0 {
                    candidate.y -= 1;
                    Some(candidate)
                } else {
                    None
                }
            },
            Direction::East => {
                if candidate.x + 1 < puzzle[0].len() {
                    candidate.x += 1;
                    Some(candidate)
                } else {
                    None
                }
            },
            Direction::South => {
                if candidate.y + 1 < puzzle.len() {
                    candidate.y += 1;
                    Some(candidate)
                } else {
                    None
                }
            },
            Direction::West => {
                if candidate.x > 0 {
                    candidate.x -= 1;
                    Some(candidate)
                } else {
                    None
                }
            },
        };
        if let Some(mut candidate) = candidate {
            candidate.loss += puzzle[candidate.y][candidate.x];
            candidate.min_remainder = (puzzle.len() - candidate.y - 1) + (puzzle[0].len() - candidate.x - 1);
            return Some(candidate);
        }
        None
    }

    fn all_next(&self, puzzle: &Vec<Vec<usize>>, ultra_crucibles: bool) -> Vec<Self> {
        let mut candidates = vec![self.clone()];
        for _ in 0..(if ultra_crucibles { 10 } else { 3 }) {
            if let Some(candidate) = candidates.last().unwrap().next(puzzle) {
                candidates.push(candidate);
            }
        }
        candidates
            .into_iter()
            .skip(if ultra_crucibles { 4 } else { 1 })
            .map(|candidate| {
                let mut new_candidates = Vec::with_capacity(2);
                for d in candidate.d.rotate() {
                    let mut new_candidate = candidate.clone();
                    new_candidate.d = d;
                    new_candidates.push(new_candidate);
                }
                new_candidates
            })
            .flatten()
            .collect()
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.loss + self.min_remainder).cmp(&(other.loss + self.min_remainder)).reverse()
    }
}

fn find_best_path(puzzle: &Vec<Vec<usize>>, ultra_crucibles: bool) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(Path { y: 0, x: 0, d: Direction::East, loss: 0, min_remainder: 0 });
    heap.push(Path { y: 0, x: 0, d: Direction::South, loss: 0, min_remainder: 0 });
    let mut visited = BTreeMap::new();
    while let Some(path) = heap.pop() {
        if path.y == puzzle.len() - 1 && path.x == puzzle[0].len() - 1 {
            return path.loss;
        }
        let cost = path.loss + path.min_remainder;
        let previous_best = visited.entry((path.y, path.x, path.d)).or_insert(usize::MAX);
        if *previous_best <= cost {
            continue;
        }
        *previous_best = cost;
        for path in path.all_next(&puzzle, ultra_crucibles) {
            heap.push(path);
        }
    }
    panic!("could not find any path to destination");
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle: Vec<Vec<usize>> = puzzle
        .split("\n")
        .map(|line| line
            .chars()
            .map(|c| String::from(c).parse().expect("failed to parse input"))
            .collect()
        )
        .collect();
    println!("part one = {}", find_best_path(&puzzle, false));
    println!("part two = {}", find_best_path(&puzzle, true));
}
