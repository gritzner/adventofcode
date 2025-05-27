use std::collections::BTreeSet;

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle: Vec<_> = puzzle
        .split("\n")
        .enumerate()
        .map(|(y, row)| row
            .chars()
            .enumerate()
            .filter(|&(_x, c)| c == '#')
            .map(|(x, _c)| (y, x))
            .collect::<Vec<_>>()
        )
        .flatten()
        .collect();

    let height = puzzle.iter().map(|&(y, _x)| y).fold(0, |acc, v| acc.max(v));
    let empty_rows: BTreeSet<_> = (0..height)
        .into_iter()
        .filter(|&y| !puzzle
            .iter()
            .any(|(gy, _gx)| *gy == y)
        )
        .collect();

    let width = puzzle.iter().map(|&(_y, x)| x).fold(0, |acc, v| acc.max(v));
    let empty_cols: BTreeSet<_> = (0..width)
        .into_iter()
        .filter(|&x| !puzzle
            .iter()
            .any(|(_gy, gx)| *gx == x)
        )
        .collect();

    for (factor, label) in [(1, "one"), (999999, "two")] {
        let puzzle: Vec<_> = puzzle
            .iter()
            .map(|&(y, x)| (
                (y + factor * empty_rows.iter().filter(|row| **row < y).count()) as isize,
                (x + factor * empty_cols.iter().filter(|col| **col < x).count()) as isize
            ))
            .collect();

        let mut sum = 0;
        for (i, &yx) in puzzle.iter().enumerate() {
            for &other_yx in puzzle.iter().skip(i+1) {
                sum += (yx.0 - other_yx.0).abs() + (yx.1 - other_yx.1).abs();
            }
        }
        println!("part {} = {}", label, sum);
    }
}
