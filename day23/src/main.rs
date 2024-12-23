use std::collections::{BTreeMap, BTreeSet};

fn get_candidates(computers: &BTreeSet<String>) -> Vec<BTreeSet<String>> {
    let n = 2_usize.pow(computers.len() as u32);
    let mut candidates = Vec::with_capacity(n);
    let computers = computers.into_iter().collect::<Vec<_>>();
    for i in 0..n {
        let mut candidate = BTreeSet::new();
        for (j, &c) in computers.iter().enumerate() {
            if (i >> j) % 2 == 1 {
                candidate.insert(c.clone());
            }
        }
        candidates.push(candidate);
    }
    candidates
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle = puzzle
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line
            .split("-")
        )
        .map(|mut s| [
            s.next().expect("invalid puzzle input").to_owned(), // could probably improve performance by mapping all computer names to unique integers instead
            s.next().expect("invalid puzzle input").to_owned()
        ])
        .collect::<Vec<_>>();

    let mut computers = BTreeMap::new();
    for [c0, c1] in puzzle {
        computers.entry(c0.clone()).or_insert_with(BTreeSet::new).insert(c1.clone());
        computers.entry(c1).or_insert_with(BTreeSet::new).insert(c0);
    }
    let mut max_connections = 0;
    for (c, others) in &mut computers {
        others.insert(c.clone());
        max_connections = max_connections.max(others.len());
    }

    let mut cliques = vec![BTreeSet::new(); max_connections + 1];
    for candidates in computers.values() {
        for candidate in get_candidates(candidates) {
            let mut is_subset = true;
            for c in &candidate {
                is_subset = is_subset && candidate.is_subset(computers.get(c).unwrap());
            }
            if is_subset {
                cliques[candidate.len()].insert(candidate);
            }
        }
    }

    let mut n = 0;
    'outer: for clique in &cliques[3] {
        for c in clique {
            if c.starts_with('t') {
                n += 1;
                continue 'outer;
            }
        }
    }
    println!("[part one] n = {}", n);

    for cliques in cliques.into_iter().rev() {
        if cliques.is_empty() {
            continue;
        }
        let mut clique = cliques.into_iter().next().unwrap().into_iter().collect::<Vec<_>>();
        clique.sort();
        let first = clique[0].clone();
        println!("[part two] largest clique = {}", clique.into_iter().skip(1).fold(first, |acc, s| format!("{},{}", acc, s)));
        break;
    }
}
