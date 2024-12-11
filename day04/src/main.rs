fn main() {
    let contents = std::fs::read_to_string("day04/input.txt").expect("Should have been able to read the file");
    let matrix: Vec<_> = contents.split("\n").filter(|line| !line.is_empty()).collect();
    let mut count = 0;
    let mut x_count = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            // part 2
            let mut s0 = Vec::new();
            let mut s1 = Vec::new();
            for k in 0..3 {
                if let Some(line) = matrix.get(i + k) {
                    if let Some(c) = line.chars().nth(j + k) {
                        s0.push(c);
                    }
                    if let Some(c) = line.chars().nth(j + 2 - k) {
                        s1.push(c);
                    }
                }
            }
            let s0: String = s0.iter().collect();
            let s1: String = s1.iter().collect();
            if (s0 == "MAS" || s0 == "SAM") && (s1 == "MAS" || s1 == "SAM") {
                x_count += 1;
            }

            // part 1
            let (i, j) = (i as isize, j as isize);
            for d in [(0, 1), (1, 0), (1, 1), (1, -1)] {
                let mut s = vec![c];
                for k in 1..4 {
                    if let Some(line) = matrix.get((i + k * d.0) as usize) {
                        if let Some(c) = line.chars().nth((j + k * d.1) as usize) {
                            s.push(c);
                        }
                    }
                }
                let s: String = s.iter().collect();
                if s == "XMAS" || s == "SAMX" {
                    count += 1;
                }
            }
        }
    }
    println!("count = {}, x_count = {}", count, x_count);
}
