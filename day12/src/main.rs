use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardinalDirection {
    North, East, South, West
}

impl CardinalDirection {
    fn step(&self, i: usize, j: usize) -> (usize, usize) {
        let (di, dj) = match self {
            CardinalDirection::North => (-1, 0),
            CardinalDirection::East => (0, 1),
            CardinalDirection::South => (1, 0),
            CardinalDirection::West => (0, -1),
        };
        let i = ((i as isize) + di) as usize;
        let j = ((j as isize) + dj) as usize;
        (i, j)
    }

    fn search_directions(&self) -> [CardinalDirection; 2] {
        match self {
            CardinalDirection::North | CardinalDirection::South => [CardinalDirection::East, CardinalDirection::West],
            CardinalDirection::East | CardinalDirection::West => [CardinalDirection::North, CardinalDirection::South]
        }
    }
}

const NEIGHBORS: [CardinalDirection; 4] = [
    CardinalDirection::North, CardinalDirection::East, CardinalDirection::South, CardinalDirection::West
];

fn main() {
    let content = std::fs::read_to_string("day12/input.txt").expect("Should have been able to read the file");
    let map: Vec<_> = content
        .split("\n")
        .filter(|line|  !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let height = map.len();
    let width = map[0].len();
    let mut visited = BTreeSet::new();
    let mut cost = 0;
    let mut bulk_cost = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let plot = (i, j);
            if visited.contains(&plot) {
                continue;
            }

            let mut region = BTreeSet::new();
            region.insert(plot);
            visited.insert(plot);
            let mut search_perimeter = vec![plot];
            while let Some((i, j)) = search_perimeter.pop() {
                for cd in NEIGHBORS {
                    let (i2, j2) = cd.step(i, j);
                    let plot = (i2, j2);
                    if i2 >= height || j2 >= width || map[i2][j2] != c || visited.contains(&plot) {
                        continue;
                    }
                    region.insert(plot);
                    visited.insert(plot);
                    search_perimeter.push(plot);
                }
            }

            let mut perimeter = BTreeSet::new();
            for &(i, j) in &region {
                for cd in NEIGHBORS {
                    if !region.contains(&cd.step(i, j)) {
                        perimeter.insert((i, j, cd));
                    }
                }
            }

            let mut sides = 0;
            let mut perimeter_visited = BTreeSet::new();
            for fence in &perimeter {
                if perimeter_visited.contains(fence) {
                    continue;
                }
                let &(i, j, cd) = fence;
                for search_cd in cd.search_directions() {
                    let mut i2 = i;
                    let mut j2 = j;
                    while perimeter.contains(&(i2, j2, cd)) {
                        perimeter_visited.insert((i2, j2, cd));
                        (i2, j2) = search_cd.step(i2, j2);
                    }
                }
                sides += 1;
            }

            cost += region.len() * perimeter.len();
            bulk_cost += region.len() * sides;
        }
    }

    println!("[part one] cost = {}\n[part two] bulk order cost = {}", cost, bulk_cost);
}
