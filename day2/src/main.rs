use std::io::{BufRead, BufReader};
use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Submarine {
    position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine{position: 0, depth: 0, aim: 0}
    }
    fn forward(&mut self, x: usize) -> Result<()> {
        self.position += x;
        self.depth += self.aim * x;
        Ok(())
    }
    fn down(&mut self, x: usize) -> Result<()> {
        self.aim += x;
        Ok(())
    }
    fn up(&mut self, x: usize) -> Result<()> {
        self.aim -= x;
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
                ("forward", x) => sub.forward(x),
                ("down", x) => sub.down(x),
                ("up", x) => sub.up(x),
                _ => unreachable!(),
            }
            _ => panic!("line not valid"),
        }?;

    }
    println!("{:?} {}", sub, sub.position().unwrap());

    Ok(())
}
