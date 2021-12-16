use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Paths(BTreeSet<Vec<String>>);
impl Paths {
    fn new() -> Paths {
        Paths(BTreeSet::new())
    }

    fn walk(
        &mut self,
        curr: String,
        revisit_allowed: &str,
        nodes: BTreeMap<String, BTreeSet<String>>,
        mut seen: Vec<String>,
    ) -> usize {
        let mut can_revisit = curr != *"start" && curr != *"end";
        let is_revisit = curr.chars().all(char::is_lowercase) && seen.contains(&curr);
        //println!("is revisit {} curr {} revisit_allowed {}", is_revisit, curr, revisit_allowed);
        if is_revisit {
            if curr == revisit_allowed {
                let times_seen = seen
                    .iter()
                    .filter(|x| *x == revisit_allowed)
                    .count();
                //println!("Times Seen {} {}", revisit_allowed, times_seen);
                can_revisit = times_seen < 2;
            } else {
                can_revisit = false;
            }
        }

        if curr == "end" {
            seen.push(curr);
            //println!("Found end {:?}", seen);
            self.0.insert(seen);
            1
        } else if is_revisit && !can_revisit {
            //println!("XXXXXX");
            0
        } else {
            seen.push(curr.clone());
            //println!("else curr {:?}", seen);
            nodes.get(&curr).unwrap().iter().fold(0, |acc, next| {
                acc + self.walk(next.clone(), revisit_allowed, nodes.clone(), seen.clone())
            })
        }
    }
}

fn main() {
    let mut nodes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (a, b) in BufReader::new(std::io::stdin()).lines().map(|line| {
        // Split each line on '-' character, passing the two pieces into the match
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
    let mut paths = Paths::new();
    for revisit in nodes
        .keys()
        .filter(|x| {
            x.chars().all(char::is_lowercase)
                && x != &&"start".to_string()
                && x != &&"end".to_string()
        })
        .collect::<Vec<_>>()
    {
        println!("Testing with revisit of {}", revisit);
        paths.walk("start".to_string(), revisit, nodes.clone(), Vec::new());
    }
    println!("Result {:?}", paths.0.len());
}
