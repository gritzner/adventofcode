fn compute_cost(machines: &Vec<[u64; 6]>, offset: u64) -> u64 {
    let mut cost = 0;
    for &[ax, ay, bx, by, targetx, targety] in machines {
        let targetx = targetx + offset;
        let targety = targety + offset;
        // solve system of linear equations: a * [ax; ay] + b * [bx; by] = [targetx; targety]
        let det = (ax * by) as f64 - (ay * bx) as f64;
        let a = (((by * targetx) as f64 - (bx * targety) as f64) / det).round() as u64;
        let b = (((ax * targety) as f64 - (ay * targetx) as f64) / det).round() as u64;
        let x = a * ax + b * bx;
        let y = a * ay + b * by;
        if x != targetx || y != targety { // check if solution works as integers
            continue;
        }
        cost += 3 * a + b;
    }
    cost
}

fn main() {
    let content = std::fs::read_to_string("day13/input.txt").expect("should have been able to read the file");

    let mut buttons = Vec::new();
    let mut machines = Vec::new();
    for (i, line) in content.split("\n").filter(|line| !line.is_empty()).enumerate() {
        let j = i % 3;
        let split_char = match j {
            0..2 => '+',
            2 => '=',
            _ => panic!("should be unreachable")
        };
        let xy = line
            .split(":")
            .enumerate()
            .filter(|&(i, _s)| i == 1)
            .map(|(_i, s)| s
                .split(",")
                .map(|s| s
                    .trim()
                    .split(split_char)
                    .enumerate()
                    .filter(|&(i, _s)| i == 1)
                    .next()
                    .map(|(_i, s)| s
                        .parse::<u64>().expect("conversion to u64 failed")
                    )
                    .expect("this Option should not be able to be None")
                )
                .collect::<Vec<_>>()
            )
            .map(|v| [v[0], v[1]])
            .next()
            .expect("this Options should not be able to be None");
        buttons.push(xy);
        if j == 2 {
            let machine = [
                buttons[0][0], buttons[0][1],
                buttons[1][0], buttons[1][1],
                buttons[2][0], buttons[2][1],
            ];
            machines.push(machine);
            buttons.clear();
        }
    }

    println!("[part one] cost = {}", compute_cost(&machines, 0));
    println!("[part two] cost = {}", compute_cost(&machines, 10000000000000)); 
}
