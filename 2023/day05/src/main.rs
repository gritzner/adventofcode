use std::collections::VecDeque;

struct Interval {
    begin: usize,
    end: usize,
}

struct Map {
    sources: Vec<usize>,
    targets: Vec<usize>,
}

impl Map {
    fn new(s: &str) -> Self {
        let mut sources = vec![0, usize::MAX];
        let mut targets = sources.clone();
        for mapping in s.split("\n").skip(1) {
            let mapping: Vec<usize> = mapping.split(" ").map(|s| s.parse().expect("failed parsing mapping")).collect();
            let i = sources.binary_search(&mapping[1]);
            let i = if let Ok(i) = i {
                sources[i] = mapping[1];
                targets[i] = mapping[0];
                i
            } else {
                let i = i.unwrap_err();
                sources.insert(i, mapping[1]);
                targets.insert(i, mapping[0]);
                i
            };
            let end = mapping[1] + mapping[2];
            if sources[i+1] != end {
                sources.insert(i+1, end);
                targets.insert(i+1, end);
            }
        }
        Self { sources, targets }
    }

    fn map(&self, source: usize) -> usize {
        let i = self.sources.binary_search(&source);
        let i = if let Ok(i) = i { i } else { i.unwrap_err()-1 };
        self.targets[i] + source - self.sources[i]
    }

    fn map_range(&self, source: Interval) -> VecDeque<Interval> {
        let i = self.sources.binary_search(&source.begin);
        let i = if let Ok(i) = i { i } else { i.unwrap_err()-1 };
        let base = self.targets[i] + source.begin - self.sources[i];
        let mut targets = VecDeque::new();
        if source.end < self.sources[i+1] {
            targets.push_back(Interval {
                begin: base,
                end: base + source.end - source.begin
            });
        } else {
            targets.push_back(Interval {
                begin: base,
                end: base + self.sources[i+1] - source.begin - 1
            });
            targets.append(&mut self.map_range(Interval {
                begin: self.sources[i+1],
                end: source.end
            }));
        }
        targets
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
    let puzzle: Vec<_> = puzzle.split("\n\n").collect();
    let seeds: Vec<usize> = puzzle[0].split(" ").skip(1).map(|s| s.parse().expect("failed to parse seeds")).collect();
    let maps: Vec<Map> = puzzle.iter().skip(1).map(|map| Map::new(map)).collect();
    
    let part_one = seeds
        .iter()
        .map(|&seed| maps
            .iter()
            .fold(seed, |acc, map| map.map(acc))
        )
        .fold(usize::MAX, |acc, v| acc.min(v));
    println!("part one = {}", part_one);

    let mut minimum = usize::MAX;
    for i in (0..seeds.len()/2).into_iter().map(|i| 2*i) {
        let mut intervals = VecDeque::new();
        intervals.push_back(Interval {
            begin: seeds[i],
            end: seeds[i] + seeds[i+1] - 1,
        });
        for map in &maps {
            let n = intervals.len();
            for _ in 0..n {
                let front = intervals.pop_front().unwrap();
                intervals.append(&mut map.map_range(front));
            }
        }
        for interval in intervals {
            minimum = minimum.min(interval.begin);
        }
    }
    println!("part two = {}", minimum);
}
