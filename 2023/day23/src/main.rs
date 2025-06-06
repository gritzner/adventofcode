use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North, East, South, West
}

impl Direction {
    fn next(&self, pos: (isize, isize)) -> (isize, isize) {
        let (dy, dx) = match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };
        (pos.0 + dy, pos.1 + dx)
    }
}

const NEIGHBORS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path, Forest, Slope(Direction)
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Direction::North),
            '>' => Self::Slope(Direction::East),
            'v' => Self::Slope(Direction::South),
            '<' => Self::Slope(Direction::West),
            _ => panic!("invalid puzzle input"),
        }
    }
}

fn neighbors(island: &BTreeMap<(isize, isize), Tile>, pos: (isize, isize), ignore_slopes: bool) -> BTreeSet<(isize, isize)> {
    let mut result = BTreeSet::new();
    for d in NEIGHBORS {
        let next = d.next(pos);
        if let Some(tile) = island.get(&next) {
            match tile {
                Tile::Path => {result.insert(next);},
                Tile::Forest => (),
                Tile::Slope(d) => if ignore_slopes || d.next(next) != pos {
                    result.insert(next);
                },
            }
        }
    }
    result
}

fn get_directed_graph(
    island: &BTreeMap<(isize, isize), Tile>,
    start: (isize, isize), end: (isize,  isize),
    ignore_slopes: bool
) -> BTreeMap<(isize, isize), BTreeSet<((isize, isize), BTreeSet<(isize, isize)>)>> {    
    let mut graph = BTreeMap::new();
    graph.insert(start, BTreeSet::new());
    graph.insert(end, BTreeSet::new());
    for (&pos, &tile) in island.iter() {
        if tile == Tile::Forest {
            continue;
        }
        if NEIGHBORS
            .into_iter()
            .filter(|d| !island.get(&d.next(pos)).is_some_and(|&tile| tile == Tile::Forest))
            .count() > 2 {
            graph.insert(pos, BTreeSet::new());
        }
    }

    let nodes: BTreeSet<_> = graph
        .keys()
        .copied()
        .collect();
    for (&source, targets) in graph.iter_mut() {
        for pos in neighbors(&island, source, ignore_slopes) {
            let mut stack = vec![pos];
            let mut path = BTreeSet::new();
            path.insert(source);
            while let Some(pos) = stack.pop() {
                assert!(stack.is_empty());
                if nodes.contains(&pos) {
                    targets.insert((pos, path));
                    break;
                }
                path.insert(pos);
                match island.get(&pos).unwrap() {
                    Tile::Path => neighbors(&island, pos, ignore_slopes).difference(&path).for_each(|&pos| stack.push(pos)),
                    Tile::Forest => panic!("implementation error: walked into the forest"),
                    Tile::Slope(d) => if ignore_slopes {
                        neighbors(&island, pos, ignore_slopes).difference(&path).for_each(|&pos| stack.push(pos))
                    } else {
                        stack.push(d.next(pos))
                    },
                }
            }
        }
    }
    graph
}

fn most_scenic_route(
    graph: &BTreeMap<(isize, isize), BTreeSet<((isize, isize), BTreeSet<(isize, isize)>)>>,
    start: (isize, isize), end: (isize, isize)
) -> usize {
    let mut longest = 0;
    let mut stack = vec![(vec![start], 0)];
    while let Some((path, num_steps)) = stack.pop() {
        let pos = *path.last().unwrap();
        if pos == end {
            longest = longest.max(num_steps);
            continue;
        }
        for (next_pos, next_path) in graph.get(&pos).unwrap() {
            if path.contains(next_pos) {
                continue;
            }
            let mut path = path.clone();
            path.push(*next_pos);
            stack.push((path, num_steps + next_path.len()));
        }
    }
    longest
}

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
    let mut island = BTreeMap::new();
    for (y, line) in puzzle.split("\n").enumerate() {
        height = height.max((y+1) as isize);
        for (x, c) in line.chars().enumerate() {
            width = width.max((x+1) as isize);
            let pos = (y as isize, x as isize);
            let tile: Tile = c.into();
            island.insert(pos, tile);
        }
    }

    let mut start = (0, 0);
    let mut end = (0, 0);
    for x in 0..width {
        if *island.get(&(0, x)).unwrap() == Tile::Path {
            start = (0, x);
        }
        if *island.get(&(height-1, x)).unwrap() == Tile::Path {
            end = (height-1, x);
        }
    }

    let graph = get_directed_graph(&island, start, end, false);
    println!("part one = {}", most_scenic_route(&graph, start, end));
    let graph = get_directed_graph(&island, start, end, true);
    println!("part two = {}", most_scenic_route(&graph, start, end));
}
