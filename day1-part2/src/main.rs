use std::io::{BufRead, BufReader};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
struct Triple (Option<usize>, Option<usize>, Option<usize>);

impl Triple {
    fn new() -> Triple {
        Triple(None, None, None)
    }
    fn add(&mut self, n: usize) -> Result<()> {
        self.0 = self.1;
        self.1 = self.2;
        self.2 = Some(n);
        Ok(())
    }
    fn sum(&self) -> Result<usize> {
        match self {
            Triple(Some(a), Some(b), Some(c)) => Ok(a + b + c),
            _ => Err(anyhow!("I have a None: {:?}", self)),
        }
    }
}

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();

    let mut last: Option<usize> = None;
    let mut cnt = 0;
    let mut triple = &mut Triple::new();
    for line in lines {
        // Get the next value
        let a: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();
        let a = a[0];

        triple.add(a);
        let sum = triple.sum();
        match (last, triple.sum()) {
            (Some(prev), Ok(sum)) => {
                if sum > prev {
                    cnt += 1;
                }
                last = Some(sum);
            },
            (None, Ok(sum)) => {
                last = Some(sum);
            },
            _ => continue,
        }

        //println!("{}: {:?}", a, triple.sum());
    }
    println!("{}", cnt);
}
