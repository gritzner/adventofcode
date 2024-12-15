fn solve_part_one(mut robot_y: usize, mut robot_x: usize, mut map: Vec<Vec<char>>, movement: String) {
    let height = map.len();
    let width = map[0].len();

    for c in movement.chars() {
        let d = match c {
            '<' => [0, width-1],
            '>' => [0, 1],
            '^' => [height-1, 0],
            'v' => [1, 0],
            _ => panic!("invalid movement sequence")
        };
        let mut y = (robot_y + d[0]) % height;
        let mut x = (robot_x + d[1]) % width;
        while map[y][x] == 'O' {
            y = (y + d[0]) % height;
            x = (x + d[1]) % width;
        }
        if map[y][x] == '#' {
            continue;
        }
        robot_y = (robot_y + d[0]) % height;
        robot_x = (robot_x + d[1]) % width;
        if map[robot_y][robot_x] == '.' {
            continue;
        }
        map[robot_y][robot_x] = '.';
        map[y][x] = 'O';
    }

    let sum = map
        .iter()
        .enumerate()
        .fold(0, |acc, (y, row)| acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, &c)| acc + if c == 'O' { 100 * y + x } else { 0 })
        );
    println!("[part one] sum = {}", sum);
}

fn can_move_vert(map: &Vec<Vec<char>>, y: usize, x: usize, dy: usize, height: usize) -> bool {
    let y = (y + dy) % height;
    match map[y][x] {
        '.' => true,
        '[' => can_move_vert(map, y, x, dy, height) && can_move_vert(map, y, x+1, dy, height),
        ']' => can_move_vert(map, y, x, dy, height) && can_move_vert(map, y, x-1, dy, height),
        '#' => false,
        _ => panic!("invalid map")
    }
}

fn move_vert(map: &mut Vec<Vec<char>>, y: usize, x: usize, dy: usize, height: usize) {
    let y = (y + dy) % height;
    if map[y][x] == '[' {
        move_vert(map, y, x, dy, height);
        move_vert(map, y, x+1, dy, height);
        map[y][x] = '.';
        map[y][x+1] = '.';
        let y = (y + dy) % height;
        map[y][x] = '[';
        map[y][x+1] = ']';
    } else if map[y][x] == ']' {
        move_vert(map, y, x, dy, height);
        move_vert(map, y, x-1, dy, height);
        map[y][x] = '.';
        map[y][x-1] = '.';
        let y = (y + dy) % height;
        map[y][x] = ']';
        map[y][x-1] = '[';
    }
}

fn move_hor(row: &mut Vec<char>, x0: usize, dx: usize, width: usize) -> usize {
    let mut x = x0;
    loop {
        x = (x + dx) % width;
        if row[x] == '#' {
            return x0;
        } else if row[x] == '.' {
            break;
        }
    }
    let x0 = (x0 + dx) % width;
    let dx = width - dx;
    while x != x0 {
        let xx = (x + dx) % width;
        row[x] = row[xx];
        x = xx;
    }
    row[x0] = '.';
    x0
}

fn solve_part_two(mut robot_y: usize, mut robot_x: usize, map: Vec<Vec<char>>, movement: String) {
    robot_x *= 2;
    let mut map = map
        .into_iter()
        .map(|row| {
            let mut expanded_row = Vec::with_capacity(2 * row.len());
            for c in row {
                for expanded_c in match c {
                    '#' => "##",
                    'O' => "[]",
                    '.' => "..",
                    _ => panic!("invalid map")
                }.chars() {
                    expanded_row.push(expanded_c);
                }
            }
            expanded_row
        })
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();

    for c in movement.chars() {
        let d = match c {
            '<' => [0, width-1],
            '>' => [0, 1],
            '^' => [height-1, 0],
            'v' => [1, 0],
            _ => panic!("invalid movement sequence")
        };
        if d[0] == 0 {
            robot_x = move_hor(&mut map[robot_y], robot_x, d[1], width);
        } else if can_move_vert(&map, robot_y, robot_x, d[0], height) {
            move_vert(&mut map, robot_y, robot_x, d[0], height);
            robot_y = (robot_y + d[0]) % height;
        }
    }

    let sum = map
        .iter()
        .enumerate()
        .fold(0, |acc, (y, row)| acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (x, &c)| acc + if c == '[' { 100 * y + x } else { 0 })
        );
    println!("[part two] sum = {}", sum);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        println!("missing command line arguments: <input filename without .txt extension> <bool indicating whether to run part one (false) or two (true)>");
        return;
    }

    let content = std::fs::read_to_string(format!("day15/{}.txt", args[1])).expect("cannot read file with puzzle input");
    let mut content = content.split("\n\n");
    
    let mut robot_y = 0;
    let mut robot_x = 0;
    let map = content
        .next()
        .map(|map| map
            .split("\n")
            .enumerate()
            .map(|(y, row)| row
                .chars()
                .enumerate()
                .map(|(x, c)|
                    if c == '@' {
                        robot_y = y;
                        robot_x = x;
                        '.'
                    } else {
                        c
                    }
                )
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
        )
        .expect("map parsing failed");

    let movement = content
        .next()
        .map(|s| s
            .split("\n")
            .filter(|s| !s.is_empty())
            .collect::<String>()
        )
        .expect("movement parsing failed");

    if args[2].parse::<bool>().expect("second argument must be bool") {
        solve_part_two(robot_y, robot_x, map, movement);
    } else {
        solve_part_one(robot_y, robot_x, map, movement);
    }
}
