use std::collections::BTreeMap;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");

    let fast_hash: Vec<usize> = (0..256)
        .map(|v| (17 * v) % 256)
        .collect();
    
    let sum = puzzle
        .split(",")
        .map(|step| step
            .as_bytes()
            .iter()
            .map(|v| *v as usize)
            .fold(0, |acc, v| (fast_hash[acc] + fast_hash[v]) % 256)            
        )
        .fold(0, |acc, v| acc + v);
    println!("part one = {}", sum);

    let mut boxes = vec![Vec::<usize>::new(); 256];
    let mut lenses = vec![BTreeMap::<String, usize>::new(); 256];
    for step in puzzle.split(",") {
        let lens: String = step
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        let hash = lens
            .as_bytes()
            .iter()
            .map(|c| *c as usize)
            .fold(0, |acc, v| (fast_hash[acc] + fast_hash[v]) % 256);
        let step: Vec<_> = step
            .chars()
            .filter(|c| !c.is_alphabetic())
            .collect();
        if step.len() == 1 {
            if let Some(&i) = lenses[hash].get(&lens) {
                boxes[hash].remove(i);
                lenses[hash].remove(&lens);
                for j in lenses[hash].values_mut() {
                    if *j > i {
                        *j -= 1;
                    }
                }
            }
        } else {
            debug_assert!(step.len() == 2);
            let focal_length = step[1].to_digit(10).expect("failed to parse focal length") as usize;
            if let Some(&i) = lenses[hash].get(&lens) {
                boxes[hash][i] = focal_length;
            } else {
                lenses[hash].insert(lens, boxes[hash].len());
                boxes[hash].push(focal_length);
            }
        }
    }
    let sum = boxes
        .into_iter()
        .enumerate()
        .map(|(i, the_box)| the_box
            .into_iter()
            .enumerate()
            .map(|(j, focal_length)| (i+1) * (j+1) * focal_length)
            .fold(0, |acc, v| acc + v)
        )
        .fold(0, |acc, v| acc + v);
    println!("part two = {}", sum);
}
