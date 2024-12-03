fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut safe = 0;
    let mut safe_without_dampener = 0;
    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }
        let full_report: Vec<_> = line.split(" ").map(|s| s.parse::<i64>().expect("parsing to i64 failed for report")).collect();
        for index in (0..full_report.len()+1).rev() {
            let mut report = full_report.clone();
            if index < full_report.len() {
                report.remove(index);
            }
            let delta: Vec<_> = report.iter().skip(1).zip(report.iter()).map(|(&v1, &v0)| v1 - v0).collect();
            let sign = delta[0].signum();
            if sign == 0 || delta.iter().skip(1).any(|&v| v.signum() != sign) {
                continue;
            }
            if delta.iter().map(|&v| v.abs()).all(|v| 1 <= v && v <= 3) {
                safe += 1;
                if index == full_report.len() {
                    safe_without_dampener += 1;
                }
                break;
            }
        }
    }
    println!("safe reports: {} ({} without dampener)", safe, safe_without_dampener);
}
