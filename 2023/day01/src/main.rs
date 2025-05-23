fn parse_line(line: &str, part_two: bool) -> usize {
    let mut digits: Vec<_> = line
        .chars()
        .enumerate()
        .filter(|(_i, c)| c.is_numeric())
        .map(|(i, c)| (i, c.to_string().parse::<usize>().unwrap()))
        .collect();
    if part_two {
        for (s, digit) in [("one",1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven",7), ("eight",8), ("nine",9)] {
            if let Some(i) = line.find(s) {
                digits.push((i, digit));
            }
            if let Some(i) = line.rfind(s) {
                digits.push((i, digit));
            }
        }
    }
    digits.sort_by_key(|x| x.0);
    10 * digits.first().unwrap().1 + digits.last().unwrap().1
}

fn solve(puzzle: &Vec<&str>, part_two: bool) -> usize {
    puzzle
        .iter()
        .map(|&line| parse_line(line, part_two))
        .fold(0, |acc, x| acc + x)
}

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle = puzzle.split("\n").collect::<Vec<_>>();
    println!("first part = {}", solve(&puzzle, false));
    println!("second part = {}", solve(&puzzle, true));
}
