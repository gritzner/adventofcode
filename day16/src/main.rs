use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardinalDirection {
    NORTH, EAST, SOUTH, WEST
}

impl CardinalDirection {
    fn rotate_clockwise(&self) -> Self {
        match self {
            CardinalDirection::NORTH => CardinalDirection::EAST,
            CardinalDirection::EAST => CardinalDirection::SOUTH,
            CardinalDirection::SOUTH => CardinalDirection::WEST,
            CardinalDirection::WEST => CardinalDirection::NORTH
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            CardinalDirection::NORTH => CardinalDirection::WEST,
            CardinalDirection::EAST => CardinalDirection::NORTH,
            CardinalDirection::SOUTH => CardinalDirection::EAST,
            CardinalDirection::WEST => CardinalDirection::SOUTH
        }
    }
}

const ALL_DIRECTIONS: [CardinalDirection; 4] = [CardinalDirection::NORTH, CardinalDirection::EAST, CardinalDirection::SOUTH, CardinalDirection::WEST];

struct Map {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    directions: [(usize, usize, CardinalDirection); 4]
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Self {
        let height = map.len();
        let width = map[0].len();
        let directions = [
            (height - 1, 0, CardinalDirection::NORTH),
            (0, 1, CardinalDirection::EAST),
            (1, 0, CardinalDirection::SOUTH),
            (0, width - 1, CardinalDirection::WEST)
        ];
        Map {
            map: map,
            height: height,
            width: width,
            directions: directions
        }
    }

    fn is_corner(&self, y: usize, x: usize) -> bool {
        if self.map[y][x] == '#' {
            return false;
        }
        let mut directions = [false; 4];
        let mut num_paths = 0;
        for (i, d) in self.directions.into_iter().enumerate() {
            let y2 = (y + d.0) % self.height;
            let x2 = (x + d.1) % self.width;
            if self.map[y2][x2] == '.' {
                directions[i] = true;
                num_paths += 1;
            }
        }
        ((directions[0] || directions[2]) && (directions[1] || directions[3])) || num_paths == 1
    }

    fn get_neighbors(&self, y: usize, x: usize) -> BTreeSet<(usize, usize, CardinalDirection, Vec<(usize, usize)>)> {
        let mut neighbors = BTreeSet::new();
        for d in self.directions {
            let mut i = 1;
            let mut path = Vec::new();
            loop {
                let y2 = (y + i * d.0) % self.height;
                let x2 = (x + i * d.1) % self.width;
                path.push((y2, x2));
                if self.map[y2][x2] == '#' {
                    break;
                } else if self.is_corner(y2, x2) {
                    neighbors.insert((y2, x2, d.2, path));
                    break;
                }
                i += 1;
            }
        }
        neighbors
    }
}

struct Vertex {
    score: usize,
    predecessors: BTreeSet<(usize, usize, CardinalDirection)>
}

impl Vertex {
    fn new() -> Self {
        Vertex {
            score: usize::MAX,
            predecessors: BTreeSet::new()
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
    let content = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");

    let mut reindeer = (usize::MAX, usize::MAX, CardinalDirection::EAST);
    let mut goal = (usize::MAX, usize::MAX);
    let map = Map::new(content
        .split("\n")
        .filter(|row| !row.is_empty())
        .enumerate()
        .map(|(y, row)| row
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                'S' => {
                    reindeer = (y, x, reindeer.2);
                    '.'
                },
                'E' => {
                    goal = (y, x);
                    '.'
                },
                _ => c
            })
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
    );

    let mut graph = BTreeMap::new();
    let mut vertices = BTreeMap::new();
    for (y, row) in map.map.iter().enumerate() {
        for x in 0..row.len() {
            if map.is_corner(y, x) {
                for cd in ALL_DIRECTIONS {
                    vertices.insert((y, x, cd), Vertex::new());
                }
                for (y2, x2, cd, path) in map.get_neighbors(y, x) {
                    graph.insert((y, x, cd), (y2, x2, path));
                }
            }
        }
    }

    vertices.get_mut(&reindeer).unwrap().score = 0;
    let mut stack = vec![reindeer];
    while let Some((y, x, cd)) = stack.pop() {
        if y == goal.0 && x == goal.1 {
            continue;
        }
        let score = vertices.get(&(y, x, cd)).unwrap().score;
        if let Some((y2, x2, path)) = graph.get(&(y, x, cd)) {
            let new_score = score + path.len();
            let y2 = *y2;
            let x2 = *x2;
            let vertex = vertices.get_mut(&(y2, x2, cd)).unwrap();
            if new_score < vertex.score {
                vertex.score = new_score;
                vertex.predecessors.clear();
                vertex.predecessors.insert((y, x, cd));
                stack.push((y2, x2, cd));
            } else if new_score == vertex.score {
                vertex.predecessors.insert((y, x, cd));
            }
        }
        let new_score = score + 1000;
        for cd2 in [cd.rotate_clockwise(), cd.rotate_counterclockwise()] {
            let vertex = vertices.get_mut(&(y, x, cd2)).unwrap();
            if new_score < vertex.score {
                vertex.score = new_score;
                vertex.predecessors.clear();
                vertex.predecessors.insert((y, x, cd));
                stack.push((y, x, cd2));
            } else if new_score == vertex.score {
                vertex.predecessors.insert((y, x, cd));
            }
        }
    }
    let mut best = reindeer.2;
    for cd in ALL_DIRECTIONS {
        if vertices.get(&(goal.0, goal.1, cd)).unwrap().score < vertices.get(&(goal.0, goal.1, best)).unwrap().score {
            best = cd;
        }
    }
    println!("[part one] score = {}", vertices.get(&(goal.0, goal.1, best)).unwrap().score);

    let mut visited = BTreeSet::new();
    let mut stack = vec![(goal.0, goal.1, best)];
    while let Some((y, x, cd)) = stack.pop() {
        visited.insert((y, x));
        for &(py, px, pcd) in &vertices.get(&(y, x, cd)).unwrap().predecessors {
            if py != y || px != x {
                for &(py, px) in &graph.get(&(py, px, pcd)).unwrap().2 {
                    visited.insert((py, px));
                }
            }
            stack.push((py, px, pcd));
        }
    }
    println!("[part two] # of tiles on at least one optimal path = {}", visited.len());
}
