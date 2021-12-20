use array2d::Array2D;
use std::cmp;
use std::fmt;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<Vec<u32>> = BufReader::new(std::io::stdin())
        .lines()
        //.map(|l| l.unwrap().chars().map(|c| c.to_digit(10_u32).unwrap()))
        .map(|l| l.unwrap().chars().collect::<_>())
        .map(|l: Vec<char>| l.iter().map(|c| c.to_digit(10_u32).unwrap()).collect::<Vec<u32>>())
        .inspect(|x| println!("X {:?}", x))
        .collect();
    let lines = Array2D::from_rows(&lines);

    println!("lines: {:?}", lines);
}
