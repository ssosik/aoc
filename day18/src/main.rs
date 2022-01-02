use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Pair{
    x: PairItem,
    y: PairItem,
}

enum PairItem {
    Num(u8),
    Pair(Box<Pair>),
}

fn main() {
    println!("Hello, world!");
    let lines: Vec<_> = BufReader::new(std::io::stdin()).lines().collect();
    for line in lines {
        for c in line.unwrap().chars() {
            println!("char {}", c);
        }
    }
}
