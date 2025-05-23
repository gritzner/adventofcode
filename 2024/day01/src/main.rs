use std::collections::BTreeMap;

fn main() {
    let contents = std::fs::read_to_string("day01/input.txt").expect("Should have been able to read the file");
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut right_counts = BTreeMap::new();
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        for (i, s) in line.split("   ").enumerate() {
            match i {
                0 => left.push(s.parse::<i64>().expect("parsing to i64 failed for left list")),
                1 => {
                    let v = s.parse::<i64>().expect("parsing to i64 failed for right list");
                    right.push(v);
                    if let Some(count) = right_counts.get_mut(&v) {
                        *count += 1;
                    } else {
                        right_counts.insert(v, 1_i64);
                    }            
                },
                _ => panic!("more elements in line than expected")
            }            
        }
    }
    
    left.sort_unstable();
    right.sort_unstable();
    println!("lengths: {}, {}", left.len(), right.len());
    let mut sum = 0;
    let mut score = 0;
    for (&l, &r) in left.iter().zip(right.iter()) {
        sum += (l - r).abs();
        if let Some(&count) = right_counts.get(&l) {
            score += l * count;
        }
    }
    println!("sum = {}", sum);
    println!("similarity score = {}", score);
}
