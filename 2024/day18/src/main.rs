use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Vec2 {
    x: isize,
    y: isize,
    ux: usize,
    uy: usize
}

impl Vec2 {
    fn new() -> Self {
        Self::from_xy(0, 0)
    }

    fn from_xy(x: isize, y: isize) -> Self {
        Self {
            x, y,
            ux: x as usize,
            uy: y as usize
        }
    }

    fn from_string(s: &str) -> Self {
        let xy = s
            .split(",")
            .map(|v| v
                .parse::<isize>()
                .expect("conversion to isize failed")
            )
            .collect::<Vec<_>>();
        Self::from_xy(xy[0], xy[1])
   }

    fn neighbors(&self, size: &Vec2) -> Vec<Self> {
        let mut n = Vec::new();
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let mut pos = self.clone();
            pos.x += dx;
            pos.y += dy;
            if pos.x < 0 || pos.x >= size.x || pos.y < 0 || pos.y >= size.y {
                continue;
            }
            pos.ux = pos.x as usize;
            pos.uy = pos.y as usize;
            n.push(pos)
        }
        n
    }
}

fn get_num_steps(size: &Vec2, walls: &Vec<Vec2>, num_walls: usize) -> usize {
    let mut map = Vec::with_capacity(size.uy);
    for _y in 0..size.uy {
        map.push(vec![(true, None); size.ux]);
    }
    for &p in walls.iter().take(num_walls) {
        map[p.uy][p.ux] = (false, None);
    }

    let mut queue = VecDeque::new();
    queue.push_back(Vec2::new());
    while let Some(pos) = queue.pop_front() {
        for n in pos.neighbors(&size) {
            if !map[n.uy][n.ux].0 || map[n.uy][n.ux].1.is_some() {
                continue;
            }
            map[n.uy][n.ux] = (true, Some(pos));
            queue.push_back(n);
        }
    }
    map[0][0] = (true, None);

    let mut n = 0;
    let mut pos = Vec2::from_xy(size.x - 1, size.y - 1);
    while map[pos.uy][pos.ux].1.is_some() {
        pos = map[pos.uy][pos.ux].1.unwrap();
        n += 1;
    }
    n
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let content = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let mut content = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| Vec2::from_string(line))
        .collect::<Vec<_>>();

    // modified the puzzle input to include the map size and number of walls to drop for part one as header for easier testing
    let size = content.remove(0);
    let num_walls = content.remove(0);
    
    println!("[part one] # of steps = {}", get_num_steps(&size, &content, num_walls.ux));

    let mut a = 0;
    let mut b = content.len() - 1;
    while a <= b {
        let m = (a + b) / 2;
        if get_num_steps(&size, &content, m) == 0 {
            b = m - 1;
        } else {
            a = m + 1;
        }
    }
    while get_num_steps(&size, &content, b) > 0 {
        b += 1;
    }
    b -= 1;
    let pos = content[b];
    println!("[part two] coordinates = {},{}", pos.x, pos.y);
}
