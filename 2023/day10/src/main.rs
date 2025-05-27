use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardinalDirection {
    NORTH, EAST, SOUTH, WEST
}

const ALL_CARDINAL_DIRECTIONS: [CardinalDirection; 4] = [
    CardinalDirection::NORTH,
    CardinalDirection::EAST,
    CardinalDirection::SOUTH,
    CardinalDirection::WEST
];

impl CardinalDirection {
    fn go(&self, y: usize, x: usize, height: usize, width: usize) -> Option<(usize, usize)> {
        match self {
            CardinalDirection::NORTH => if y > 0 { Some((y-1, x)) } else { None },
            CardinalDirection::EAST => if x+1 < width { Some((y, x+1)) } else { None },
            CardinalDirection::SOUTH => if y+1 < height { Some((y+1, x)) } else { None },
            CardinalDirection::WEST => if x > 0 { Some((y, x-1)) } else { None },
        }
    }

    fn dir(source_y: usize, source_x: usize, target_yx: (usize, usize), height: usize, width: usize) -> usize {
        for (i, cd) in ALL_CARDINAL_DIRECTIONS.into_iter().enumerate() {
            if let Some(candidate) = cd.go(source_y, source_x, height, width) {
                if candidate == target_yx {
                    return i;
                }
            }
        }
        panic!("cannot reach target position from source position");
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
    let mut puzzle: Vec<Vec<char>> = puzzle
        .split("\n")
        .map(|row| row.chars().collect())
        .collect();
    let (height, width) = (puzzle.len(), puzzle[0].len());
    let (start_y, start_x) = puzzle
        .iter()
        .enumerate()
        .map(|(y, row)| (y, row.iter().enumerate().find(|(_x, c)| **c == 'S')))
        .filter(|(_y, x)| x.is_some())
        .map(|(y, x)| (y, x.unwrap().0))
        .next()
        .unwrap();

    let mut next = BTreeMap::new();
    next.insert((CardinalDirection::NORTH, '|'), CardinalDirection::NORTH);
    next.insert((CardinalDirection::SOUTH, '|'), CardinalDirection::SOUTH);
    next.insert((CardinalDirection::EAST, '-'), CardinalDirection::EAST);
    next.insert((CardinalDirection::WEST, '-'), CardinalDirection::WEST);
    next.insert((CardinalDirection::SOUTH, 'L'), CardinalDirection::EAST);
    next.insert((CardinalDirection::WEST, 'L'), CardinalDirection::NORTH);
    next.insert((CardinalDirection::SOUTH, 'J'), CardinalDirection::WEST);
    next.insert((CardinalDirection::EAST, 'J'), CardinalDirection::NORTH);
    next.insert((CardinalDirection::NORTH, '7'), CardinalDirection::WEST);
    next.insert((CardinalDirection::EAST, '7'), CardinalDirection::SOUTH);
    next.insert((CardinalDirection::NORTH, 'F'), CardinalDirection::EAST);
    next.insert((CardinalDirection::WEST, 'F'), CardinalDirection::SOUTH);

    let mut path = Vec::new();
    'outer: for cd in ALL_CARDINAL_DIRECTIONS {
        let mut cd = cd;
        path.clear();
        path.push(cd.go(start_y, start_x, height, width));
        while let &Some((y, x)) = path.last().unwrap() {
            if y == start_y && x == start_x {
                break 'outer;
            }
            if let Some(&next_cd) = next.get(&(cd, puzzle[y][x])) {
                cd = next_cd;
                path.push(cd.go(y, x, height, width));
            } else {
                path.push(None);
            }
        }
    }
    println!("part one = {}", path.len() / 2);

    let path: Vec<_> = path
        .into_iter()
        .map(|yx| yx.unwrap())
        .collect();
    let mut directions = [
        CardinalDirection::dir(start_y, start_x, path[0], height, width),
        CardinalDirection::dir(start_y, start_x, path[path.len()-2], height, width),
    ];
    directions.sort();
    let &(y, x) = path.last().unwrap();
    if directions == [0, 2] {
        puzzle[y][x] = '|';
    } else if directions == [1, 3] {
        puzzle[y][x] = '-';
    } else if directions == [0, 1] {
        puzzle[y][x] = 'L';        
    } else if directions == [0, 3] {
        puzzle[y][x] = 'J';
    } else if directions == [2, 3] {
        puzzle[y][x] = '7';
    } else if directions == [1, 2] {
        puzzle[y][x] = 'F';
    } else {
        panic!("cannot determine pipe type at starting position");
    }
    let path: BTreeSet<_> = path.into_iter().collect();

    let mut part_two = 0;
    for (y, row) in puzzle.iter().enumerate() {
        let mut inside = false;
        let mut from_north = true;
        for (x, _) in row.iter().enumerate() {
            if path.contains(&(y, x)) {
                inside = match puzzle[y][x] {
                    '|' => !inside,
                    'L' => {
                        from_north = true;
                        inside
                    },
                    'F' => {
                        from_north = false;
                        inside
                    },
                    '7' => if from_north { !inside } else { inside },
                    'J' => if !from_north { !inside } else { inside },
                    _ => inside,
                };
            } else if inside {
                part_two +=1 ;
            }
        }
    }
    println!("part two = {}", part_two);
}
