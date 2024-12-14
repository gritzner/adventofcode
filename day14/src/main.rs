use std::collections::BTreeSet;

const HEIGHT: u64 = 103;
const WIDTH: u64 = 101;

const CENTER_Y: u64 = (HEIGHT - 1) / 2;
const CENTER_X: u64 = (WIDTH - 1) / 2;

fn state_after_time(robots: &Vec<[u64; 4]>, time: u64) -> (u64, u64, BTreeSet<(u64, u64)>) {
    let mut quadrants = [0_u64; 4];
    let mut occupied = BTreeSet::new();
    for &[px, py, vx, vy] in robots {
        let x = (px + time * vx) % WIDTH;
        let y = (py + time * vy) % HEIGHT;
        occupied.insert((x, y));
        if x == CENTER_X || y == CENTER_Y {
            continue;
        }
        let index = if x < CENTER_X { 0 } else { 2 } + if y < CENTER_Y { 0 } else { 1 };
        quadrants[index] += 1;
    }
    (quadrants.iter().fold(1, |acc, &v| acc * v), quadrants.iter().fold(0, |acc, &v| acc.max(v)), occupied)
}

fn main() {
    let content = std::fs::read_to_string("day14/input.txt").expect("should have been able to read the file");
    let robots = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line
            .split(" ")
            .map(|s| s
                .split("=")
                .skip(1)
                .map(|s| s
                    .split(",")
                    .map(|s| s
                        .parse::<i64>()
                        .expect("conversion to i64 failed")
                    )
                )
            )
            .flatten()
            .flatten()
            .collect::<Vec<_>>()
        )
        .map(|v| [
            v[0] as u64, v[1] as u64,
            (if v[2] >= 0 { v[2] } else { WIDTH as i64 + v[2] }) as u64,
            (if v[3] >= 0 { v[3] } else { HEIGHT as i64 + v[3] }) as u64,
        ])
        .collect::<Vec<_>>();

    println!("[part one] safety factor = {}", state_after_time(&robots, 100).0);
    
    let mut max = 0;
    let mut time = 0;
    for t in 1..(WIDTH * HEIGHT) { // assuming a repeat in patterns after WIDTH * HEIGHT steps
        let cluster_size = state_after_time(&robots, t).1; // most robots cluster close to together implies one quadrant probably sees a spike in the number for robots in that quadrant
        if cluster_size > max {
            max = cluster_size;
            time = t;
        }
    }
    println!("[part two] min. time until Easter egg = {}", time);

    let occupied = state_after_time(&robots, time).2;
    let mut state = Vec::new();
    for y in 0..HEIGHT {
        let mut row = Vec::new();
        for x in 0..WIDTH {
            row.push(if occupied.contains(&(x, y)) { '*' } else { '.' });
        }
        row.push('\n');
        state.push(row.iter().collect::<String>());
    }
    std::fs::write("day14/easter_egg.txt", state.into_iter().collect::<String>()).expect("writing Easter egg state to day14/easter_egg.txt failed");
}
