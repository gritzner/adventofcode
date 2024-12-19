fn build_graph(edges: &Vec<&str>, s: &str) -> Vec<Vec<usize>> {
    // vec of vertices with each vertex a vec of incoming edges
    let mut vertices = vec![Vec::new(); s.len()+1];
    for i in 0..s.len() {
        for &e in edges {
            let j = i + e.len();
            if j > s.len() {
                continue;
            }
            if s.get(i..j).unwrap() == e {
                vertices[j].push(i);
            }
        }
    }
    vertices
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let mut puzzle = puzzle
        .split("\n")
        .filter(|line| !line.is_empty());
    
    let towels = puzzle
        .next()
        .expect("invalid puzzle input")
        .split(", ")
        .collect::<Vec<_>>();

    let mut n = 0;
    let mut k = 0;
    for pattern in puzzle {
        let graph = build_graph(&towels, pattern);
        let mut kk = vec![0_u64; pattern.len()+1];
        kk[0] = 1; // start vertex (= empty string)
        for j in 1..pattern.len()+1 {
            for &v in &graph[j] {
                kk[j] += kk[v];
            }
        }
        k += kk[pattern.len()];
        if kk[pattern.len()] > 0 {
            n += 1;
        }
    }
    println!("[part one] patterns possible = {}", n);
    println!("[part two] possible variations = {}", k);
}
