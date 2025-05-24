struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl From<&str> for Cubes {
    fn from(s: &str) -> Self {
        let mut result = Self { red: 0, green: 0, blue: 0 };
        for s in s.split(", ") {
            let mut s = s.split(" ");
            let (count, color): (usize, &str) = (s.next().unwrap().parse().expect("failed to parse cube count"), s.next().unwrap());
            match color {
                "red" => result.red += count,
                "green" => result.green += count,
                "blue" => result.blue += count,
                _ => panic!("found unknown color '{}'", color)
            }
        }
        result
    }
}

trait ValidCubeCount {
    fn is_valid_cube_count(&self, limits: &Cubes) -> bool;
}

impl ValidCubeCount for Cubes {
    fn is_valid_cube_count(&self, limits: &Cubes) -> bool {
        self.red <= limits.red && self.green <= limits.green && self.blue <= limits.blue
    }
}

impl ValidCubeCount for Vec<Cubes> {
    fn is_valid_cube_count(&self, limits: &Cubes) -> bool {
        self.iter().all(|cubes| cubes.is_valid_cube_count(limits))
    }
}

struct Game {
    id: usize,
    samples: Vec<Cubes>,
}

impl Game {
    fn get_power(&self) -> usize {
        let mut minimum_cubes = Cubes { red: 0, green: 0, blue: 0 };
        for cubes in &self.samples {
            minimum_cubes.red = minimum_cubes.red.max(cubes.red);
            minimum_cubes.green = minimum_cubes.green.max(cubes.green);
            minimum_cubes.blue = minimum_cubes.blue.max(cubes.blue);
        }
        minimum_cubes.red * minimum_cubes.green * minimum_cubes.blue
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut s = s.split(": ");
        let (id, samples) = (s.next().unwrap(), s.next().unwrap());
        Self { 
            id: id.split(" ").last().unwrap().parse().expect("failed to parse game ID"),
            samples: samples.split("; ").map(|sample| sample.into()).collect(),
        }
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
    let puzzle: Vec<Game> = puzzle.split("\n").map(|line| line.into()).collect();

    const LIMITS: Cubes = Cubes { red: 12, green: 13, blue: 14 };
    let part_one = puzzle
        .iter()
        .filter(|game| game.samples.is_valid_cube_count(&LIMITS))
        .map(|game| game.id)
        .fold(0_usize, |acc, x| acc + x);
    println!("part one = {}", part_one);

    let part_two = puzzle
        .iter()
        .map(|game| game.get_power())
        .fold(0_usize, |acc, x| acc + x);
    println!("part two = {}", part_two);
}
