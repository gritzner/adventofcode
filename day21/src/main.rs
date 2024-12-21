use std::collections::BTreeMap;

const NUMERIC_BUTTONS: [char; 11] = ['A', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A']
];
const NUMERIC_BUTTON_POSITIONS: [(usize, usize); 11] = [
    (3, 2), (3, 1), // A, 0
    (2, 0), (2, 1), (2, 2), // 1, 2, 3
    (1, 0), (1, 1), (1, 2), // 4, 5, 6
    (0, 0), (0, 1), (0, 2) // 7, 8, 9
];
const NUMERIC_KEYPAD_NEIGHBORS: [(usize, usize); 4] = [(3, 0), (0, 1), (1, 0), (0, 2)]; // up, right, down, left

const DIRECTIONAL_BUTTONS: [char; 5] = ['A', '^', '<', 'v', '>'];
const DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>']
];
const DIRECTIONAL_BUTTON_POSITIONS: [(usize, usize); 5] = [
    (0, 2), (0, 1), // A, ^
    (1, 0), (1, 1), (1, 2) // <, v, >
];
const DIRECTIONAL_KEYPAD_NEIGHBORS: [(usize, usize); 3] = [(1, 0), (0, 1), (0, 2)]; // vertical, right, left

fn get_paths(buttons: &[char], keypad: &[[char; 3]], positions: &[(usize, usize)], neighbors: &[(usize, usize)]) -> BTreeMap<char, BTreeMap<char, Vec<String>>> {
    let mut paths = BTreeMap::new();
    for (i, &button) in buttons.into_iter().enumerate() {
        let mut targets = BTreeMap::<char, Vec<String>>::new();
        let mut stack = vec![(positions[i], Vec::new())];
        while let Some(((y, x), path)) = stack.pop() {
            let button = keypad[y][x];
            let paths = targets.entry(button).or_insert_with(Vec::new);
            if paths.is_empty() || path.len() < paths[0].len() {
                paths.clear();
                paths.push(path.iter().map(|(_button, direction)| *direction).collect());
            } else if path.len() == paths[0].len() {
                paths.push(path.iter().map(|(_button, direction)| *direction).collect());
            }
            for (dy, dx) in neighbors {
                let y2 = (y + dy) % keypad.len();
                let x2 = (x + dx) % 3; // width of the keypads
                let d = (y2 as isize - y as isize).abs().max((x2 as isize - x as isize).abs());
                let other = keypad[y2][x2];
                if d > 1 || other == ' ' || path.iter().filter(|(button, _direction)| *button == other).count() > 0 {
                    continue;
                }
                let d = if y2 == y {
                    if x2 < x { '<' } else { '>' }
                } else {
                    if y2 < y { '^' } else { 'v' }
                };
                let mut path = path.clone();
                path.push((button, d));
                stack.push(((y2, x2), path));
            }
        }
        paths.insert(button, targets);
    }
    paths
}

fn get_length(
    paths: &BTreeMap<char, BTreeMap<char, Vec<String>>>, other_paths: &BTreeMap<char, BTreeMap<char, Vec<String>>>,
    path: String, current_depth: usize, max_depth: usize,
    cache: &mut BTreeMap<(usize, String), usize>
) -> usize {
    let mut n = 0;
    for (i, second) in path.chars().enumerate() {
        let first = if i == 0 { 'A' } else { path.chars().nth(i-1).unwrap() };
        let mut k = usize::MAX;
        for s in &paths[&first][&second] {
            let mut s = s.clone();
            s.push('A');
            k = k.min(
                if current_depth < max_depth {
                    if !cache.contains_key(&(current_depth, s.clone())) {
                        let k = get_length(other_paths, other_paths, s.clone(), current_depth+1, max_depth, cache);
                        cache.insert((current_depth, s), k);
                        k
                    } else {
                        *cache.get(&(current_depth, s)).unwrap()
                    }
                } else {
                    s.len()
                }
            );
        }
        n += k;
    }
    n
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
        .map(|line| line.to_owned())
        .collect::<Vec<_>>();

    let numeric_paths = get_paths(&NUMERIC_BUTTONS, &NUMERIC_KEYPAD, &NUMERIC_BUTTON_POSITIONS, &NUMERIC_KEYPAD_NEIGHBORS);
    let directional_paths = get_paths(&DIRECTIONAL_BUTTONS, &DIRECTIONAL_KEYPAD, &DIRECTIONAL_BUTTON_POSITIONS, &DIRECTIONAL_KEYPAD_NEIGHBORS);

    for (n, label) in [(2, "one"), (25, "two")] {
        let mut cache = BTreeMap::new();
        let mut complexity = 0;
        for puzzle in &puzzle {
            let mut k = puzzle.clone();
            k.pop();
            let k = k.parse::<usize>().expect("invalid puzzle input");
            complexity += k * get_length(&numeric_paths, &directional_paths, puzzle.clone(), 0, n, &mut cache);
        }
        println!("[part {}] sum of complexities = {}", label, complexity);
    }
}
