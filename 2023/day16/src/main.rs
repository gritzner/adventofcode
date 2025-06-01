use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North, East, South, West
}

impl Direction {
    fn next(&self, y: usize, x: usize, height: usize, width: usize) -> Option<(usize, usize)> {
        match self {
            Direction::North => if y > 0 { Some((y-1, x)) } else { None },
            Direction::East => if x+1 < width { Some((y, x+1)) } else { None },
            Direction::South => if y+1 < height { Some((y+1, x)) } else { None },
            Direction::West => if x > 0 { Some((y, x-1)) } else { None },
        }
    }
}

fn move_beam(stack: &mut Vec<(usize, usize, Direction)>, y: usize, x: usize, d: Direction, height: usize, width: usize) {
    if let Some((next_y, next_x)) = d.next(y, x, height, width) {
        stack.push((next_y, next_x, d));
    }
}

fn get_num_energized_tiles(puzzle: &Vec<Vec<char>>, initial_y: usize, initial_x: usize, initial_d: Direction, height: usize, width: usize) -> usize {
    let mut stack = vec![(initial_y, initial_x, initial_d)];
    let mut visited = BTreeSet::new();
    while let Some((y, x, d)) = stack.pop() {
        if visited.contains(&(y, x, d)) {
            continue;
        }
        visited.insert((y, x, d));
        match puzzle[y][x] {
            '/' => match d {
                Direction::North => move_beam(&mut stack, y, x, Direction::East, height, width),
                Direction::East => move_beam(&mut stack, y, x, Direction::North, height, width),
                Direction::South => move_beam(&mut stack, y, x, Direction::West, height, width),
                Direction::West => move_beam(&mut stack, y, x, Direction::South, height, width),
            },
            '\\' => match d {
                Direction::North => move_beam(&mut stack, y, x, Direction::West, height, width),
                Direction::East => move_beam(&mut stack, y, x, Direction::South, height, width),
                Direction::South => move_beam(&mut stack, y, x, Direction::East, height, width),
                Direction::West => move_beam(&mut stack, y, x, Direction::North, height, width),
            },
            '|' => {
                if d == Direction::North || d == Direction::South {
                    move_beam(&mut stack, y, x, d, height, width)
                } else {
                    for next_d in [Direction::North, Direction::South] {
                        move_beam(&mut stack, y, x, next_d, height, width)
                    }
                }
            },
            '-' => {
                if d == Direction::East || d == Direction::West {
                    move_beam(&mut stack, y, x, d, height, width)
                } else {
                    for next_d in [Direction::East, Direction::West] {
                        move_beam(&mut stack, y, x, next_d, height, width)
                    }
                }
            },
            _ => move_beam(&mut stack, y, x, d, height, width),
        };
    }
    let energized: BTreeSet<_> = visited
        .into_iter()
        .map(|(y, x, _d)| (y, x))
        .collect();
    energized.len()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle: Vec<Vec<_>> = puzzle
        .split("\n")
        .map(|line| line
            .chars()
            .collect()
        )
        .collect();
    let height = puzzle.len();
    let width = puzzle[0].len();

    let mut num_energized = Vec::new();
    for y in 0..height {
        num_energized.push(get_num_energized_tiles(&puzzle, y, 0, Direction::East, height, width));
        num_energized.push(get_num_energized_tiles(&puzzle, y, width-1, Direction::West, height, width));
    }
    for x in 0..width {
        num_energized.push(get_num_energized_tiles(&puzzle, 0, x, Direction::South, height, width));
        num_energized.push(get_num_energized_tiles(&puzzle, height-1, x, Direction::North, height, width));
    }
    println!("part one = {}", num_energized[0]);
    println!("part two = {}", num_energized.into_iter().fold(0, |acc, v| acc.max(v)));
}
