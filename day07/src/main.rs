fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let contents: Vec<_> = contents
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(": ").collect::<Vec<_>>())
        .map(|eq| (eq[0].parse::<u64>().expect("conversion to u64 failed"), eq[1]))
        .map(|eq| (eq.0, eq.1.split(" ").map(|s| s.parse::<u64>().expect("conversion to u64 failed")).collect::<Vec<_>>()))
        .collect();

    let mut total = 0;
    for eq in &contents {
        for i in 0..2_usize.pow((eq.1.len()-1) as u32) {
            let mut sum = eq.1[0];
            for (j, &v) in eq.1.iter().skip(1).enumerate() {
                if i & 2_usize.pow(j as u32) != 0 {
                    sum *= v;
                } else {
                    sum += v;
                }
            }
            if sum == eq.0 {
                total += eq.0;
                break;
            }
        }
    }
    println!("[part one] total = {}", total);
    
    total = 0;
    for eq in &contents {
        for i in 0..3_usize.pow((eq.1.len()-1) as u32) {
            let mut op = i;
            let mut sum = eq.1[0];
            for &v in eq.1.iter().skip(1) {
                match op % 3 {
                    0 => { sum += v },
                    1 => { sum *= v },
                    _ => { sum = format!("{}{}", sum, v).parse::<u64>().expect("conversion to u64 failed") }
                }
                op /= 3;
            }
            if sum == eq.0 {
                total += eq.0;
                break;
            }
        }
    }
    println!("[part two] total = {}", total);
}
