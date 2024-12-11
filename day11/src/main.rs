use std::collections::BTreeMap;

fn blink(stone: usize) -> (usize, Option<usize>) {
    if stone == 0 {
        (1, None)
    } else {
        let mut n = 1;
        let mut n10 = 10; // 10 to the power of n
        while stone >= n10 {
            n += 1;
            n10 *= 10;
        }
        if n % 2 == 1 {
            (2024 * stone, None)
        } else {
            n10 = 10_usize.pow(n / 2);
            (stone / n10, Some(stone % n10))
        }        
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let stones_vec: Vec<_> = content
        .split("\n")
        .next()
        .expect("failed fetching first line")
        .split(" ")
        .map(|s| s.parse::<usize>().expect("failed to parse into usize"))
        .collect();

    let mut stones = BTreeMap::new();
    for stone in stones_vec {
        let count = stones.entry(stone).or_insert(0_usize);
        *count += 1;
    }

    let mut new_stones = BTreeMap::new();
    let mut blinks = BTreeMap::new();
    for i in 0..75 {
        for (&stone, &count) in stones.iter() {
            let after_blink = blinks.entry(stone).or_insert_with(|| blink(stone));
            let new_count = new_stones.entry(after_blink.0).or_insert(0);
            *new_count += count;
            if let Some(after_blink) = after_blink.1 {
                let new_count = new_stones.entry(after_blink).or_insert(0);
                *new_count += count;
            }
        }

        stones.clear();
        stones.append(&mut new_stones);
        if i == 24 || i == 74 {
            println!(
                "[part {}] # of stones = {}, unique numbers = {}, unique numbers seen = {}",
                if i == 24 { "one" } else { "two" },
                stones.values().fold(0, |acc, count| acc + count),
                stones.len(),
                blinks.len()
            );
        }
    }
}
