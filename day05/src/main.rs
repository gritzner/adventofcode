use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let contents: Vec<_> = contents.split("\n").filter(|line| !line.is_empty()).collect();
    let rules: Vec<_> = contents.iter()
        .filter(|line| line.contains('|'))
        .map(|rule| rule.split('|')
            .map(|n| n.parse::<u64>().expect("conversion from str to u64 failed"))
            .collect::<Vec<_>>()
        )
        .collect();
    let updates: Vec<_> = contents.iter()
        .filter(|line| !line.contains('|'))
        .map(|rule| rule.split(',')
            .map(|n| n.parse::<u64>().expect("conversion from str to u64 failed"))
            .collect::<Vec<_>>()
        )
        .collect();

    let mut constraints = BTreeMap::new();
    for rule in &rules {
        let set = constraints.entry(rule[0]).or_insert_with(BTreeSet::new);
        set.insert(rule[1]);
    }

    let mut sum = 0;
    let mut incorrect = Vec::new();
    'outer: for update in &updates {
        let mut prev_pages = BTreeSet::new();
        for &page in update {
            if let Some(constraint) = constraints.get(&page) {
                if prev_pages.intersection(constraint).count() > 0 {
                    incorrect.push(update.clone());
                    continue 'outer;
                }
            }
            prev_pages.insert(page);
        }
        assert!(update.len() % 2 == 1);
        let n = (update.len() - 1) / 2;
        sum += update[n];
    }
    println!("sum = {}", sum);

    sum = 0;
    for update in incorrect {
        let set: BTreeSet<_> = update.iter().map(|&n| n).collect();
        let mut v: Vec<_> = update.iter()
            .map(|&n| (n, if let Some(constraint) = constraints.get(&n) { set.intersection(constraint).count() as i64 } else { 0 }))
            .collect();
        v.sort_by_key(|n| -n.1);
        assert!(update.len() % 2 == 1);
        let n = (update.len() - 1) / 2;
        sum += v[n].0;
    }
    println!("sum = {}", sum);
}
