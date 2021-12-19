use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Instructions {
    depth: u8,
    instructions: BTreeMap<(char, char), char>,
    counts: BTreeMap<char, usize>,
}

#[derive(Clone, Copy)]
enum Instruction {
    Counted(char),
    Uncounted(char),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Counted(c) => write!(f, "Counted {}", c),
            Instruction::Uncounted(c) => write!(f, "Uncounted {}", c),
        };
        Ok(())
    }
}
impl Instructions {
    fn new(depth: u8) -> Instructions {
        Instructions {
            depth,
            instructions: BTreeMap::new(),
            counts: BTreeMap::new(),
        }
    }
    fn insert(&mut self, left: char, right: char, insert: char) {
        self.instructions.insert((left, right), insert);
    }
    fn get(&self, left: Instruction, right: Instruction) -> Option<&char> {
        let left = match left {
            Instruction::Counted(x) => x,
            Instruction::Uncounted(x) => x,
        };
        let right = match right {
            Instruction::Counted(x) => x,
            Instruction::Uncounted(x) => x,
        };
        self.instructions.get(&(left, right))
    }
    fn count(&mut self, c: Instruction) -> Instruction {
        match c {
            Instruction::Counted(_) => c,
            Instruction::Uncounted(x) => {
                self.counts.entry(x).and_modify(|x| *x += 1).or_insert(1);
                Instruction::Counted(x)
            }
        }
    }
    fn recurse(&mut self, left: char, right: char) {
        self._recurse(
            0,
            Instruction::Uncounted(left),
            Instruction::Uncounted(right),
        )
    }
    fn _recurse(&mut self, depth: u8, left: Instruction, right: Instruction) {
        println!("Here {} left {} right {}", depth, left, right);
        if depth < self.depth {
            let left = self.count(left);
            let right = self.count(right);
            if let Some(val) = self.get(left, right) {
                self._recurse(depth + 1, left, Instruction::Uncounted(val.clone()));
            }
            if let Some(val) = self.get(left, right) {
                self._recurse(depth + 1, Instruction::Uncounted(val.clone()), right);
            }
        }
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
    let mut instructions = Instructions::new(10);
    for line in lines.iter().skip(2) {
        // Split each line on ',' character, passing the two pieces into the match
        match line.as_ref().unwrap().split(" -> ").collect::<Vec<&str>>()[..] {
            [a, b] => {
                match a.chars().into_iter().collect::<Vec<char>>()[..] {
                    [x, y] => {
                        instructions.insert(x, y, char::from_str(b).unwrap());
                    }
                    _ => unreachable!("EEk {:?}", line),
                };
            }
            _ => unreachable!("EEk {:?}", line),
        }
    }

    println!("Instructions {:?}", instructions);
    //let ret = instructions.recurse(input[0], input[1]);
    //println!("RET {:?}", ret.into_iter().collect::<String>());

    for tuple in input.iter().tuple_windows::<(_, _)>().collect::<Vec<_>>() {
        println!("Chunk {}{}", *tuple.0, *tuple.1);
        instructions.recurse(*tuple.0, *tuple.1);
    }

    let mut values: Vec<usize> = instructions.counts.into_values().collect();
    values.sort_unstable();
    println!(
        "Values: {:?} min {} max {}",
        values,
        values[0],
        values[values.len() - 1]
    );

    println!("Score: {}", values[values.len() - 1] - values[0]);
}
