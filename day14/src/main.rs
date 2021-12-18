#![feature(iter_map_while)]
use itertools::Itertools;
use std::collections::BTreeMap;

use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Instructions(u8, BTreeMap<(char, char), char>);

impl Instructions {
    fn recurse(&self, left: char, right: char) -> Vec<char> {
        let mut ret = vec![left];
        ret.extend(self._recurse(0, left, right));
        ret
    }
    fn _recurse(&self, depth: u8, left: char, right: char) -> Vec<char> {
        //println!("Here {} left {} right {}", depth, left, right);
        let mut ret = Vec::new();
        if depth >= self.0 {
            return vec![right];
        }
        match self.1.get(&(left, right)) {
            Some(val) => {
                ret.extend(self._recurse(depth + 1, left, *val));
                ret.extend(self._recurse(depth + 1, *val, right));
            }
            None => {
                println!("NONE {} left {} right {}", depth, left, right);
                return vec![left, right];
            }
        };
        ret
    }
}

fn main() {
    let lines: Vec<_> = BufReader::new(std::io::stdin()).lines().collect();

    let input = lines
        .get(0)
        .unwrap()
        .as_ref()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    println!("input {:?}", input);

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

    let instructions = Instructions(10, instructions);
    println!("Instructions {:?}", instructions);
    //let ret = instructions.recurse(input[0], input[1]);
    //println!("RET {:?}", ret.into_iter().collect::<String>());

    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    for tuple in input.iter().tuple_windows::<(_, _)>().collect::<Vec<_>>() {
        println!("Chunk {}{}", *tuple.0, *tuple.1);
        for c in instructions.recurse(*tuple.0, *tuple.1) {
            counts.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let mut values: Vec<usize> = counts.into_values().collect();
    values.sort_unstable();
    println!(
        "Values: {:?} min {} max {}",
        values,
        values[0],
        values[values.len() - 1]
    );

    println!("Score: {}", values[values.len() - 1] - values[0]);
}
