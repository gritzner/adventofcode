use regex::Regex;

fn get_num_winning_combinations(puzzle: &str, remove_whitespace: bool, pattern: &Regex) -> usize {
    let mut puzzle = puzzle
        .split("\n")
        .map(|line| if remove_whitespace {
            line.replace(" ", "")
        } else {
            line.into()
        })
        .map(|line| pattern
            .find_iter(&line)
            .map(|m| m.as_str().parse().expect("failed to parse usize"))
            .collect()
        );
    let times: Vec<usize> = puzzle.next().expect("failed to parse times");
    let distances = puzzle.next().expect("failed to parse distances");

    let mut winning_combinations = 1;
    for (t, d) in times.into_iter().zip(distances.into_iter()) {
        let t = t as f64;
        let d = d as f64;
        let d = (t.powi(2) - 4.0 * d).sqrt();
        let mut least = 0.5 * (t - d);
        if least.fract() == 0.0 {
            least += 0.5;
        }
        let mut most = 0.5 * (t + d);
        if most.fract() == 0.0 {
            most -= 0.5;
        }
        winning_combinations *= (most.floor() - least.ceil()) as usize + 1;
    }    
    winning_combinations
}

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let pattern = Regex::new(r"[0-9]+").unwrap();
    println!("part one = {}", get_num_winning_combinations(&puzzle, false, &pattern));
    println!("part two = {}", get_num_winning_combinations(&puzzle, true, &pattern));
}
