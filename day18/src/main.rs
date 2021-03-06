use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Depth(u8);
struct Pair{
    x: PairItem,
    y: PairItem,
    depth: Depth,
}

enum PairItem {
    Num(u8),
    Pair(Box<Pair>),
}

#[derive(Debug)]
struct ParseError;

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let head = chars.next().unwrap();
        let rest: String = chars.collect();
        if head == '[' {
            let x = Pair::from_str(&rest).unwrap();
        }
        println!("C {}", head);
        println!("rest {:?}", rest);

        Ok(Pair{ x: PairItem::Num(3), y: PairItem::Num(5) })
    }
}

fn main() {
    println!("Hello, world!");
    let lines: Vec<_> = BufReader::new(std::io::stdin()).lines().collect();
    for line in lines {
        Pair::from_str(&line.unwrap());
        //for c in line.unwrap().chars() {
        //    println!("char {}", c);
        //    match c {
        //        "[" => _,
        //        "]" => _,
        //        "," => _,
        //        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => _,
        //    }
        //}
    }
}
