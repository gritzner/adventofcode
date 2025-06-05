use std::collections::{BTreeMap, BTreeSet};

fn is_at_ground_level(cubes: &BTreeSet<(usize, usize, usize)>) -> bool {
    cubes
        .iter()
        .any(|pos| pos.2 == 1)
}

fn shift_down(cubes: &BTreeSet<(usize, usize, usize)>) -> BTreeSet<(usize, usize, usize)> {
    cubes
        .iter()
        .copied()
        .map(|pos| (pos.0, pos.1, pos.2-1))
        .collect()
}

fn get_sum_chain_reactions(supports: &Vec<Vec<usize>>, supported_by: &Vec<BTreeSet<usize>>) -> usize {
    let mut sum = 0;
    for (i, supports_i) in supports.iter().enumerate() {
        let mut disintegrated = BTreeSet::new();
        disintegrated.insert(i);
        let mut stack = supports_i.clone();
        while let Some(i) = stack.pop() {
            if supported_by[i].difference(&disintegrated).count() == 0 {
                sum += 1;
                disintegrated.insert(i);
                stack.append(&mut supports[i].clone());
            }
        }
    }
    sum
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    
    let mut puzzle: Vec<_> = puzzle
        .split("\n")
        .map(|brick| {
            let brick: Vec<Vec<usize>> = brick
                .split("~")
                .map(|v| {
                    let v: Vec<usize> = v
                        .split(",")
                        .map(|v| v
                            .parse()
                            .expect("failed to parse input")
                        )
                        .collect();
                    assert_eq!(v.len(), 3);
                    v
                })
                .collect();
            assert_eq!(brick.len(), 2);
            brick[0]
                .iter()
                .copied()
                .zip(brick[1]
                    .iter()
                    .copied()
                )
                .map(|(a, b)| (a.min(b), a.max(b)+1))
                .collect::<Vec<_>>()
        })
        .collect();
    puzzle.sort_by_key(|brick| brick[2].0);

    let mut stack = BTreeMap::new();
    let mut bricks = Vec::new();
    for (i, brick) in puzzle.into_iter().enumerate() {
        let mut cubes = BTreeSet::new();
        for x in brick[0].0..brick[0].1 {
            for y in brick[1].0..brick[1].1 {
                for z in brick[2].0..brick[2].1 {
                    let pos = (x, y, z);
                    stack.insert(pos, i);
                    cubes.insert(pos);
                }
            }
        }
        bricks.push(cubes);
    };
    
    let mut supported_by = vec![BTreeSet::new(); bricks.len()];
    loop {
        supported_by.iter_mut().for_each(|brick| brick.clear());
        for (i, cubes) in bricks.iter().enumerate() {
            if is_at_ground_level(cubes) {
                supported_by[i].insert(usize::MAX);
            }
            for pos in shift_down(cubes) {
                if let Some(&j) = stack.get(&pos) {
                    if j != i {
                        supported_by[i].insert(j);
                    }
                }
            }
        }

        let mut all_settled = true;
        for (i, (brick, supported_by)) in bricks.iter_mut().zip(supported_by.iter()).enumerate() {
            if !supported_by.is_empty() {
                continue;
            }
            all_settled = false;
            for pos in brick.iter() {
                stack.remove(pos);
            }
            let mut new_cubes = shift_down(brick);
            brick.clear();
            brick.append(&mut new_cubes);
            for pos in brick.iter() {
                stack.insert(pos.clone(), i);
            }
        }

        if all_settled {
            break;
        }
    }

    let mut supports = vec![Vec::new(); bricks.len()];
    for (i, supported_by) in supported_by.iter().enumerate() {
        for &j in supported_by {
            if j < usize::MAX {
                supports[j].push(i);
            }
        }
    }

    let n = supports
        .iter()
        .filter(|supports| !supports
            .into_iter()
            .any(|&i| supported_by[i].len() == 1)
        )
        .count();
    println!("part one = {}", n);
    println!("part two = {}", get_sum_chain_reactions(&supports, &supported_by));
}
