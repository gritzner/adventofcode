use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Corner {
    TopLeft, TopRight, BottomRight, BottomLeft
}

fn get_lagoon_size(puzzle: &String, part_two: bool) -> usize {
    let puzzle: Vec<_> = puzzle
        .split("\n")
        .map(|line| {
            let line: Vec<_> = line.split(" ").collect();
            if part_two {
                let line: Vec<_> = line[2]  
                    .chars()
                    .skip(2)
                    .enumerate()
                    .map(|(i, c)| if i < 6 {
                        let j = c.to_digit(16).expect("failed to convert hex digit") as usize;
                        if i < 5 {
                            j * 16_usize.pow(4 - i as u32)
                        } else {
                            j
                        }
                    } else {
                        0
                    })
                    .collect();
                let n = line
                    .iter()
                    .enumerate()
                    .filter(|&(i, _v)| i < 5)
                    .map(|(_i, &v)| v)
                    .reduce(|acc, v| acc + v)
                    .unwrap() as isize;
                (["R", "D", "L", "U"][line[5]], n)
            } else {
                (line[0], line[1].parse::<isize>().expect("failed to parse input"))
            }
        })
        .collect();

    let mut corner_types = BTreeMap::new();
    corner_types.insert(("U", "R"), Corner::TopLeft);
    corner_types.insert(("L", "D"), Corner::TopLeft);
    corner_types.insert(("R", "D"), Corner::TopRight);
    corner_types.insert(("U", "L"), Corner::TopRight);
    corner_types.insert(("D", "L"), Corner::BottomRight);
    corner_types.insert(("R", "U"), Corner::BottomRight);
    corner_types.insert(("D", "R"), Corner::BottomLeft);
    corner_types.insert(("L", "U"), Corner::BottomLeft);

    let just_corners: Vec<_> = puzzle
        .iter()
        .enumerate()
        .map(|(i, &(d, _n))| {
            let j = if i > 0 { i - 1 } else { puzzle.len() - 1 };
            let prev_d = puzzle[j].0;
            *corner_types.get(&(prev_d, d)).expect("invalid puzzle")
        })
        .collect();
    
    let mut y = 0_isize;
    let mut x = 0_isize;
    let mut corners = BTreeMap::new();
    for ((d, n), corner) in puzzle.into_iter().zip(just_corners) {
        let row = corners.entry(y).or_insert_with(Vec::new);
        row.push((x, Some(corner)));
        match d {
            "U" => {
                for y2 in (y-n+1)..y {
                    let row = corners.entry(y2).or_insert_with(Vec::new);
                    row.push((x, None));
                }
                y -= n;
            },
            "R" => x += n,
            "D" => {
                for y2 in (y+1)..(y+n) {
                    let row = corners.entry(y2).or_insert_with(Vec::new);
                    row.push((x, None));
                }
                y += n;
            },
            "L" => x -= n,
            _ => panic!("invalid puzzle"),
        }
    }
    debug_assert!(y == 0 && x == 0);

    let mut sum = 0;
    for row in corners.values_mut() {
        row.sort_by_key(|x| x.0);
        let mut inside = false;
        let mut left = (Corner::TopLeft, false);
        for (i, (x, corner)) in row.iter().copied().enumerate() {
            if inside {
                sum += (x - row[i-1].0 - 1) as usize;
            }
            if let Some(corner) = corner {
                match corner {
                    Corner::TopLeft | Corner::BottomLeft => {
                        left = (corner, inside);
                        inside = true;
                    },
                    Corner::TopRight => {
                        if left.0 == Corner::BottomLeft {
                            inside = !left.1;
                        } else {
                            inside = left.1;
                        }
                    },
                    Corner::BottomRight => {
                        if left.0 == Corner::TopLeft {
                            inside = !left.1;
                        } else {
                            inside = left.1;
                        }
                    },
                };
            } else {
                inside = !inside;
            }
        }
        sum += row.len();
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

    println!("part one = {}", get_lagoon_size(&puzzle, false));
    println!("part two = {}", get_lagoon_size(&puzzle, true));
}
