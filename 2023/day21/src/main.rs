/* the solution below works but is way too slow

use std::collections::{BTreeMap, BTreeSet};

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Garden {
    height: isize,
    width: isize,
    start_position: (isize, isize),
    rocks: BTreeSet<(isize, isize)>,
}

impl Garden {
    fn new(puzzle: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut start_position = (0, 0);
        let mut rocks = BTreeSet::new();
        for (y, line) in puzzle.split("\n").enumerate() {
            let y = y as isize;
            height = height.max(y+1);
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                width = width.max(x+1);
                let pos = (y, x);
                if c == 'S' {
                    start_position = pos;
                } else if c == '#' {
                    rocks.insert(pos);
                }
            }
        }
        Self { height, width, start_position, rocks }
    }

    fn step(&self, positions: &BTreeSet<(isize, isize)>) -> [BTreeSet<(isize, isize)>; 5] {
        let mut result = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
        for &(y, x) in positions {
            for (dy, dx) in NEIGHBORS {
                let (y, x) = (y + dy, x + dx);
                if y < 0 {
                    result[1].insert((self.height-1, x));
                } else if y >= self.height {
                    result[3].insert((0, x));
                } else if x < 0 {
                    result[4].insert((y, self.width-1));
                } else if x >= self.width {
                    result[2].insert((y, 0));
                } else if !self.rocks.contains(&(y, x)) {
                    result[0].insert((y, x));
                }
            }
        }
        result
    }
}

struct BijectiveMap<T> {
    forward: BTreeMap<T, usize>,
    backward: Vec<T>,
    successors: Vec<Option<usize>>,
    to_neighbors: Vec<Option<[usize; 4]>>,
}

impl<T> BijectiveMap<T> where T: Clone + Ord {
    fn new(first: T) -> Self {
        let mut forward = BTreeMap::new();
        forward.insert(first.clone(), 0);
        Self { 
            forward,
            backward: vec![first],
            successors: vec![None],
            to_neighbors: vec![None],
        }
    }

    fn to_index(&mut self, v: &T) -> usize {
        let entry = self.forward.entry(v.clone()).or_insert_with(|| {
            self.backward.push(v.clone());
            self.successors.push(None);
            self.to_neighbors.push(None);
            self.backward.len() - 1
        });
        *entry
    }

    fn from_index(&self, index: usize) -> T {
        self.backward[index].clone()
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
    let garden = Garden::new(&puzzle);

    let mut states = BTreeMap::new();
    let mut state_cache = BijectiveMap::new(BTreeSet::<(isize, isize)>::new());
    let mut initial_state = BTreeSet::new();
    initial_state.insert(garden.start_position);
    states.insert((0, 0), state_cache.to_index(&initial_state));
    let mut union_cache = BTreeMap::new();

    for step in 0..1000 {
        if step == 64 {
            println!("part one = {}", state_cache.from_index(*states.get(&(0, 0)).unwrap()).len());
        }
        let mut new_states = BTreeMap::new();
        for (&(y, x), &state) in &states {
            if let Some(new_state) = state_cache.successors[state] {
                new_states.insert((y, x), new_state);
            } else {
                let states = garden
                    .step(&state_cache.from_index(state))
                    .map(|state| state_cache.to_index(&state));
                new_states.insert((y, x), states[0]);
                state_cache.successors[state] = Some(states[0]);
                state_cache.to_neighbors[state] = Some([states[1], states[2], states[3], states[4]]);
            }
        }
        for ((y, x), state) in states {
            for ((dy, dx), to_neighbor) in NEIGHBORS.into_iter().zip(state_cache.to_neighbors[state].unwrap()) {
                let neighbor = (y + dy, x + dx);
                let states = (*new_states.get(&neighbor).or(Some(&0)).unwrap(), to_neighbor);
                let states = (states.0.min(states.1), states.0.max(states.1));
                let new_state = if states.0 == 0 {
                    states.1
                } else {
                    let entry = union_cache.entry(states).or_insert_with(|| {
                        let union: BTreeSet<_> = state_cache.from_index(states.0)
                            .union(&state_cache.from_index(states.1))
                            .copied()
                            .collect();
                        state_cache.to_index(&union)
                    });
                    *entry
                };
                if new_state > 0 {
                    new_states.insert(neighbor, new_state);
                }
            }
        }
        states = new_states;
    }
    println!("part two = {}", states.values().map(|&v| state_cache.from_index(v).len()).reduce(|acc, v| acc + v).unwrap());
}
*/

// solution by https://old.reddit.com/r/adventofcode/comments/18nol3m/2023_day_21_a_geometric_solutionexplanation_for/
// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

use std::collections::{BTreeMap, BTreeSet, VecDeque};

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    
    let mut height = 0;
    let mut width = 0;
    let mut starting_position = (0, 0);
    let mut rocks = BTreeSet::new();
    for (y , line) in puzzle.split("\n").enumerate() {
        let y = y as isize;
        height = height.max(y + 1);
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            width = width.max(x + 1);
            match c {
                'S' => starting_position = (y, x),
                '#' => {
                    rocks.insert((y, x));
                    ()
                },
                _ => (),
            }
        }
    }

    let mut visited = BTreeMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((starting_position, 0_usize));
    while let Some(((y, x), steps)) = queue.pop_front() {
        visited.entry((y, x)).or_insert_with(|| {
            for (dy, dx) in NEIGHBORS {
                let (y, x) = (y + dy, x + dx);
                if y >= 0 && y < height && x >= 0 && x < width && !rocks.contains(&(y, x)) {
                    queue.push_back(((y, x), steps+1));
                }
            }
            steps
        });
    }

    println!("part one = {}", visited.values().filter(|v| **v <= 64 && (**v % 2 == 0)).count());

    let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    let even_corners = visited.values().filter(|v| **v > 65 && (**v % 2 == 0)).count();
    let odd_full = visited.values().filter(|v| **v % 2 == 1).count();
    let odd_corners = visited.values().filter(|v| **v > 65 && (**v % 2 == 1)).count();

    assert_eq!(height, width);
    let n = height as usize;
    let n = (26501365 - (n / 2)) / n;
    assert_eq!(n, 202300);

    let n = (n+1).pow(2) * odd_full + n.pow(2) * even_full - (n+1) * odd_corners + n * even_corners;
    println!("part two = {}", n);
}
