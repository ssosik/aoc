use array2d::Array2D;
use std::collections::BTreeSet;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    println!("Hello, world!");
    let heightmap: Vec<Vec<u32>> = BufReader::new(std::io::stdin())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .inspect(|x| println!("{:?}", x))
                .filter_map(|v| v.to_digit(10u32))
                .collect::<Vec<_>>()
        })
        .collect();
    println!("HeightMap {:?}", heightmap);
}
