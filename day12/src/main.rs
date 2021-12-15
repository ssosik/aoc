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
        //println!("a {} b {}", a, b);
        nodes
            .entry(a.clone())
            .and_modify(|bset| {
                bset.insert(b.clone());
            })
            .or_insert_with(|| BTreeSet::from([b.clone()]));
        nodes
            .entry(b.clone())
            .and_modify(|bset| {
                bset.insert(a.clone());
            })
            .or_insert_with(|| BTreeSet::from([a.clone()]));
    }
    println!("{:?}", nodes.clone());
    let result = walk("start".to_string(), nodes.clone(), Vec::new());
    println!("Result {}", result);
}

fn walk(curr: String, nodes: BTreeMap<String, BTreeSet<String>>, mut seen: Vec<String>) -> usize {
    //println!("Curr {} Seen {:?}", curr, seen);
    if curr == "end" {
        seen.push(curr);
        println!("Found end {:?}", seen);
        1
    } else if curr.chars().all(char::is_lowercase) && seen.contains(&curr) {
        //println!("Found double visit for {} in {:?}", curr, seen);
        0
    } else {
        seen.push(curr.clone());
        nodes.get(&curr).unwrap().iter().fold(0, |acc, next| {
            acc + walk(next.clone(), nodes.clone(), seen.clone())
        })
    }
}
