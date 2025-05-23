use std::collections::{BTreeMap, BTreeSet};

fn get_candidates(computers: &BTreeSet<usize>) -> Vec<BTreeSet<usize>> {
    let n = 2_usize.pow(computers.len() as u32);
    let mut candidates = Vec::with_capacity(n);
    let computers = computers.into_iter().map(|&v| v).collect::<Vec<_>>();
    for i in 0..n {
        let mut candidate = BTreeSet::new();
        for (j, &c) in computers.iter().enumerate() {
            if (i >> j) % 2 == 1 {
                candidate.insert(c);
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
            s.next().expect("invalid puzzle input"),
            s.next().expect("invalid puzzle input")
        ])
        .collect::<Vec<_>>();

    let mut computer_ids = BTreeMap::new();
    let mut computers = BTreeMap::new();
    for [c0, c1] in puzzle {
        let n = computer_ids.len();
        let c0 = *computer_ids.entry(c0).or_insert(n);
        let n = computer_ids.len();
        let c1 = *computer_ids.entry(c1).or_insert(n);
        computers.entry(c0).or_insert_with(BTreeSet::new).insert(c1);
        computers.entry(c1).or_insert_with(BTreeSet::new).insert(c0);
    }
    let mut max_connections = 0;
    for (&c, others) in &mut computers {
        others.insert(c);
        max_connections = max_connections.max(others.len());
    }
    let mut starts_with_t = BTreeSet::new();
    let mut ids_to_name = vec![""; computer_ids.len()];
    for (c, id) in computer_ids {
        ids_to_name[id] = c;
        if c.starts_with('t') {
            starts_with_t.insert(id);
        }
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
            if starts_with_t.contains(c) {
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
        let mut clique = cliques.into_iter().next().unwrap().into_iter().map(|id| ids_to_name[id]).collect::<Vec<_>>();
        clique.sort();
        let first = clique[0].to_owned();
        println!("[part two] largest clique = {}", clique.into_iter().skip(1).fold(first, |acc, s| format!("{},{}", acc, s)));
        break;
    }
}
