use std::collections::BTreeSet;

fn main() {
    let content = std::fs::read_to_string("day10/input.txt").expect("Should have been able to read the file");
    let map: Vec<_> = content
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).expect("conversion to u32 failed"))
            .collect::<Vec<_>>()
        )
        .collect();

    let mut heads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &n) in row.iter().enumerate() {
            if n != 0 {
                continue;
            }
            let mut tops = BTreeSet::new();
            let mut rating = 0;
            let mut stack = vec![(i, j)];
            while let Some(pos) = stack.pop() {
                let n = map[pos.0][pos.1];
                if n == 9 {
                    tops.insert(pos);
                    rating += 1;
                    continue;
                }
                for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let i = ((pos.0 as isize) + d.0) as usize; // -1 will become usize::MAX
                    let j = ((pos.1 as isize) + d.1) as usize; // -1 will become usize::MAX
                    if map.len() <= i || map[0].len() <= j {
                        continue;
                    }
                    if map[i][j] == n + 1 {
                        stack.push((i, j))
                    }
                }
            }
            if rating > 0 {
                heads.push((tops, rating));
            }
        }
    }
    let sum = heads.iter().map(|(tops, _rating)| tops.len()).fold(0, |acc, v| acc + v);
    println!("[part one] sum = {}", sum);
    let sum = heads.iter().map(|(_tops, rating)| rating).fold(0, |acc, v| acc + v);
    println!("[part two] sum = {}", sum);
}
