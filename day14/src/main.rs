use itertools::Itertools;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let lines: Vec<_> = BufReader::new(std::io::stdin()).lines().collect();

    // Load the initial input
    let input = lines
        .get(0)
        .unwrap()
        .as_ref()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    println!("input {:?}", input);

    let mut instructions: BTreeMap<(char, char), ((char, char), (char, char))> = BTreeMap::new();
    // Load the initial coordinate inputs
    for line in lines.iter().skip(2) {
        // Split each line on ',' character, passing the two pieces into the match
        match line.as_ref().unwrap().split(" -> ").collect::<Vec<&str>>()[..] {
            [a, b] => {
                match a.chars().into_iter().collect::<Vec<char>>()[..] {
                    [x, y] => {
                        let m = char::from_str(b).unwrap();
                        instructions.insert((x, y), ((x, m), (m, y)));
                    }
                    _ => unreachable!("EEk {:?}", line),
                };
            }
            _ => unreachable!("EEk {:?}", line),
        }
    }
    println!("Instructions {:?}", instructions);

    let iterate = |t: (char, char)| {
        let i = instructions.get(&t).unwrap();
        iterate(i.0);
        iterate(i.1);
    };
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    for tuple in input.iter().tuple_windows::<(_, _)>().collect::<Vec<_>>() {
        println!("Chunk {}{}", *tuple.0, *tuple.1);
        //counts = instructions.recurse(*tuple.0, *tuple.1);

        break;
    }

    //let mut values: Vec<usize> = counts.into_values().collect();
    //values.sort_unstable();
    //println!(
    //    "Values: {:?} min {} max {}",
    //    values,
    //    values[0],
    //    values[values.len() - 1]
    //);

    //println!("Score: {}", values[values.len() - 1] - values[0]);
}
