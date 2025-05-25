use std::collections::BTreeMap;
use regex::Regex;

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let mut symbols = BTreeMap::new();
    for (y, line) in puzzle.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_x, c)| !(c.is_numeric() || *c == '.')) {
            symbols.insert((x as isize, y as isize), if c == '*' { Some(Vec::new()) } else { None });
        }
    }

    let pattern = Regex::new(r"[0-9]+").unwrap();
    let mut sum = 0;
    for (y, line) in puzzle.split("\n").enumerate() {
        let y = y as isize;
        for m in pattern.find_iter(line) {
            let x = m.start() as isize;
            let v: usize = m.as_str().parse().expect("failed to parse part number into usize");
            let mut add_to_sum = true;
            for y in (y-1)..(y+2) {
                for x in (x-1)..(x+(m.len() as isize)+1) {
                    if let Some(maybe_gear) = symbols.get_mut(&(x, y)) {
                        if add_to_sum {
                            sum += v;
                            add_to_sum = false;
                        }
                        if let Some(gear) = maybe_gear {
                            gear.push(v);
                        }
                    }
                }
            }
        }
    }
    println!("part one = {}", sum);
    sum = symbols
        .into_iter()
        .filter(|(_pos, maybe_gear)| maybe_gear.is_some())
        .map(|(_pos, gear)| gear.unwrap())
        .filter(|gear| gear.len() == 2)
        .fold(0, |acc, gear| acc + gear[0] * gear[1]);
    println!("part two = {}", sum);
}
