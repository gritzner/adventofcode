use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)] //TODO: remove Debug
enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn direction(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)
        }
    }

    fn next_direction(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

fn next_pos(contents: &Vec<&str>, obstacle: Option<(isize, isize)>, pos: (isize, isize, Direction)) -> Option<(isize, isize, Direction)> {
    let direction = pos.2.direction();
    let i = pos.0 + direction.0;
    let j = pos.1 + direction.1;
    let o = obstacle.unwrap_or((-1, -1));
    contents.get(i as usize)
        .map(|line| line.chars().nth(j as usize))
        .filter(|c| c.is_some())
        .map(|c| if c == Some('#') || (i == o.0 && j == o.1) { (pos.0, pos.1, pos.2.next_direction()) } else { (i, j, pos.2) })
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let contents: Vec<_> = contents.split("\n").filter(|line| !line.is_empty()).collect();

    let mut initial_pos = None;
    for (i, line) in contents.iter().enumerate() {
        if let Some(j) = line.find("^") {
            initial_pos = Some((i as isize, j as isize, Direction::Up));
            break;
        }
    }
    assert!(initial_pos.is_some());

    let mut pos = initial_pos;
    let mut visited = BTreeSet::new();
    while let Some(current_pos) = pos {
        visited.insert((current_pos.0, current_pos.1));
        pos = next_pos(&contents, None, current_pos);
    }
    println!("visited.len() = {}", visited.len());

    let mut count = 0;
    for (i, j) in visited {
        if initial_pos.map(|pos| pos.0 == i && pos.1 == j).unwrap() {
            continue;
        }
        pos = initial_pos;
        let mut visited = BTreeSet::new();
        while let Some(current_pos) = pos {
            if !visited.insert(current_pos) {
                count += 1;
                break;
            }
            pos = next_pos(&contents, Some((i, j)), current_pos);
        }
    }
    println!("count = {}", count);
}
