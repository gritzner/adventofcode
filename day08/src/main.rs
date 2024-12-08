use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut antennae = BTreeMap::new();
    let mut height = 0;
    let mut width = 0;
    for (i, line) in contents.split("\n").filter(|line| !line.is_empty()).enumerate() {
        height = height.max(i as isize); // assumes that the last row in the input contains an antenna (true for my input)
        for (j, c) in line.chars().enumerate().filter(|&(_j, c)| c != '.') {
            width = width.max(j as isize); // assumes that the last column in the input contains an antenna (true for my input)
            let same_freq_antennae = antennae.entry(c).or_insert_with(Vec::new);
            same_freq_antennae.push((i as isize, j as isize));
        }
    }

    // part one
    let mut antinodes = BTreeSet::new();
    for same_freq_antennae in antennae.values() {
        for (i, &pos) in same_freq_antennae.iter().enumerate() {
            for (_j, &other_pos) in same_freq_antennae.iter().enumerate().filter(|&(j, _other_pos)| i != j) {
                let pos = (2 * other_pos.0 - pos.0, 2 * other_pos.1 - pos.1);
                if 0 <= pos.0 && pos.0 <= height && 0 <= pos.1 && pos.1 <= width {
                    antinodes.insert(pos);
                }
            }
        }
    }
    println!("[part one] antinode count = {}", antinodes.len());

    // part two
    antinodes.clear();
    for same_freq_antennae in antennae.values() {
        for (i, &pos) in same_freq_antennae.iter().enumerate() {
            antinodes.insert(pos);
            for (_j, &other_pos) in same_freq_antennae.iter().enumerate().filter(|&(j, _other_pos)| i != j) {
                let delta = (other_pos.0 - pos.0, other_pos.1 - pos.1);
                let mut k = 1;
                loop  {
                    let pos = (other_pos.0 + k * delta.0, other_pos.1 + k * delta.1);
                    if pos.0 < 0 || height < pos.0 || pos.1 < 0 || width < pos.1 {
                        break;
                    }
                    antinodes.insert(pos);
                    k += 1;
                }
            }
        }
    }
    println!("[part two] antinode count = {}", antinodes.len());
}
