use std::io::{BufRead, BufReader};
use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Submarine {
    position: usize,
    depth: usize
}

impl Submarine {
    fn new() -> Submarine {
        Submarine{position: 0, depth: 0}
    }
    fn forward(&mut self, n: usize) -> Result<()> {
        self.position += n;
        Ok(())
    }
    fn down(&mut self, n: usize) -> Result<()> {
        self.depth += n;
        Ok(())
    }
    fn up(&mut self, n: usize) -> Result<()> {
        self.depth -= n;
        Ok(())
    }
    fn position(&self) -> Result<usize> {
        Ok(self.position * self.depth)
    }
}

fn main() -> Result<()> {
    let input = BufReader::new(std::io::stdin());
    let lines = input.lines();

    let sub = &mut Submarine::new();
    for line in lines {
        match line
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            [direction, amount] => match (direction, amount.parse::<usize>().ok().unwrap()) {
                ("forward", n) => sub.forward(n),
                ("down", n) => sub.down(n),
                ("up", n) => sub.up(n),
                _ => unreachable!(),
            }
            _ => panic!("line not valid"),
        }?;

    }
    println!("{:?} {}", sub, sub.position().unwrap());

    Ok(())
}
