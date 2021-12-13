use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader};

fn main() {
    let mut nodes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (a, b) in BufReader::new(std::io::stdin()).lines().map(|line| {
        // Split each line on '|' character, passing the two pieces into the match
        match line.unwrap().split('-').collect::<Vec<&str>>()[..] {
            [a, b] => (a.to_string(), b.to_string()),
            _ => unreachable!(),
        }
    }) {
        println!("a {} b {}", a, b);
        nodes.entry(a.clone())
            .and_modify(|bset| {
                bset.insert(b.clone());
                ()
            })
            .or_insert(BTreeSet::from([b.clone()]));
        nodes.entry(b.clone())
            .and_modify(|bset| {
                bset.insert(a.clone());
                ()
            })
            .or_insert(BTreeSet::from([a]));
    }
    println!("{:?}", nodes);
}

