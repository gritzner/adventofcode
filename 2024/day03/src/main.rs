use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("day03/input.txt").expect("Should have been able to read the file");

    let re = Regex::new(r"do\(\)").expect("regular expression compilation failed");
    let mut enabled = vec![0];
    for m in re.find_iter(&contents) {
        enabled.push(m.start() as isize);
    }
    
    let re = Regex::new(r"don\'t\(\)").expect("regular expression compilation failed");
    let mut disabled = vec![-1];
    for m in re.find_iter(&contents) {
        disabled.push(m.start() as isize);
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("regular expression compilation failed");
    let mut sum = 0;
    let mut masked_sum = 0;
    for m in re.captures_iter(&contents) {
        let v0 = m[1].parse::<i64>().expect("cannot parse first matched number");
        let v1 = m[2].parse::<i64>().expect("cannot parse second matched number");
        let prod = v0 * v1;
        sum += prod;
        let start = m.get(0).unwrap().start() as isize;
        let i = enabled[(enabled.partition_point(|&x| x < start) as usize) - 1];
        let j = disabled[(disabled.partition_point(|&x| x < start) as usize) - 1];
        if j < i {
            masked_sum += prod;
        }
    }
    println!("sum = {}, masked sum = {}", sum, masked_sum);
}
