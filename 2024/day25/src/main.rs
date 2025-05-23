use std::collections::BTreeSet;

const MAX_HEIGHT: usize = 5;
const NUM_PINS: usize = 5;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle = puzzle
        .split("\n\n")
        .filter(|schematic| !schematic.is_empty())
        .map(|schematic| schematic
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| line
                .chars()
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    
    let mut locks = Vec::new();
    let mut keys = vec![vec![BTreeSet::new(); MAX_HEIGHT+1]; NUM_PINS];
    let mut next_key = 0_usize;
    for schematic in puzzle {
        if schematic[0][0] == '#' {
            let mut lock = Vec::with_capacity(NUM_PINS);
            for j in 0..NUM_PINS {
                for i in 0..MAX_HEIGHT+1 {
                    if schematic[i+1][j] == '.' {
                        lock.push(i);
                        break;
                    }
                }
            }
            locks.push(lock);
        } else {
            for j in 0..NUM_PINS {
                keys[j][0].insert(next_key);
                for i in 1..MAX_HEIGHT+1 {
                    if schematic[i][j] == '.' {
                        keys[j][i].insert(next_key);
                    }
                }
            }
            next_key += 1;
        }
    }

    let mut n = 0;
    for lock in locks {
        let mut fitting_keys = keys[0][lock[0]].clone();
        for (j, &i) in lock.iter().enumerate().skip(1) {
            fitting_keys = fitting_keys.intersection(&keys[j][i]).cloned().collect();
        }
        n += fitting_keys.len();
    }
    println!("[part one] fitting pairs = {}", n);
}
