use std::io::{BufRead, BufReader};
use phf::phf_map;

static HEX: phf::Map<char, [u8; 4]> = phf_map! {
    '0' => [0,0,0,0],
    '1' => [0,0,0,1],
    '2' => [0,0,1,0],
    '3' => [0,0,1,1],
    '4' => [0,1,0,0],
    '5' => [0,1,0,1],
    '6' => [0,1,1,0],
    '7' => [0,1,1,1],
    '8' => [1,0,0,0],
    '9' => [1,0,0,1],
    'A' => [1,0,1,0],
    'B' => [1,0,1,1],
    'C' => [1,1,0,0],
    'D' => [1,1,0,1],
    'E' => [1,1,1,0],
    'F' => [1,1,1,1],
};

struct packet{}

fn main() {
    let binary = BufReader::new(std::io::stdin())
        .lines()
        // Take only the one line
        .take(1)
        .inspect(|line| println!("Hex Input list {:?}", line))
        // Convert the line into a Vec of chars
        .map(|l| l.unwrap().chars().collect::<_>())
        // Convert each char into its binary representation of 4 bits, flattening
        // into one continuous list of bits
        .flat_map(|l: Vec<char>| {
            l.iter()
                // Lookup the char in the Hex map, returning the slice of bits
                .flat_map(|c| HEX.get(c).unwrap())
                .collect::<Vec<&u8>>()
        })
        .collect::<Vec<_>>();

    println!("Binary {:?}", binary);
}
