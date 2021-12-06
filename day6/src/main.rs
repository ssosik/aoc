use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    //let n = 18;
    let n = 80;
    let mut input = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    println!("{:>5}: input {:?}", 0, input);
    for i in 1..=n {
        let mut tmp: Vec<usize> = Vec::new();
        let mut babies: Vec<usize> = Vec::new();
        for t in input.clone() {
            tmp.push(match t {
                0 => {
                    babies.push(8);
                    6
                }
                _ => t - 1,
            });
        }
        input = tmp;
        input.extend(babies);
        //println!("{:>5}: input {:?}", i, input);
    }
    println!("Len: {}", input.len());
}
