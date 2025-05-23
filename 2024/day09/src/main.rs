fn checksum(blocks: &Vec<i64>, s: &str) {
    let mut sum = 0;
    for (i, &n) in blocks.iter().enumerate() {
        if n < 0 {
            continue;
        }
        sum += (i as u64) * (n as u64);
    }
    println!("[part {}] checksum = {}", s, sum);
}

fn main() {
    let content = std::fs::read_to_string("day09/input.txt").expect("Should have been able to read the file");
    let content = content.split("\n").next().expect("Input incomplete");

    let mut blocks = Vec::new();
    for (i, n) in content.chars().map(|c| c.to_digit(10).expect("conversion to u32/usize failed") as usize).enumerate() {
        let i = if i % 2 == 0 { (i / 2) as i64 } else { -1 };
        blocks.append(&mut vec![i; n]);
    }
    let backup = blocks.clone();
    
    let mut i = 0;
    while i < blocks.len() {
        while i + 1 < blocks.len() && blocks[i] < 0 {
            blocks[i] = blocks.pop().expect("loop not terminated in time");
        }
        i += 1;
    }
    checksum(&blocks, "one");

    blocks = backup;
    'outer: for file_id in (0..*blocks.last().unwrap()+1).rev() {
        let mut span = [usize::MAX, 0];
        for (i, &n) in blocks.iter().enumerate() {
            if n == file_id {
                span[0] = span[0].min(i);
                span[1] = span[1].max(i);
            }
        }
        let len = span[1] - span[0] + 1;

        let mut i = 0;
        while i < span[0] {
            if blocks[i] < 0 {
                let begin = i;
                while i + 1 < blocks.len() && blocks[i+1] < 0 {
                    i += 1;
                }
                if i - begin + 1 >= len {
                    for j in 0..len {
                        blocks[begin+j] = file_id;
                        blocks[span[0]+j] = -1;
                    }
                    continue 'outer;    
                }
            }
            i += 1;
        }
    }
    checksum(&blocks, "two");
}
