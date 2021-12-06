use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let n = 256;
    let mut counts: BTreeMap<u8, usize> = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .fold(BTreeMap::new(), |mut bt, n| {
            bt.entry(n.parse::<u8>().unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            bt
        });
    for _ in 1..=n {
        let mut new: BTreeMap<u8, usize> = BTreeMap::new();
        for k in counts.clone().keys().rev() {
            if k == &0 {
                new.insert(8, counts[k]);
                new.entry(6)
                    .and_modify(|e| *e += counts[k])
                    .or_insert_with(|| counts[k]);
            } else {
                new.insert(*k - 1, counts[k]);
            }
        }
        counts = new;
    }
    println!("sum {}", counts.values().sum::<usize>());
}
