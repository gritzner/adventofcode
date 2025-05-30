use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock, Wall, Empty
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Rock,
            '#' => Tile::Wall,
            _ => Tile::Empty,
        }
    }
}

impl From<Tile> for String {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Rock => "O".to_owned(),
            Tile::Wall => "#".to_owned(),
            Tile::Empty => ".".to_owned(),
        }
    }
}

trait Platform {
    fn compute_load(&self) -> usize;
    fn tilt_north(&mut self);
    fn tilt_east(&mut self);
    fn tilt_south(&mut self);
    fn tilt_west(&mut self);
    fn to_string(&self) -> String;
}

fn from_string(s: String) -> Vec<Vec<Tile>> {
    s.split("\n")
        .map(|line| line
            .chars()
            .map(|c| c.into())
            .collect()
        )
        .collect()
}

impl Platform for Vec<Vec<Tile>> {
    fn compute_load(&self) -> usize {
        let mut load = 0;
        for x in 0..self[0].len() {
            for y in 0..self.len() {
                if self[y][x] == Tile::Rock {
                    load += self.len() - y;
                }
            }
        }
        load
    }

    fn tilt_north(&mut self) {
        for x in 0..self[0].len() {
            let mut first = 0;
            for y in 0..self.len() {
                match self[y][x] {
                    Tile::Rock => {
                        self[y][x] = Tile::Empty;
                        self[first][x] = Tile::Rock;
                        first += 1;
                    },
                    Tile::Wall => {
                        first = y + 1;
                    },
                    Tile::Empty => (),
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.len() {
            let mut first = self[y].len() - 1;
            for x in (0..self[y].len()).rev() {
                match self[y][x] {
                    Tile::Rock => {
                        self[y][x] = Tile::Empty;
                        self[y][first] = Tile::Rock;
                        first -= 1;
                    },
                    Tile::Wall => {
                        if x > 0 {
                            first = x - 1;
                        }
                    },
                    Tile::Empty => (),
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self[0].len() {
            let mut first = self.len() - 1;
            for y in (0..self.len()).rev() {
                match self[y][x] {
                    Tile::Rock => {
                        self[y][x] = Tile::Empty;
                        self[first][x] = Tile::Rock;
                        first -= 1;
                    },
                    Tile::Wall => {
                        if y > 0 {
                            first = y - 1;
                        }
                    },
                    Tile::Empty => (),
                }
            }
        }
    }
    fn tilt_west(&mut self) {
        for y in 0..self.len() {
            let mut first = 0;
            for x in 0..self[y].len() {
                match self[y][x] {
                    Tile::Rock => {
                        self[y][x] = Tile::Empty;
                        self[y][first] = Tile::Rock;
                        first += 1;
                    },
                    Tile::Wall => {
                        first = x + 1;
                    },
                    Tile::Empty => (),
                }
            }
        }
    }

    fn to_string(&self) -> String {
        let s: Vec<String> = self
            .into_iter()
            .map(|row| row
                .into_iter()
                .map(|&tile| tile.into())
                .collect::<Vec<String>>()
            )
            .map(|row| row.join(""))
            .collect();
        s.join("\n")
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("missing command line argument: <input filename without .txt extension>");
        return;
    }

    let path = args[0].split("/").last().expect("expecting to run in a terminal on macOS");
    let puzzle = std::fs::read_to_string(format!("{}/{}.txt", path, args[1])).expect("cannot read file with puzzle input");
    let puzzle: Vec<Vec<Tile>> = puzzle
        .split("\n")
        .map(|line| line
            .chars()
            .map(|c| c.into())
            .collect()
        )
        .collect();

    let mut once = puzzle.clone();
    once.tilt_north();
    println!("part one = {}", once.compute_load());

    let mut states = vec![puzzle.to_string()];
    let mut cache = BTreeMap::new();
    cache.insert(states[0].clone(), 0);
    let mut int_cache = BTreeMap::new();
    let mut current = 0;
    for _ in 0..1000000000 {
        current = *int_cache.entry(current).or_insert_with(|| {
            let mut next = from_string(states[current].clone());
            next.tilt_north();
            next.tilt_west();
            next.tilt_south();
            next.tilt_east();
            let next = next.to_string();
            let n = cache.len();
            *cache.entry(next.clone()).or_insert_with(|| {
                states.push(next);
                n
            })
       });
    }
    let puzzle = from_string(states[current].clone());
    println!("part two = {}", puzzle.compute_load());
}
