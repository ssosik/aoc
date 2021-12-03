use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::error;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
const RADIX: u32 = 10;

fn main() -> Result<()> {
    
    let input = BufReader::new(std::io::stdin());
    let lines = input.lines();

    let mut bit_counts: BTreeMap<usize, BTreeMap<u8, usize>> = BTreeMap::new();
    for line in lines {
        for (idx, bit) in line.unwrap().chars().enumerate() {
            let bit = bit.to_digit(RADIX).expect("to digit failed") as u8;
            bit_counts.entry(idx)
                .and_modify(|m| {
                    m.entry(bit)
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1);
                })
                .or_insert(BTreeMap::from([(bit, 1)]));
            print!("{}:{} ", idx, bit);
        }
        println!("");
    };

    println!("{:?}", bit_counts);
    Ok(())
}
