use std::collections::VecDeque;

fn bfs(map: &mut Vec<Vec<Option<[usize; 2]>>>, start: [usize; 2], index: usize, height: usize, width: usize) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        let distance = map[pos[0]][pos[1]].unwrap()[index] + 1;
        for (dy, dx) in [(height - 1, 0), (0, 1), (1, 0), (0, width -1)] {
            let y = (pos[0] + dy) % height;
            let x = (pos[1] + dx) % width;
            if let Some(mut other) = map[y][x] {
                if other[index] > distance {
                    other[index] = distance;
                    map[y][x] = Some(other);
                    queue.push_back([y, x]);
                }
            }
        }
    }
}

fn get_good_cheats_up_to_distance(
    map: &Vec<Vec<Option<[usize; 2]>>>,
    y: isize, x: isize, d: isize,
    base_cost: usize, start_to_finish: usize,
    height: isize, width: isize
) -> usize {
    let mut n = 0;
    for y2 in (y-d).max(0)..(y+d+1).min(height) {
        for x2 in (x-d).max(0)..(x+d+1).min(width) {
            let actual_d = (x2 - x).abs() + (y2 - y).abs();
            if actual_d > d {
                continue;
            }
            if let Some(tile) = map[y2 as usize][x2 as usize] {
                let candidate = base_cost + (actual_d as usize) + tile[1];
                if candidate < start_to_finish && (start_to_finish - candidate) >= 100 {
                    n += 1;
                }
            }
        }
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
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");

    let mut start = [usize::MAX, usize::MAX];
    let mut end = [usize::MAX, usize::MAX];

    let mut map = puzzle
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| line
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                'S' => {
                    start = [y, x];
                    Some([0, usize::MAX])   
                },
                'E' => {
                    end = [y, x];
                    Some([usize::MAX, 0])
                },
                '.' => Some([usize::MAX, usize::MAX]),
                '#' => None,
                _ => panic!("invalid puzzle input")
            })
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();

    bfs(&mut map, start, 0, height, width);
    bfs(&mut map, end, 1, height, width);
    let start_to_finish = map[start[0]][start[1]].unwrap()[1];

    let mut n = [0, 0];
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Some(tile) = tile {
                n[0] += get_good_cheats_up_to_distance(
                    &map, y as isize, x as isize, 2,
                    tile[0], start_to_finish, height as isize, width as isize
                );
                n[1] += get_good_cheats_up_to_distance(
                    &map, y as isize, x as isize, 20,
                    tile[0], start_to_finish, height as isize, width as isize
                );
            }
        }
    }

    println!("[part one] # of cheats which save at least 100ps = {}", n[0]);
    println!("[part two] # of cheats which save at least 100ps = {}", n[1]);
}
