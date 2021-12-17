#![feature(iter_map_while)]
use itertools::Itertools;
use std::collections::BTreeMap;

use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let lines: Vec<_> = BufReader::new(std::io::stdin()).lines().collect();

    let mut template = lines.get(0)
        .unwrap()
        .as_ref()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    println!("Template {:?}", template);

    // Load the initial coordinate inputs
    let mut instructions: BTreeMap<(char, char), char> = BTreeMap::new();
    for line in lines.iter().skip(2) {
        // Split each line on ',' character, passing the two pieces into the match
        match line.as_ref().unwrap().split(" -> ").collect::<Vec<&str>>()[..] {
            [a, b] => {
                match a.chars().into_iter().collect::<Vec<char>>()[..] {
                    [x, y] => {
                        instructions.insert((x, y), char::from_str(b).unwrap());
                    }
                    _ => unreachable!("EEk {:?}", line),
                };
            }
            _ => unreachable!("EEk {:?}", line),
        }
    }
    println!("Instructions {:?}", instructions);

    for _ in 0..10 {
        let mut tmp = Vec::new();
        let mut last: char = 'X';
        for tuple in template
            .iter()
            .tuple_windows::<(_, _)>()
            .collect::<Vec<_>>()
        {
            let tuple = (*tuple.0, *tuple.1);
            println!("Tuple {:?} {:?}", tuple, instructions.get(&tuple));
            tmp.push(tuple.0);
            if let Some(c) = instructions.get(&tuple) {
                tmp.push(*c);
            }
            last = tuple.1;
        }
        tmp.push(last);
        template = tmp;
        println!("Template {:?}", template);
    }

    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    for c in template {
        counts.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut values: Vec<usize> = counts.into_values().collect();
    values.sort_unstable();
    println!("Values: {:?} min {} max {}", values, values[0], values[values.len() - 1]);


    println!("Score: {}", values[values.len() - 1] - values[0]);
}
