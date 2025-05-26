use std::collections::{BTreeMap, BTreeSet};
use num::integer::lcm;

fn main() {
        let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let mut puzzle = puzzle.split("\n\n");
    let path: Vec<_> = puzzle.next().unwrap().chars().map(|c| c == 'R').collect();
    let mut nodes = BTreeMap::new();
    let mut graph = BTreeMap::new();
    for node in puzzle.next().unwrap().split("\n") {
        let mut node = node.split(" = (");
        let source = node.next().unwrap();
        let mut node = node.next().unwrap().split(", ");
        let left = node.next().unwrap();
        let mut node = node.next().unwrap().split(")");
        let right = node.next().unwrap();
        let node: Vec<usize> = [source, left, right]
            .into_iter()
            .map(|node| {
                let n = nodes.len();
                *nodes.entry(node).or_insert(n)
            })
            .collect();
        graph.insert(node[0], (node[1], node[2]));
    }
    
    let destinations: BTreeSet<_> = nodes
        .iter()
        .filter(|(k, _v)| k.ends_with("Z"))
        .map(|(_k, v)| *v)
        .collect();
    let mut steps_map = BTreeMap::new();
    for (k, v) in nodes.into_iter().filter(|(k, _v)| k.ends_with("A")) {
        let mut node = v;
        let mut pos = 0;
        let mut num_steps = 0_usize;
        while !destinations.contains(&node) {
            let (left, right) = graph.get(&node).unwrap();
            if *path.get(pos).unwrap() {
                node = *right;
            } else {
                node = *left;
            }
            pos = (pos + 1) % path.len();
            num_steps += 1;
        }
        steps_map.insert(k, num_steps);
    }
    println!("part one = {}", steps_map.get("AAA").unwrap());
    println!("part two = {}", steps_map.values().fold(1, |acc, &v| lcm(acc, v)));
}
